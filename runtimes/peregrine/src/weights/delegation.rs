// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 33.0.0
//! DATE: 2024-06-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/runtime-weight-template.hbs
// --header=HEADER-GPL
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
	/// Storage: `Delegation::DelegationHierarchies` (r:1 w:1)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Ctype::Ctypes` (r:1 w:0)
	/// Proof: `Ctype::Ctypes` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationNodes` (r:0 w:1)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn create_hierarchy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `363`
		//  Estimated: `3658`
		// Minimum execution time: 54_184_000 picoseconds.
		Weight::from_parts(54_679_000, 0)
			.saturating_add(Weight::from_parts(0, 3658))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Delegation::DelegationNodes` (r:2 w:2)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn add_delegation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `502`
		//  Estimated: `70340`
		// Minimum execution time: 58_551_000 picoseconds.
		Weight::from_parts(59_230_000, 0)
			.saturating_add(Weight::from_parts(0, 70340))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Delegation::DelegationNodes` (r:5 w:5)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationHierarchies` (r:1 w:0)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + r * (239 ±0)`
		//  Estimated: `3545 + r * (34675 ±0)`
		// Minimum execution time: 23_789_000 picoseconds.
		Weight::from_parts(15_393_497, 0)
			.saturating_add(Weight::from_parts(0, 3545))
			// Standard Error: 15_631
			.saturating_add(Weight::from_parts(9_165_652, 0).saturating_mul(r.into()))
			// Standard Error: 15_631
			.saturating_add(Weight::from_parts(164_742, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:6 w:1)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationHierarchies` (r:1 w:0)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 5]`.
	/// The range of component `c` is `[1, 5]`.
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414 + c * (239 ±0)`
		//  Estimated: `35665 + c * (34675 ±0)`
		// Minimum execution time: 27_453_000 picoseconds.
		Weight::from_parts(24_815_813, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 17_140
			.saturating_add(Weight::from_parts(4_177_094, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:6 w:6)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:6 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationHierarchies` (r:1 w:1)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 5]`.
	fn remove_delegation(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `651 + r * (294 ±0)`
		//  Estimated: `35665 + r * (34675 ±0)`
		// Minimum execution time: 95_305_000 picoseconds.
		Weight::from_parts(62_278_882, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 50_940
			.saturating_add(Weight::from_parts(35_949_936, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:6 w:6)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:6 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationHierarchies` (r:0 w:1)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 5]`.
	fn reclaim_deposit(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583 + r * (294 ±0)`
		//  Estimated: `35665 + r * (34675 ±0)`
		// Minimum execution time: 90_454_000 picoseconds.
		Weight::from_parts(57_650_128, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 55_951
			.saturating_add(Weight::from_parts(36_036_209, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(r.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:1 w:0)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Delegation::DelegationHierarchies` (r:1 w:0)
	/// Proof: `Delegation::DelegationHierarchies` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn can_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `613`
		//  Estimated: `35665`
		// Minimum execution time: 12_053_000 picoseconds.
		Weight::from_parts(12_336_000, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `Delegation::DelegationNodes` (r:6 w:0)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 5]`.
	fn can_revoke(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + c * (240 ±0)`
		//  Estimated: `35665 + c * (34675 ±0)`
		// Minimum execution time: 11_015_000 picoseconds.
		Weight::from_parts(7_987_576, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 15_942
			.saturating_add(Weight::from_parts(3_572_065, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:6 w:0)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 5]`.
	fn can_remove(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + c * (240 ±0)`
		//  Estimated: `35665 + c * (34675 ±0)`
		// Minimum execution time: 10_991_000 picoseconds.
		Weight::from_parts(7_990_678, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			// Standard Error: 16_191
			.saturating_add(Weight::from_parts(3_563_410, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 34675).saturating_mul(c.into()))
	}
	/// Storage: `Delegation::DelegationNodes` (r:1 w:1)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `35665`
		// Minimum execution time: 84_092_000 picoseconds.
		Weight::from_parts(84_836_000, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Delegation::DelegationNodes` (r:1 w:1)
	/// Proof: `Delegation::DelegationNodes` (`max_values`: None, `max_size`: Some(32200), added: 34675, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `35665`
		// Minimum execution time: 76_973_000 picoseconds.
		Weight::from_parts(78_253_000, 0)
			.saturating_add(Weight::from_parts(0, 35665))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
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
				> 3658
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
				> 70340
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
				> 3545
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
				> 35665
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
				> 35665
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
				> 35665
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
				> 35665
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
				> 35665
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
				> 35665
		);
	}
}
