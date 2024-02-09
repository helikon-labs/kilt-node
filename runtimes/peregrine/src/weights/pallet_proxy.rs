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

//! Autogenerated weights for `pallet_proxy`
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
// --pallet=pallet-proxy
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/pallet_proxy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 9]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (41 ±0)`
		//  Estimated: `3932`
		// Minimum execution time: 17_752_000 picoseconds.
		Weight::from_parts(18_359_226, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			// Standard Error: 2_385
			.saturating_add(Weight::from_parts(55_493, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(777), added: 3252, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 9]`.
	/// The range of component `p` is `[1, 9]`.
	fn proxy_announced(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842 + a * (72 ±0) + p * (46 ±0)`
		//  Estimated: `4242`
		// Minimum execution time: 41_218_000 picoseconds.
		Weight::from_parts(42_446_761, 0)
			.saturating_add(Weight::from_parts(0, 4242))
			// Standard Error: 5_399
			.saturating_add(Weight::from_parts(200_543, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(777), added: 3252, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 9]`.
	/// The range of component `p` is `[1, 9]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757 + a * (72 ±0) + p * (5 ±0)`
		//  Estimated: `4242`
		// Minimum execution time: 26_822_000 picoseconds.
		Weight::from_parts(28_225_856, 0)
			.saturating_add(Weight::from_parts(0, 4242))
			// Standard Error: 4_687
			.saturating_add(Weight::from_parts(189_735, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(777), added: 3252, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 9]`.
	/// The range of component `p` is `[1, 9]`.
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757 + a * (72 ±0) + p * (5 ±0)`
		//  Estimated: `4242`
		// Minimum execution time: 26_947_000 picoseconds.
		Weight::from_parts(28_300_838, 0)
			.saturating_add(Weight::from_parts(0, 4242))
			// Standard Error: 5_013
			.saturating_add(Weight::from_parts(176_387, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(777), added: 3252, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 9]`.
	/// The range of component `p` is `[1, 9]`.
	fn announce(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `770 + a * (72 ±0) + p * (46 ±0)`
		//  Estimated: `4242`
		// Minimum execution time: 36_775_000 picoseconds.
		Weight::from_parts(37_340_302, 0)
			.saturating_add(Weight::from_parts(0, 4242))
			// Standard Error: 3_715
			.saturating_add(Weight::from_parts(254_492, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 9]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (41 ±0)`
		//  Estimated: `3932`
		// Minimum execution time: 27_351_000 picoseconds.
		Weight::from_parts(27_997_279, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			// Standard Error: 2_934
			.saturating_add(Weight::from_parts(51_101, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 9]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (41 ±0)`
		//  Estimated: `3932`
		// Minimum execution time: 27_399_000 picoseconds.
		Weight::from_parts(28_691_381, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 9]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (41 ±0)`
		//  Estimated: `3932`
		// Minimum execution time: 24_430_000 picoseconds.
		Weight::from_parts(25_129_279, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			// Standard Error: 2_419
			.saturating_add(Weight::from_parts(52_848, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 9]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3932`
		// Minimum execution time: 29_351_000 picoseconds.
		Weight::from_parts(29_860_918, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			// Standard Error: 2_417
			.saturating_add(Weight::from_parts(24_478, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(467), added: 2942, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 8]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168 + p * (41 ±0)`
		//  Estimated: `3932`
		// Minimum execution time: 25_596_000 picoseconds.
		Weight::from_parts(26_195_803, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			// Standard Error: 2_120
			.saturating_add(Weight::from_parts(57_903, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_proxy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
	#[test]
	fn test_proxy_announced() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4242
		);
	}
	#[test]
	fn test_remove_announcement() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4242
		);
	}
	#[test]
	fn test_reject_announcement() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4242
		);
	}
	#[test]
	fn test_announce() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4242
		);
	}
	#[test]
	fn test_add_proxy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
	#[test]
	fn test_remove_proxy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
	#[test]
	fn test_remove_proxies() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
	#[test]
	fn test_create_pure() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
	#[test]
	fn test_kill_pure() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3932
		);
	}
}
