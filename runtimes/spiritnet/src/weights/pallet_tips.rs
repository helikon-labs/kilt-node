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

//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

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
// --chain=spiritnet-dev
// --pallet=pallet-tips
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/pallet_tips.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `6938`
		// Minimum execution time: 29_979_000 picoseconds.
		Weight::from_parts(44_762_847, 0)
			.saturating_add(Weight::from_parts(0, 6938))
			// Standard Error: 105
			.saturating_add(Weight::from_parts(1_246, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3907`
		// Minimum execution time: 30_545_000 picoseconds.
		Weight::from_parts(52_508_000, 0)
			.saturating_add(Weight::from_parts(0, 3907))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TipsMembership Members (r:1 w:0)
	/// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(673), added: 1168, mode: MaxEncodedLen)
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:0 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 21]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + t * (32 ±0)`
		//  Estimated: `5905 + t * (64 ±0)`
		// Minimum execution time: 25_227_000 picoseconds.
		Weight::from_parts(29_785_727, 0)
			.saturating_add(Weight::from_parts(0, 5905))
			// Standard Error: 48
			.saturating_add(Weight::from_parts(1_329, 0).saturating_mul(r.into()))
			// Standard Error: 38_608
			.saturating_add(Weight::from_parts(210_269, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(t.into()))
	}
	/// Storage: TipsMembership Members (r:1 w:0)
	/// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(673), added: 1168, mode: MaxEncodedLen)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 21]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + t * (80 ±0)`
		//  Estimated: `5989 + t * (80 ±0)`
		// Minimum execution time: 19_262_000 picoseconds.
		Weight::from_parts(27_079_300, 0)
			.saturating_add(Weight::from_parts(0, 5989))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: TipsMembership Members (r:1 w:0)
	/// Proof: TipsMembership Members (max_values: Some(1), max_size: Some(673), added: 1168, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 21]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1183 + t * (80 ±0)`
		//  Estimated: `14101 + t * (162 ±1)`
		// Minimum execution time: 58_294_000 picoseconds.
		Weight::from_parts(83_950_284, 0)
			.saturating_add(Weight::from_parts(0, 14101))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 162).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 21]`.
	fn slash_tip(_t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `4003`
		// Minimum execution time: 18_931_000 picoseconds.
		Weight::from_parts(23_160_134, 0)
			.saturating_add(Weight::from_parts(0, 4003))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_report_awesome() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6938
		);
	}
	#[test]
	fn test_retract_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3907
		);
	}
	#[test]
	fn test_tip_new() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5905
		);
	}
	#[test]
	fn test_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5989
		);
	}
	#[test]
	fn test_close_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 14101
		);
	}
	#[test]
	fn test_slash_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4003
		);
	}
}
