// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! A collator for Aura that looks ahead of the most recently included parachain block
//! when determining what to build upon.
//!
//! The block building mechanism consists of two parts:
//! 	1. A block-builder task that builds parachain blocks at each of our slot.
//! 	2. A collator task that transforms the blocks into a collation and submits them to the relay
//!     chain.
//!
//! This collator also builds additional blocks when the maximum backlog is not saturated.
//! The size of the backlog is determined by invoking a runtime API. If that runtime API
//! is not supported, this assumes a maximum backlog size of 1.
//!
//! This takes more advantage of asynchronous backing, though not complete advantage.
//! When the backlog is not saturated, this approach lets the backlog temporarily 'catch up'
//! with periods of higher throughput. When the backlog is saturated, we typically
//! fall back to the limited cadence of a single parachain block per relay-chain block.
//!
//! Despite this, the fact that there is a backlog at all allows us to spend more time
//! building the block, as there is some buffer before it can get posted to the relay-chain.
//! The main limitation is block propagation time - i.e. the new blocks created by an author
//! must be propagated to the next author before their turn.

use codec::{Codec, Encode};

use cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface;
use cumulus_client_consensus_common::{self as consensus_common, ParachainBlockImportMarker};
use cumulus_client_consensus_proposer::ProposerInterface;
use cumulus_primitives_aura::AuraUnincludedSegmentApi;
use cumulus_primitives_core::{CollectCollationInfo, PersistedValidationData};
use cumulus_relay_chain_interface::RelayChainInterface;

use polkadot_primitives::{BlockId, Id as ParaId, OccupiedCoreAssumption};

use sc_client_api::{backend::AuxStore, BlockBackend, BlockOf, UsageProvider};
use sc_consensus::BlockImport;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_consensus_aura::{AuraApi, Slot, SlotDuration};
use sp_core::crypto::Pair;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member};
use sp_timestamp::Timestamp;
use std::{sync::Arc, time::Duration};

use super::CollatorMessage;
use crate::{
	collator::{self as collator_util},
	collators::{check_validation_code_or_log, cores_scheduled_for_para},
	LOG_TARGET,
};

/// Parameters for [`run_block_builder`].
pub struct BuilderTaskParams<Block: BlockT, BI, CIDP, Client, Backend, RClient, CHP, Proposer, CS> {
	/// Inherent data providers. Only non-consensus inherent data should be provided, i.e.
	/// the timestamp, slot, and paras inherents should be omitted, as they are set by this
	/// collator.
	pub create_inherent_data_providers: CIDP,
	/// Used to actually import blocks.
	pub block_import: BI,
	/// The underlying para client.
	pub para_client: Arc<Client>,
	/// The para client's backend, used to access the database.
	pub para_backend: Arc<Backend>,
	/// A handle to the relay-chain client.
	pub relay_client: RClient,
	/// A validation code hash provider, used to get the current validation code hash.
	pub code_hash_provider: CHP,
	/// The underlying keystore, which should contain Aura consensus keys.
	pub keystore: KeystorePtr,
	/// The para's ID.
	pub para_id: ParaId,
	/// The underlying block proposer this should call into.
	pub proposer: Proposer,
	/// The generic collator service used to plug into this consensus engine.
	pub collator_service: CS,
	/// The amount of time to spend authoring each block.
	pub authoring_duration: Duration,
	/// Channel to send built blocks to the collation task.
	pub collator_sender: sc_utils::mpsc::TracingUnboundedSender<CollatorMessage<Block>>,
	/// Slot duration of the relay chain
	pub relay_chain_slot_duration: Duration,
}

#[derive(Debug)]
struct SlotAndTime {
	timestamp: Timestamp,
	slot: Slot,
}

#[derive(Debug)]
struct SlotTimer {
	slot_duration: SlotDuration,
	drift: Duration,
}

/// Returns current duration since unix epoch.
fn duration_now() -> Duration {
	use std::time::SystemTime;
	let now = SystemTime::now();
	now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|e| {
		panic!("Current time {:?} is before unix epoch. Something is wrong: {:?}", now, e)
	})
}

