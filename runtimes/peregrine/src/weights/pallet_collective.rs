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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet-collective
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:100 w:100)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3194 ±0)`
		//  Estimated: `15736 + m * (1967 ±16) + p * (4336 ±16)`
		// Minimum execution time: 19_175_000 picoseconds.
		Weight::from_parts(19_525_000, 0)
			.saturating_add(Weight::from_parts(0, 15736))
			// Standard Error: 43_751
			.saturating_add(Weight::from_parts(4_969_215, 0).saturating_mul(m.into()))
			// Standard Error: 43_751
			.saturating_add(Weight::from_parts(7_702_461, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4336).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `69 + m * (32 ±0)`
		//  Estimated: `1555 + m * (32 ±0)`
		// Minimum execution time: 18_842_000 picoseconds.
		Weight::from_parts(17_962_622, 0)
			.saturating_add(Weight::from_parts(0, 1555))
			// Standard Error: 16
			.saturating_add(Weight::from_parts(1_379, 0).saturating_mul(b.into()))
			// Standard Error: 165
			.saturating_add(Weight::from_parts(15_737, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `69 + m * (32 ±0)`
		//  Estimated: `3535 + m * (32 ±0)`
		// Minimum execution time: 21_589_000 picoseconds.
		Weight::from_parts(20_913_635, 0)
			.saturating_add(Weight::from_parts(0, 3535))
			// Standard Error: 16
			.saturating_add(Weight::from_parts(1_429, 0).saturating_mul(b.into()))
			// Standard Error: 172
			.saturating_add(Weight::from_parts(26_636, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `359 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `3751 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 28_645_000 picoseconds.
		Weight::from_parts(29_369_330, 0)
			.saturating_add(Weight::from_parts(0, 3751))
			// Standard Error: 49
			.saturating_add(Weight::from_parts(2_289, 0).saturating_mul(b.into()))
			// Standard Error: 517
			.saturating_add(Weight::from_parts(19_526, 0).saturating_mul(m.into()))
			// Standard Error: 510
			.saturating_add(Weight::from_parts(140_431, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `812 + m * (64 ±0)`
		//  Estimated: `4276 + m * (64 ±0)`
		// Minimum execution time: 24_489_000 picoseconds.
		Weight::from_parts(25_172_896, 0)
			.saturating_add(Weight::from_parts(0, 4276))
			// Standard Error: 201
			.saturating_add(Weight::from_parts(33_003, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3846 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 31_765_000 picoseconds.
		Weight::from_parts(32_037_066, 0)
			.saturating_add(Weight::from_parts(0, 3846))
			// Standard Error: 428
			.saturating_add(Weight::from_parts(22_205, 0).saturating_mul(m.into()))
			// Standard Error: 417
			.saturating_add(Weight::from_parts(135_895, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `703 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4020 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 43_739_000 picoseconds.
		Weight::from_parts(45_558_407, 0)
			.saturating_add(Weight::from_parts(0, 4020))
			// Standard Error: 129
			.saturating_add(Weight::from_parts(2_056, 0).saturating_mul(b.into()))
			// Standard Error: 1_335
			.saturating_add(Weight::from_parts(172_886, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3866 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 34_207_000 picoseconds.
		Weight::from_parts(34_939_608, 0)
			.saturating_add(Weight::from_parts(0, 3866))
			// Standard Error: 395
			.saturating_add(Weight::from_parts(22_999, 0).saturating_mul(m.into()))
			// Standard Error: 385
			.saturating_add(Weight::from_parts(132_997, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `723 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4040 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 46_782_000 picoseconds.
		Weight::from_parts(47_978_688, 0)
			.saturating_add(Weight::from_parts(0, 4040))
			// Standard Error: 86
			.saturating_add(Weight::from_parts(2_207, 0).saturating_mul(b.into()))
			// Standard Error: 911
			.saturating_add(Weight::from_parts(20_525, 0).saturating_mul(m.into()))
			// Standard Error: 888
			.saturating_add(Weight::from_parts(167_634, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `1711 + p * (32 ±0)`
		// Minimum execution time: 18_452_000 picoseconds.
		Weight::from_parts(19_982_326, 0)
			.saturating_add(Weight::from_parts(0, 1711))
			// Standard Error: 358
			.saturating_add(Weight::from_parts(125_678, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_set_members() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 15736
		);
	}
	#[test]
	fn test_execute() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1555
		);
	}
	#[test]
	fn test_propose_execute() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3535
		);
	}
	#[test]
	fn test_propose_proposed() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3751
		);
	}
	#[test]
	fn test_vote() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4276
		);
	}
	#[test]
	fn test_close_early_disapproved() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3846
		);
	}
	#[test]
	fn test_close_early_approved() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4020
		);
	}
	#[test]
	fn test_close_disapproved() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3866
		);
	}
	#[test]
	fn test_close_approved() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4040
		);
	}
	#[test]
	fn test_disapprove_proposal() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1711
		);
	}
}
