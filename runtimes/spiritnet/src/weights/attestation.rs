// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for `attestation`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/runtime-weight-template.hbs
// --header=HEADER-GPL
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=spiritnet-dev
// --pallet=attestation
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/attestation.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `attestation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> attestation::WeightInfo for WeightInfo<T> {
	/// Storage: Ctype Ctypes (r:1 w:0)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn add() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `761`
		//  Estimated: `10810`
		// Minimum execution time: 37_180_000 picoseconds.
		Weight::from_parts(37_884_000, 0)
			.saturating_add(Weight::from_parts(0, 10810))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3660`
		// Minimum execution time: 19_496_000 picoseconds.
		Weight::from_parts(19_848_000, 0)
			.saturating_add(Weight::from_parts(0, 3660))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 33_437_000 picoseconds.
		Weight::from_parts(33_900_000, 0)
			.saturating_add(Weight::from_parts(0, 7257))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn reclaim_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 33_142_000 picoseconds.
		Weight::from_parts(33_734_000, 0)
			.saturating_add(Weight::from_parts(0, 7257))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1389`
		//  Estimated: `9864`
		// Minimum execution time: 47_960_000 picoseconds.
		Weight::from_parts(48_308_000, 0)
			.saturating_add(Weight::from_parts(0, 9864))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Attestation Attestations (r:1 w:1)
	/// Proof: Attestation Attestations (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842`
		//  Estimated: `7257`
		// Minimum execution time: 42_674_000 picoseconds.
		Weight::from_parts(43_047_000, 0)
			.saturating_add(Weight::from_parts(0, 7257))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_add() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 10810
		);
	}
	#[test]
	fn test_revoke() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3660
		);
	}
	#[test]
	fn test_remove() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7257
		);
	}
	#[test]
	fn test_reclaim_deposit() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7257
		);
	}
	#[test]
	fn test_change_deposit_owner() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 9864
		);
	}
	#[test]
	fn test_update_deposit() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7257
		);
	}
}
