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

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		Weight::from_ref_time(15_785_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		Weight::from_ref_time(46_933_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		Weight::from_ref_time(55_668_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		Weight::from_ref_time(15_433_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:2 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:72 w:72)
	// Storage: ParachainStaking Rewards (r:72 w:72)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:0)
	fn set_inflation(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(860_135_000 as u64)
			// Standard Error: 5_204_549
			.saturating_add(Weight::from_ref_time(166_980_678 as u64).saturating_mul(n as u64))
			// Standard Error: 11_174_052
			.saturating_add(Weight::from_ref_time(319_821_281 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(152 as u64))
			.saturating_add(T::DbWeight::get().reads((27 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((51 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(145 as u64))
			.saturating_add(T::DbWeight::get().writes((25 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((51 as u64).saturating_mul(m as u64)))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(71_385_000 as u64)
			// Standard Error: 27_673
			.saturating_add(Weight::from_ref_time(3_574_268 as u64).saturating_mul(n as u64))
			// Standard Error: 61_251
			.saturating_add(Weight::from_ref_time(393_012 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(28_124_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:1)
	// Storage: ParachainStaking BlocksRewarded (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:36 w:36)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(168_116_000 as u64)
			// Standard Error: 34_857
			.saturating_add(Weight::from_ref_time(1_232_650 as u64).saturating_mul(n as u64))
			// Standard Error: 77_327
			.saturating_add(Weight::from_ref_time(31_901_426 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(29 as u64))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(m as u64)))
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
		Weight::from_ref_time(77_165_699 as u64)
			// Standard Error: 15_581
			.saturating_add(Weight::from_ref_time(831_262 as u64).saturating_mul(n as u64))
			// Standard Error: 32_355
			.saturating_add(Weight::from_ref_time(1_635_259 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(101_464_502 as u64)
			// Standard Error: 16_047
			.saturating_add(Weight::from_ref_time(888_794 as u64).saturating_mul(n as u64))
			// Standard Error: 26_023
			.saturating_add(Weight::from_ref_time(1_978_806 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(21 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(62_095_826 as u64)
			// Standard Error: 12_132
			.saturating_add(Weight::from_ref_time(476_338 as u64).saturating_mul(n as u64))
			// Standard Error: 19_675
			.saturating_add(Weight::from_ref_time(1_129_496 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:1)
	// Storage: ParachainStaking BlocksRewarded (r:36 w:36)
	// Storage: ParachainStaking Rewards (r:36 w:36)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(101_890_000 as u64)
			// Standard Error: 30_385
			.saturating_add(Weight::from_ref_time(1_040_081 as u64).saturating_mul(n as u64))
			// Standard Error: 66_546
			.saturating_add(Weight::from_ref_time(32_125_693 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(m as u64)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		Weight::from_ref_time(71_470_018 as u64)
			// Standard Error: 13_867
			.saturating_add(Weight::from_ref_time(938_029 as u64).saturating_mul(n as u64))
			// Standard Error: 28_786
			.saturating_add(Weight::from_ref_time(1_857_412 as u64).saturating_mul(m as u64))
			// Standard Error: 106_632
			.saturating_add(Weight::from_ref_time(1_932_519 as u64).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(61_169_638 as u64)
			// Standard Error: 16_197
			.saturating_add(Weight::from_ref_time(875_175 as u64).saturating_mul(n as u64))
			// Standard Error: 33_635
			.saturating_add(Weight::from_ref_time(1_699_929 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
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
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:0 w:1)
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(78_807_697 as u64)
			// Standard Error: 14_488
			.saturating_add(Weight::from_ref_time(1_065_735 as u64).saturating_mul(n as u64))
			// Standard Error: 32_201
			.saturating_add(Weight::from_ref_time(2_188_025 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		Weight::from_ref_time(83_685_009 as u64)
			// Standard Error: 12_652
			.saturating_add(Weight::from_ref_time(974_920 as u64).saturating_mul(n as u64))
			// Standard Error: 28_109
			.saturating_add(Weight::from_ref_time(1_928_583 as u64).saturating_mul(m as u64))
			// Standard Error: 109_022
			.saturating_add(Weight::from_ref_time(1_733_864 as u64).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(74_505_829 as u64)
			// Standard Error: 14_680
			.saturating_add(Weight::from_ref_time(892_703 as u64).saturating_mul(n as u64))
			// Standard Error: 32_626
			.saturating_add(Weight::from_ref_time(1_762_577 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(79_400_932 as u64)
			// Standard Error: 13_884
			.saturating_add(Weight::from_ref_time(898_989 as u64).saturating_mul(n as u64))
			// Standard Error: 30_857
			.saturating_add(Weight::from_ref_time(1_759_657 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_unstaked(u: u32, ) -> Weight {
		Weight::from_ref_time(54_926_082 as u64)
			// Standard Error: 86_974
			.saturating_add(Weight::from_ref_time(202_637 as u64).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		Weight::from_ref_time(20_662_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn increment_delegator_rewards() -> Weight {
		Weight::from_ref_time(40_748_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:1 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:1 w:1)
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	fn increment_collator_rewards() -> Weight {
		Weight::from_ref_time(35_860_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_rewards() -> Weight {
		Weight::from_ref_time(53_997_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:3 w:0)
	// Storage: ParachainStaking BlocksAuthored (r:2 w:0)
	// Storage: ParachainStaking BlocksRewarded (r:72 w:72)
	// Storage: ParachainStaking Rewards (r:72 w:72)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:0)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:0)
	fn execute_scheduled_reward_change(n: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(850_431_000 as u64)
			// Standard Error: 5_147_003
			.saturating_add(Weight::from_ref_time(163_355_731 as u64).saturating_mul(n as u64))
			// Standard Error: 11_050_503
			.saturating_add(Weight::from_ref_time(316_055_688 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(153 as u64))
			.saturating_add(T::DbWeight::get().reads((27 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((51 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(146 as u64))
			.saturating_add(T::DbWeight::get().writes((25 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((51 as u64).saturating_mul(m as u64)))
	}
}
