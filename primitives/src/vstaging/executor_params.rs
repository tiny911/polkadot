// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Abstract execution environment parameter set.
//!
//! Parameter set is encoded as an opaque vector which structure depends on the execution
//! environment itself (except for environment type/version which is always represented
//! by the first element of the vector). Decoding to a usable semantics structure is
//! done in `polkadot-node-core-pvf`.

use crate::{BlakeTwo256, HashT as _};
use parity_scale_codec::{Decode, Encode};
use polkadot_core_primitives::Hash;
use scale_info::TypeInfo;
use sp_std::{ops::Deref, vec, vec::Vec};

/// A single executor parameter
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub enum ExecutorParam {
	/// ## Parameters setting the executuion environment semantics:
	/// Max. memory size
	MaxMemorySize(u32),
	/// Wasm logical stack size limit (max. number of Wasm values on stack)
	StackLogicalMax(u32),
	/// Executor machine stack size limit, in bytes
	StackNativeMax(u32),
	/// Max. amount of memory the preparation worker is allowed to use during
	/// pre-checking, in bytes
	PrecheckingMaxMemory(u64),
}

/// Unit type wrapper around [`type@Hash`] that represents an execution parameter set hash.
///
/// This type is produced by [`ExecutorParams::hash`].
#[derive(Clone, Copy, Encode, Decode, Hash, Eq, PartialEq, PartialOrd, Ord, TypeInfo)]
pub struct ExecutorParamsHash(Hash);

impl ExecutorParamsHash {
	/// Create a new executor parameter hash from `H256` hash
	pub fn from_hash(hash: Hash) -> Self {
		Self(hash)
	}
}

impl sp_std::fmt::Display for ExecutorParamsHash {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		self.0.fmt(f)
	}
}

impl sp_std::fmt::Debug for ExecutorParamsHash {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl sp_std::fmt::LowerHex for ExecutorParamsHash {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		sp_std::fmt::LowerHex::fmt(&self.0, f)
	}
}

/// # Deterministically serialized execution environment semantics
/// Represents an arbitrary semantics of an arbitrary execution environment, so should be kept as
/// abstract as possible.
// ADR: For mandatory entries, mandatoriness should be enforced in code rather than separating them
// into individual fields of the structure. Thus, complex migrations shall be avoided when adding
// new entries and removing old ones. At the moment, there's no mandatory parameters defined. If
// they show up, they must be clearly documented as mandatory ones.
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub struct ExecutorParams(Vec<ExecutorParam>);

impl ExecutorParams {
	/// Creates a new, empty executor parameter set
	pub fn new() -> Self {
		ExecutorParams(vec![])
	}

	/// Returns hash of the set of execution environment parameters
	pub fn hash(&self) -> ExecutorParamsHash {
		ExecutorParamsHash(BlakeTwo256::hash(&self.encode()))
	}
}

impl Deref for ExecutorParams {
	type Target = Vec<ExecutorParam>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<&[ExecutorParam]> for ExecutorParams {
	fn from(arr: &[ExecutorParam]) -> Self {
		ExecutorParams(arr.to_vec())
	}
}

impl Default for ExecutorParams {
	fn default() -> Self {
		ExecutorParams(vec![])
	}
}
