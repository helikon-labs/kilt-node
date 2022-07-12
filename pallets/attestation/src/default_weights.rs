// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

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

//! Autogenerated weights for attestation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-15, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=attestation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/attestation/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for attestation.
pub trait WeightInfo {
	fn add() -> Weight;
	fn revoke() -> Weight;
	fn remove() -> Weight;
	fn reclaim_deposit() -> Weight;
}

/// Weights for attestation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add() -> Weight {
		(74_496_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn revoke() -> Weight {
		(37_029_000_u64)
			// Standard Error: 44_000
			.saturating_add(6_325_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove() -> Weight {
		(64_058_000_u64)
			// Standard Error: 44_000
			.saturating_add(6_317_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn reclaim_deposit() -> Weight {
		(56_873_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add() -> Weight {
		(74_496_000_u64)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn revoke() -> Weight {
		(37_029_000_u64)
			// Standard Error: 44_000
			.saturating_add(6_325_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove() -> Weight {
		(64_058_000_u64)
			// Standard Error: 44_000
			.saturating_add(6_317_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn reclaim_deposit() -> Weight {
		(56_873_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
