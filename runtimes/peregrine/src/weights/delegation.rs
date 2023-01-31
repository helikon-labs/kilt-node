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

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/delegation.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `delegation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> delegation::WeightInfo for WeightInfo<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		Weight::from_ref_time(57_917_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		Weight::from_ref_time(62_749_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(32_160_000 as u64)
			// Standard Error: 37_000
			.saturating_add(Weight::from_ref_time(14_117_000 as u64).saturating_mul(r as u64))
			// Standard Error: 37_000
			.saturating_add(Weight::from_ref_time(467_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(48_395_000 as u64)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(4_088_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		Weight::from_ref_time(68_696_000 as u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(24_154_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		Weight::from_ref_time(60_232_000 as u64)
			// Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(24_455_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		Weight::from_ref_time(18_381_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_revoke(c: u32, ) -> Weight {
		Weight::from_ref_time(12_919_000 as u64)
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(4_488_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_remove(c: u32, ) -> Weight {
		Weight::from_ref_time(12_874_000 as u64)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(4_494_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(69_415_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(64_222_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
