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

//! Autogenerated weights for `pallet_vesting`
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
// --pallet=pallet-vesting
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/pallet_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `9398`
		// Minimum execution time: 39_051_000 picoseconds.
		Weight::from_parts(38_023_834, 0)
			.saturating_add(Weight::from_parts(0, 9398))
			// Standard Error: 2_462
			.saturating_add(Weight::from_parts(61_419, 0).saturating_mul(l.into()))
			// Standard Error: 4_380
			.saturating_add(Weight::from_parts(87_864, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `9398`
		// Minimum execution time: 37_151_000 picoseconds.
		Weight::from_parts(38_707_034, 0)
			.saturating_add(Weight::from_parts(0, 9398))
			// Standard Error: 7_129
			.saturating_add(Weight::from_parts(26_862, 0).saturating_mul(l.into()))
			// Standard Error: 12_684
			.saturating_add(Weight::from_parts(44_343, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1225 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `12995`
		// Minimum execution time: 42_428_000 picoseconds.
		Weight::from_parts(43_847_219, 0)
			.saturating_add(Weight::from_parts(0, 12995))
			// Standard Error: 8_664
			.saturating_add(Weight::from_parts(38_153, 0).saturating_mul(l.into()))
			// Standard Error: 15_416
			.saturating_add(Weight::from_parts(65_375, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1225 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `12995`
		// Minimum execution time: 41_218_000 picoseconds.
		Weight::from_parts(42_909_315, 0)
			.saturating_add(Weight::from_parts(0, 12995))
			// Standard Error: 5_748
			.saturating_add(Weight::from_parts(31_453, 0).saturating_mul(l.into()))
			// Standard Error: 10_226
			.saturating_add(Weight::from_parts(23_960, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1423 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `12995`
		// Minimum execution time: 59_715_000 picoseconds.
		Weight::from_parts(61_476_486, 0)
			.saturating_add(Weight::from_parts(0, 12995))
			// Standard Error: 10_132
			.saturating_add(Weight::from_parts(50_994, 0).saturating_mul(l.into()))
			// Standard Error: 18_027
			.saturating_add(Weight::from_parts(53_279, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1733 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `15602`
		// Minimum execution time: 63_836_000 picoseconds.
		Weight::from_parts(70_221_442, 0)
			.saturating_add(Weight::from_parts(0, 15602))
			// Standard Error: 13_985
			.saturating_add(Weight::from_parts(54_718, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `958 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `12995`
		// Minimum execution time: 42_973_000 picoseconds.
		Weight::from_parts(45_719_053, 0)
			.saturating_add(Weight::from_parts(0, 12995))
			// Standard Error: 5_074
			.saturating_add(Weight::from_parts(21_358, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1169), added: 3644, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `958 + l * (25 ±0) + s * (40 ±0)`
		//  Estimated: `12995`
		// Minimum execution time: 43_040_000 picoseconds.
		Weight::from_parts(43_426_102, 0)
			.saturating_add(Weight::from_parts(0, 12995))
			// Standard Error: 6_147
			.saturating_add(Weight::from_parts(58_590, 0).saturating_mul(l.into()))
			// Standard Error: 11_353
			.saturating_add(Weight::from_parts(33_362, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_vest_locked() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 9398
		);
	}
	#[test]
	fn test_vest_unlocked() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 9398
		);
	}
	#[test]
	fn test_vest_other_locked() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 12995
		);
	}
	#[test]
	fn test_vest_other_unlocked() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 12995
		);
	}
	#[test]
	fn test_vested_transfer() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 12995
		);
	}
	#[test]
	fn test_force_vested_transfer() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 15602
		);
	}
	#[test]
	fn test_not_unlocking_merge_schedules() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 12995
		);
	}
	#[test]
	fn test_unlocking_merge_schedules() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 12995
		);
	}
}