/// TODO For testing of slot drift, check if can be moved elsewhere.
/// Returns the duration until the next slot from now.
fn time_until_next_slot(slot_duration: Duration, drift: Duration) -> Duration {
	let now = duration_now().as_millis() - drift.as_millis();

	let next_slot = (now + slot_duration.as_millis()) / slot_duration.as_millis();
	let remaining_millis = next_slot * slot_duration.as_millis() - now;
	Duration::from_millis(remaining_millis as u64)
}

impl SlotTimer {
	pub fn new_with_drift(slot_duration: SlotDuration, drift: Duration) -> Self {
		Self { slot_duration, drift }
	}

	/// Returns a future that resolves when the next slot arrives.
	pub async fn wait_until_next_slot(&self) -> SlotAndTime {
		let time_until_next_slot =
			time_until_next_slot(self.slot_duration.as_duration(), self.drift);
		tokio::time::sleep(time_until_next_slot).await;
		let timestamp = sp_timestamp::Timestamp::current();
		SlotAndTime { slot: Slot::from_timestamp(timestamp, self.slot_duration), timestamp }
	}
}

/// Run block-builder.
pub async fn run_block_builder<Block, P, BI, CIDP, Client, Backend, RClient, CHP, Proposer, CS>(
	params: BuilderTaskParams<Block, BI, CIDP, Client, Backend, RClient, CHP, Proposer, CS>,
) where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>
		+ UsageProvider<Block>
		+ BlockOf
		+ AuxStore
		+ HeaderBackend<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api:
		AuraApi<Block, P::Public> + CollectCollationInfo<Block> + AuraUnincludedSegmentApi<Block>,
	Backend: sc_client_api::Backend<Block> + 'static,
	RClient: RelayChainInterface + Clone + 'static,
	CIDP: CreateInherentDataProviders<Block, ()> + 'static,
	CIDP::InherentDataProviders: Send,
	BI: BlockImport<Block> + ParachainBlockImportMarker + Send + Sync + 'static,
	Proposer: ProposerInterface<Block> + Send + Sync + 'static,
	CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
	CHP: consensus_common::ValidationCodeHashProvider<Block::Hash> + Send + 'static,
	P: Pair,
	P::Public: AppPublic + Member + Codec,
	P::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let BuilderTaskParams {
		relay_client,
		create_inherent_data_providers,
		para_client,
		keystore,
		block_import,
		para_id,
		proposer,
		collator_service,
		collator_sender,
		code_hash_provider,
		authoring_duration,
		para_backend,
		relay_chain_slot_duration,
	} = params;

	let slot_duration = match crate::slot_duration(&*para_client) {
		Ok(s) => s,
		Err(e) => {
			tracing::error!(target: crate::LOG_TARGET, ?e, "Failed to fetch slot duration from runtime. Killing collator task.");
			return
		},
	};

	let slot_timer = SlotTimer::new_with_drift(slot_duration, Duration::from_secs(1));

	let mut collator = {
		let params = collator_util::Params {
			create_inherent_data_providers,
			block_import,
			relay_client: relay_client.clone(),
			keystore: keystore.clone(),
			para_id,
			proposer,
			collator_service,
		};

		collator_util::Collator::<Block, P, _, _, _, _, _>::new(params)
	};

	let Ok(expected_cores) = u64::try_from(
		relay_chain_slot_duration.as_millis() / slot_duration.as_duration().as_millis(),
	) else {
		tracing::error!(target: LOG_TARGET, ?relay_chain_slot_duration, ?slot_duration, "Unable to calculate expected parachain expected_cores.");
		return;
	};
	let expected_cores = expected_cores.max(1);

	loop {
		// We wait here until the next slot arrives.
		let para_slot = slot_timer.wait_until_next_slot().await;

		let Ok(relay_parent) = relay_client.best_block_hash().await else {
			tracing::warn!("Unable to fetch latest relay chain block hash, skipping slot.");
			continue;
		};

		let scheduled_cores = cores_scheduled_for_para(relay_parent, para_id, &relay_client).await;
		if scheduled_cores.is_empty() {
			tracing::debug!(target: LOG_TARGET, "Parachain not scheduled, skipping slot.");
			continue;
		}

		let core_index_in_scheduled: u64 = *para_slot.slot % expected_cores;
		let Some(core_index) = scheduled_cores.get(core_index_in_scheduled as usize) else {
			tracing::debug!(target: LOG_TARGET, core_index_in_scheduled, core_len = scheduled_cores.len(), "Para is scheduled, but not enough cores available.");
			continue;
		};

		let Ok(Some(relay_parent_header)) = relay_client.header(BlockId::Hash(relay_parent)).await
		else {
			tracing::warn!("Unable to fetch latest relay chain block header.");
			continue;
		};

		let max_pov_size = match relay_client
			.persisted_validation_data(relay_parent, para_id, OccupiedCoreAssumption::Included)
			.await
		{
			Ok(None) => continue,
			Ok(Some(pvd)) => pvd.max_pov_size,
			Err(err) => {
				tracing::error!(target: crate::LOG_TARGET, ?err, "Failed to gather information from relay-client");
				continue
			},
		};

		let (included_block, parent) = match crate::collators::find_parent(
			relay_parent,
			para_id,
			&*para_backend,
			&relay_client,
		)
		.await
		{
			Some(value) => value,
			None => continue,
		};

		let parent_header = parent.header;
		let parent_hash = parent.hash;

		// We mainly call this to inform users at genesis if there is a mismatch with the
		// on-chain data.
		collator.collator_service().check_block_status(parent_hash, &parent_header);

		let slot_claim = match crate::collators::can_build_upon::<_, _, P>(
			para_slot.slot,
			para_slot.timestamp,
			parent_hash,
			included_block,
			&*para_client,
			&keystore,
		)
		.await
		{
			Some(slot) => slot,
			None => continue,
		};

		tracing::debug!(
			target: crate::LOG_TARGET,
			?core_index,
			slot = ?para_slot.slot,
			unincluded_segment_len = parent.depth,
			relay_parent = %relay_parent,
			included = %included_block,
			parent = %parent_hash,
			"Building block."
		);

		let validation_data = PersistedValidationData {
			parent_head: parent_header.encode().into(),
			relay_parent_number: *relay_parent_header.number(),
			relay_parent_storage_root: *relay_parent_header.state_root(),
			max_pov_size,
		};

		let (parachain_inherent_data, other_inherent_data) = match collator
			.create_inherent_data(
				relay_parent,
				&validation_data,
				parent_hash,
				slot_claim.timestamp(),
			)
			.await
		{
			Err(err) => {
				tracing::error!(target: crate::LOG_TARGET, ?err);
				break
			},
			Ok(x) => x,
		};

		let validation_code_hash = match code_hash_provider.code_hash_at(parent_hash) {
			None => {
				tracing::error!(target: crate::LOG_TARGET, ?parent_hash, "Could not fetch validation code hash");
				break
			},
			Some(v) => v,
		};

		check_validation_code_or_log(&validation_code_hash, para_id, &relay_client, relay_parent)
			.await;

		let Ok(Some(candidate)) = collator
			.build_block_and_import(
				&parent_header,
				&slot_claim,
				None,
				(parachain_inherent_data, other_inherent_data),
				authoring_duration,
				// Set the block limit to 50% of the maximum PoV size.
				//
				// TODO: If we got benchmarking that includes the proof size,
				// we should be able to use the maximum pov size.
				(validation_data.max_pov_size / 2) as usize,
			)
			.await
		else {
			tracing::error!(target: crate::LOG_TARGET, "Unable to build block at slot.");
			continue;
		};

		let new_block_hash = candidate.block.header().hash();

		// Announce the newly built block to our peers.
		collator.collator_service().announce_block(new_block_hash, None);

		if let Err(err) = collator_sender.unbounded_send(CollatorMessage {
			relay_parent,
			parent_header,
			parachain_candidate: candidate,
			validation_code_hash,
			core_index: *core_index,
		}) {
			tracing::error!(target: crate::LOG_TARGET, ?err, "Unable to send block to collation task.");
		}
	}
}