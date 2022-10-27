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

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=parachain-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/parachain_staking.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking Round (r:1 w:0)
	fn on_initialize_no_action() -> Weight {
		Weight::from_ref_time(7_676_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		Weight::from_ref_time(21_767_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn on_initialize_new_year() -> Weight {
		Weight::from_ref_time(34_117_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		Weight::from_ref_time(67_526_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		Weight::from_ref_time(8_397_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:0 w:1)
	fn set_inflation() -> Weight {
		Weight::from_ref_time(23_106_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 153_000
			.saturating_add(Weight::from_ref_time(17_438_000 as u64).saturating_mul(n as u64))
			// Standard Error: 257_000
			.saturating_add(Weight::from_ref_time(26_761_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(29_651_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 168_000
			.saturating_add(Weight::from_ref_time(17_200_000 as u64).saturating_mul(n as u64))
			// Standard Error: 277_000
			.saturating_add(Weight::from_ref_time(46_332_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(25 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(m as u64)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 146_000
			.saturating_add(Weight::from_ref_time(14_104_000 as u64).saturating_mul(n as u64))
			// Standard Error: 303_000
			.saturating_add(Weight::from_ref_time(28_540_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 173_000
			.saturating_add(Weight::from_ref_time(17_832_000 as u64).saturating_mul(n as u64))
			// Standard Error: 281_000
			.saturating_add(Weight::from_ref_time(32_782_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(21 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 182_000
			.saturating_add(Weight::from_ref_time(17_990_000 as u64).saturating_mul(n as u64))
			// Standard Error: 296_000
			.saturating_add(Weight::from_ref_time(33_359_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 176_000
			.saturating_add(Weight::from_ref_time(20_889_000 as u64).saturating_mul(n as u64))
			// Standard Error: 285_000
			.saturating_add(Weight::from_ref_time(53_045_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(m as u64)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_more(n: u32, m: u32, _u: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 141_000
			.saturating_add(Weight::from_ref_time(18_896_000 as u64).saturating_mul(n as u64))
			// Standard Error: 293_000
			.saturating_add(Weight::from_ref_time(38_642_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 161_000
			.saturating_add(Weight::from_ref_time(17_682_000 as u64).saturating_mul(n as u64))
			// Standard Error: 335_000
			.saturating_add(Weight::from_ref_time(36_380_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking LastDelegation (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 152_000
			.saturating_add(Weight::from_ref_time(17_569_000 as u64).saturating_mul(n as u64))
			// Standard Error: 339_000
			.saturating_add(Weight::from_ref_time(38_079_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_more(n: u32, m: u32, _u: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 132_000
			.saturating_add(Weight::from_ref_time(18_908_000 as u64).saturating_mul(n as u64))
			// Standard Error: 293_000
			.saturating_add(Weight::from_ref_time(41_146_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 156_000
			.saturating_add(Weight::from_ref_time(17_796_000 as u64).saturating_mul(n as u64))
			// Standard Error: 347_000
			.saturating_add(Weight::from_ref_time(38_534_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn revoke_delegation(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 156_000
			.saturating_add(Weight::from_ref_time(18_126_000 as u64).saturating_mul(n as u64))
			// Standard Error: 347_000
			.saturating_add(Weight::from_ref_time(39_008_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 157_000
			.saturating_add(Weight::from_ref_time(18_273_000 as u64).saturating_mul(n as u64))
			// Standard Error: 349_000
			.saturating_add(Weight::from_ref_time(39_507_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_unstaked(u: u32, ) -> Weight {
		Weight::from_ref_time(57_821_000 as u64)
			// Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(181_000 as u64).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		Weight::from_ref_time(22_635_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
