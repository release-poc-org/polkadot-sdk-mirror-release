// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Staging Primitives.

// Put any primitives used by staging APIs functions here

use bitvec::vec::BitVec;

/// Bit indices in the `HostConfiguration.node_features` that correspond to different node features.
/// The maximum bit that can be currently set/unset via the `set_node_feature` extrinsic is 255.
pub type NodeFeatures = BitVec<u8, bitvec::order::Lsb0>;

/// Module containing feature-specific bit indices into the `NodeFeatures` bitvec.
pub mod node_features {
	/// Index of the availability chunk shuffling feature bit.
	pub const AVAILABILITY_CHUNK_SHUFFLING: u8 = 0;
}
