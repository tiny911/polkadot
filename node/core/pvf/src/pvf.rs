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

use crate::artifacts::ArtifactId;
use polkadot_parachain::primitives::ValidationCodeHash;
use polkadot_primitives::vstaging::ExecutorParams;
use sp_core::blake2_256;
use std::{fmt, sync::Arc};

/// A struct that carries code of a parachain validation function and its hash.
///
/// Should be cheap to clone.
#[derive(Clone)]
pub struct Pvf {
	pub(crate) code: Arc<Vec<u8>>,
	pub(crate) code_hash: ValidationCodeHash,
}

impl fmt::Debug for Pvf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Pvf {{ code, code_hash: {:?} }}", self.code_hash)
	}
}

impl Pvf {
	/// Returns an instance of the PVF out of the given PVF code.
	pub fn from_code(code: Vec<u8>) -> Self {
		let code = Arc::new(code);
		let code_hash = blake2_256(&code).into();
		Self { code, code_hash }
	}

	/// Creates a new PVF which artifact id can be uniquely identified by the given number.
	#[cfg(test)]
	pub(crate) fn from_discriminator(num: u32) -> Self {
		let descriminator_buf = num.to_le_bytes().to_vec();
		Pvf::from_code(descriminator_buf)
	}
}

/// Coupling PVF code with executor params
#[derive(Debug, Clone)]
pub struct PvfWithExecutorParams {
	pvf: Pvf,
	executor_params: Arc<ExecutorParams>,
}

impl PvfWithExecutorParams {
	/// Creates a new PVF-ExecutorParams pair structure
	pub fn new(pvf: Pvf, executor_params: ExecutorParams) -> Self {
		Self { pvf, executor_params: Arc::new(executor_params) }
	}

	/// Returns artifact ID that corresponds to the PVF with given executor params
	pub(crate) fn as_artifact_id(&self) -> ArtifactId {
		ArtifactId::new(self.pvf.code_hash, self.executor_params.hash())
	}

	/// Returns validation code hash for the PVF
	pub(crate) fn code_hash(&self) -> ValidationCodeHash {
		self.pvf.code_hash
	}

	/// Returns PVF code
	pub(crate) fn code(&self) -> Arc<Vec<u8>> {
		self.pvf.code.clone()
	}

	/// Returns executor params
	pub(crate) fn executor_params(&self) -> ExecutorParams {
		(*self.executor_params).clone()
	}

	/// Creates a structure for tests
	#[cfg(test)]
	pub(crate) fn from_discriminator(num: u32) -> Self {
		Self {
			pvf: Pvf::from_discriminator(num),
			executor_params: Arc::new(ExecutorParams::default()),
		}
	}
}
