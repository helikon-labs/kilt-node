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

//! Autogenerated weights for pallet_tips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_tips.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(46_140_143 as u64)
			// Standard Error: 95
			.saturating_add(Weight::from_ref_time(1_230 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(27_002_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TipsMembership Members (r:1 w:0)
	// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	// Storage: Tips Reasons (r:1 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Tips (r:0 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(26_851_327 as u64)
			// Standard Error: 29
			.saturating_add(Weight::from_ref_time(1_934 as u64).saturating_mul(r as u64))
			// Standard Error: 4_814
			.saturating_add(Weight::from_ref_time(23_555 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TipsMembership Members (r:1 w:0)
	// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(20_421_194 as u64)
			// Standard Error: 4_172
			.saturating_add(Weight::from_ref_time(95_572 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: TipsMembership Members (r:1 w:0)
	// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn close_tip(_t: u32, ) -> Weight {
		Weight::from_ref_time(62_396_084 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn slash_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(19_666_583 as u64)
			// Standard Error: 4_641
			.saturating_add(Weight::from_ref_time(450 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
