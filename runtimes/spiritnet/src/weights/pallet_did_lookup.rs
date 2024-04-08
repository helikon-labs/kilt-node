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

//! Autogenerated weights for `pallet_did_lookup`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet-did-lookup
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/pallet_did_lookup.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_did_lookup`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did_lookup::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn associate_account_multisig_sr25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `4414`
		// Minimum execution time: 149_149_000 picoseconds.
		Weight::from_parts(150_280_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn associate_account_multisig_ed25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `4414`
		// Minimum execution time: 147_787_000 picoseconds.
		Weight::from_parts(148_719_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn associate_account_multisig_ecdsa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `4414`
		// Minimum execution time: 136_184_000 picoseconds.
		Weight::from_parts(137_771_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn associate_eth_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `417`
		//  Estimated: `4414`
		// Minimum execution time: 139_089_000 picoseconds.
		Weight::from_parts(140_345_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn associate_sender() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `4414`
		// Minimum execution time: 88_095_000 picoseconds.
		Weight::from_parts(88_978_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:1)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	fn remove_sender_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `4414`
		// Minimum execution time: 53_261_000 picoseconds.
		Weight::from_parts(54_215_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:1)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	fn remove_account_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `4414`
		// Minimum execution time: 54_841_000 picoseconds.
		Weight::from_parts(55_291_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `889`
		//  Estimated: `7838`
		// Minimum execution time: 84_230_000 picoseconds.
		Weight::from_parts(84_955_000, 0)
			.saturating_add(Weight::from_parts(0, 7838))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(949), added: 3424, mode: `MaxEncodedLen`)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `4414`
		// Minimum execution time: 76_722_000 picoseconds.
		Weight::from_parts(77_395_000, 0)
			.saturating_add(Weight::from_parts(0, 4414))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_associate_account_multisig_sr25519() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_associate_account_multisig_ed25519() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_associate_account_multisig_ecdsa() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_associate_eth_account() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_associate_sender() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_remove_sender_association() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_remove_account_association() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
	#[test]
	fn test_change_deposit_owner() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7838
		);
	}
	#[test]
	fn test_update_deposit() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4414
		);
	}
}
