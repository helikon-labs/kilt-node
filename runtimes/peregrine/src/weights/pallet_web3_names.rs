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

//! Autogenerated weights for `pallet_web3_names`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

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
// --chain=dev
// --pallet=pallet-web3-names
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/pallet_web3_names.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_web3_names`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_web3_names::WeightInfo for WeightInfo<T> {
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: Web3Names Banned (r:1 w:0)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn claim(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `112`
		//  Estimated: `14259`
		// Minimum execution time: 38_463_000 picoseconds.
		Weight::from_parts(64_883_939, 0)
			.saturating_add(Weight::from_parts(0, 14259))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn release_by_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `382`
		//  Estimated: `10745`
		// Minimum execution time: 37_480_000 picoseconds.
		Weight::from_parts(37_928_000, 0)
			.saturating_add(Weight::from_parts(0, 10745))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn reclaim_deposit(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + n * (1 ±0)`
		//  Estimated: `7199`
		// Minimum execution time: 33_540_000 picoseconds.
		Weight::from_parts(45_409_661, 0)
			.saturating_add(Weight::from_parts(0, 7199))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn ban(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + n * (1 ±0)`
		//  Estimated: `10713`
		// Minimum execution time: 37_229_000 picoseconds.
		Weight::from_parts(55_165_737, 0)
			.saturating_add(Weight::from_parts(0, 10713))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn unban(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `47 + n * (1 ±0)`
		//  Estimated: `3514`
		// Minimum execution time: 17_356_000 picoseconds.
		Weight::from_parts(23_157_909, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Web3Names Names (r:1 w:0)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `763`
		//  Estimated: `13352`
		// Minimum execution time: 83_486_000 picoseconds.
		Weight::from_parts(84_164_000, 0)
			.saturating_add(Weight::from_parts(0, 13352))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `7199`
		// Minimum execution time: 42_487_000 picoseconds.
		Weight::from_parts(43_332_000, 0)
			.saturating_add(Weight::from_parts(0, 7199))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_claim() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 14259
		);
	}
	#[test]
	fn test_release_by_owner() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 10745
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
				> 7199
		);
	}
	#[test]
	fn test_ban() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 10713
		);
	}
	#[test]
	fn test_unban() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3514
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
				> 13352
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
				> 7199
		);
	}
}
