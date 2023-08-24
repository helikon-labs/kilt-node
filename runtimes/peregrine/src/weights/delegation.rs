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

//! Autogenerated weights for `delegation`
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
// --pallet=delegation
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/delegation.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `delegation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> delegation::WeightInfo for WeightInfo<T> {
	/// Storage: Delegation DelegationHierarchies (r:1 w:1)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Ctype Ctypes (r:1 w:0)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationNodes (r:0 w:1)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	fn create_hierarchy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `864`
		//  Estimated: `10695`
		// Minimum execution time: 40_117_000 picoseconds.
		Weight::from_parts(41_127_000, 0)
			.saturating_add(Weight::from_parts(0, 10695))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Delegation DelegationNodes (r:2 w:2)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn add_delegation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `985`
		//  Estimated: `73937`
		// Minimum execution time: 42_234_000 picoseconds.
		Weight::from_parts(43_070_000, 0)
			.saturating_add(Weight::from_parts(0, 73937))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Delegation DelegationNodes (r:5 w:5)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_root_child(r: u32, _c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + r * (239 ±0)`
		//  Estimated: `4535 + r * (34675 ±0)`
		// Minimum execution time: 30_423_000 picoseconds.
		Weight::from_parts(19_299_702, 0)
			.saturating_add(Weight::from_parts(0, 4535))
			// Standard Error: 21_162
			.saturating_add(Weight::from_parts(13_603_622, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: Delegation DelegationNodes (r:6 w:1)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414 + c * (239 ±0)`
		//  Estimated: `39210 + c * (34675 ±0)`
		// Minimum execution time: 34_294_000 picoseconds.
		Weight::from_parts(30_070_289, 0)
			.saturating_add(Weight::from_parts(0, 39210))
			// Standard Error: 15_628
			.saturating_add(Weight::from_parts(4_876_959, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: Delegation DelegationNodes (r:6 w:6)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationHierarchies (r:1 w:1)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 5]`.
	fn remove_delegation(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1055 + r * (240 ±0)`
		//  Estimated: `42807 + r * (34675 ±0)`
		// Minimum execution time: 71_095_000 picoseconds.
		Weight::from_parts(50_569_323, 0)
			.saturating_add(Weight::from_parts(0, 42807))
			// Standard Error: 53_141
			.saturating_add(Weight::from_parts(23_694_270, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: Delegation DelegationNodes (r:6 w:6)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationHierarchies (r:0 w:1)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 5]`.
	fn reclaim_deposit(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `987 + r * (240 ±0)`
		//  Estimated: `39262 + r * (34675 ±0)`
		// Minimum execution time: 64_137_000 picoseconds.
		Weight::from_parts(41_818_994, 0)
			.saturating_add(Weight::from_parts(0, 39262))
			// Standard Error: 34_246
			.saturating_add(Weight::from_parts(23_779_982, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: Delegation DelegationNodes (r:1 w:0)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: Delegation DelegationHierarchies (r:1 w:0)
	/// Proof: Delegation DelegationHierarchies (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn can_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `613`
		//  Estimated: `39210`
		// Minimum execution time: 13_333_000 picoseconds.
		Weight::from_parts(13_673_000, 0)
			.saturating_add(Weight::from_parts(0, 39210))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: Delegation DelegationNodes (r:6 w:0)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 5]`.
	fn can_revoke(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + c * (240 ±0)`
		//  Estimated: `35665 + c * (34675 ±0)`
		// Minimum execution time: 12_190_000 picoseconds.
		Weight::from_parts(8_916_318, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 17_296
			.saturating_add(Weight::from_parts(4_010_992, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: Delegation DelegationNodes (r:6 w:0)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 5]`.
	fn can_remove(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + c * (240 ±0)`
		//  Estimated: `35665 + c * (34675 ±0)`
		// Minimum execution time: 12_165_000 picoseconds.
		Weight::from_parts(8_859_796, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 16_892
			.saturating_add(Weight::from_parts(4_031_715, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: Delegation DelegationNodes (r:1 w:1)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1600`
		//  Estimated: `41869`
		// Minimum execution time: 49_874_000 picoseconds.
		Weight::from_parts(50_659_000, 0)
			.saturating_add(Weight::from_parts(0, 41869))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Delegation DelegationNodes (r:1 w:1)
	/// Proof: Delegation DelegationNodes (max_values: None, max_size: Some(32200), added: 34675, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1053`
		//  Estimated: `39262`
		// Minimum execution time: 45_593_000 picoseconds.
		Weight::from_parts(46_939_000, 0)
			.saturating_add(Weight::from_parts(0, 39262))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_create_hierarchy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 10695
		);
	}
	#[test]
	fn test_add_delegation() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 73937
		);
	}
	#[test]
	fn test_revoke_delegation_root_child() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4535
		);
	}
	#[test]
	fn test_revoke_delegation_leaf() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 39210
		);
	}
	#[test]
	fn test_remove_delegation() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 42807
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
				> 39262
		);
	}
	#[test]
	fn test_can_attest() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 39210
		);
	}
	#[test]
	fn test_can_revoke() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 35665
		);
	}
	#[test]
	fn test_can_remove() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 35665
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
				> 41869
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
				> 39262
		);
	}
}
