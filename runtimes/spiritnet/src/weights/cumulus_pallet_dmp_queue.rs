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

//! Autogenerated weights for `cumulus_pallet_dmp_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 33.0.0
//! DATE: 2024-06-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

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
// --chain=spiritnet-dev
// --pallet=cumulus_pallet_dmp_queue
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/cumulus_pallet_dmp_queue.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cumulus_pallet_dmp_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_dmp_queue::WeightInfo for WeightInfo<T> {
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65764`
		//  Estimated: `69229`
		// Minimum execution time: 131_789_000 picoseconds.
		Weight::from_parts(133_377_000, 0)
			.saturating_add(Weight::from_parts(0, 69229))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	fn on_idle_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65660`
		//  Estimated: `69125`
		// Minimum execution time: 67_376_000 picoseconds.
		Weight::from_parts(68_669_000, 0)
			.saturating_add(Weight::from_parts(0, 69125))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_overweight_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65794`
		//  Estimated: `69259`
		// Minimum execution time: 125_522_000 picoseconds.
		Weight::from_parts(126_924_000, 0)
			.saturating_add(Weight::from_parts(0, 69259))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	fn on_idle_overweight_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65690`
		//  Estimated: `69155`
		// Minimum execution time: 60_857_000 picoseconds.
		Weight::from_parts(61_583_000, 0)
			.saturating_add(Weight::from_parts(0, 69155))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_on_idle_good_msg() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 69229
		);
	}
	#[test]
	fn test_on_idle_large_msg() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 69125
		);
	}
	#[test]
	fn test_on_idle_overweight_good_msg() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 69259
		);
	}
	#[test]
	fn test_on_idle_overweight_large_msg() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 69155
		);
	}
}
