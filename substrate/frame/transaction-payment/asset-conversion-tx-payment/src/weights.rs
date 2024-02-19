// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_asset_conversion_tx_payment`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-04, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Georges-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/debug/substrate-node
// benchmark
// pallet
// --steps=2
// --repeat=2
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_asset_conversion_tx_payment
// --chain=dev
// --output=./substrate/frame/transaction-payment/asset-conversion-tx-payment/src/weights.rs
// --template=./substrate/.maintain/frame-weight-template.hbs
// --header=./substrate/HEADER-APACHE2

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_asset_conversion_tx_payment`.
pub trait WeightInfo {
	fn charge_asset_tx_payment_zero() -> Weight;
	fn charge_asset_tx_payment_native() -> Weight;
	fn charge_asset_tx_payment_asset() -> Weight;
}

/// Weights for `pallet_asset_conversion_tx_payment` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn charge_asset_tx_payment_zero() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn charge_asset_tx_payment_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `1700`
		// Minimum execution time: 322_000_000 picoseconds.
		Weight::from_parts(325_000_000, 1700)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn charge_asset_tx_payment_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905`
		//  Estimated: `6208`
		// Minimum execution time: 1_253_000_000 picoseconds.
		Weight::from_parts(1_263_000_000, 6208)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn charge_asset_tx_payment_zero() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn charge_asset_tx_payment_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `1700`
		// Minimum execution time: 322_000_000 picoseconds.
		Weight::from_parts(325_000_000, 1700)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn charge_asset_tx_payment_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905`
		//  Estimated: `6208`
		// Minimum execution time: 1_253_000_000 picoseconds.
		Weight::from_parts(1_263_000_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}