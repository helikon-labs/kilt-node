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

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=pallet-collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/pallet_collective.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(9_279_000 as u64).saturating_mul(m as u64))
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(13_278_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(32_650_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(22_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(36_100_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(33_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(46_604_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(b as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(28_000 as u64).saturating_mul(m as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(237_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(51_121_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(55_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(49_150_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(56_000 as u64).saturating_mul(m as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(210_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(64_393_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(b as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(44_000 as u64).saturating_mul(m as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(234_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(53_938_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(39_000 as u64).saturating_mul(m as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(226_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(67_728_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(b as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(39_000 as u64).saturating_mul(m as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(239_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(36_721_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(216_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
