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

//! Autogenerated weights for pallet_web3_names
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=pallet-web3-names
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/pallet_web3_names.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_web3_names`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_web3_names::WeightInfo for WeightInfo<T> {
	// Storage: Web3Names Names (r:1 w:1)
	// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: Web3Names Banned (r:1 w:0)
	// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn claim(_n: u32, ) -> Weight {
		Weight::from_parts(40_237_964 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Names (r:1 w:1)
	// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn release_by_owner() -> Weight {
		Weight::from_parts(32_912_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	// Storage: Web3Names Names (r:0 w:1)
	// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn reclaim_deposit(n: u32, ) -> Weight {
		Weight::from_parts(31_703_821 as u64, 0)
			// Standard Error: 36_296
			.saturating_add(Weight::from_parts(235_783 as u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	// Storage: Web3Names Names (r:0 w:1)
	// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn ban(_n: u32, ) -> Weight {
		Weight::from_parts(43_007_574 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn unban(_n: u32, ) -> Weight {
		Weight::from_parts(17_954_246 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Web3Names Names (r:1 w:0)
	// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		Weight::from_parts(66_017_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		Weight::from_parts(57_911_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
