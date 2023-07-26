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

//! Autogenerated weights for pallet_did_lookup
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-25
//! STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/weight-template.hbs
// --header=HEADER-GPL
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=dev
// --pallet=pallet-did-lookup
// --extrinsic=*
// --output=./pallets/pallet-did-lookup/src/default_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_did_lookup.
pub trait WeightInfo {
	fn associate_account_multisig_sr25519() -> Weight;
	fn associate_account_multisig_ed25519() -> Weight;
	fn associate_account_multisig_ecdsa() -> Weight;
	fn associate_eth_account() -> Weight;
	fn associate_sender() -> Weight;
	fn remove_sender_association() -> Weight;
	fn remove_account_association() -> Weight;
	fn change_deposit_owner() -> Weight;
	fn update_deposit() -> Weight;
}

/// Weights for pallet_did_lookup using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_sr25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 112_624 nanoseconds.
		Weight::from_parts(113_903_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_ed25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 110_641 nanoseconds.
		Weight::from_parts(113_848_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_ecdsa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 99_859 nanoseconds.
		Weight::from_parts(101_823_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_eth_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `7191`
		// Minimum execution time: 100_902 nanoseconds.
		Weight::from_parts(102_547_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_sender() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 93_066 nanoseconds.
		Weight::from_parts(93_643_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn remove_sender_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 32_778 nanoseconds.
		Weight::from_parts(33_472_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn remove_account_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 35_213 nanoseconds.
		Weight::from_parts(35_608_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `721`
		//  Estimated: `9798`
		// Minimum execution time: 45_805 nanoseconds.
		Weight::from_parts(46_104_000, 9798)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 41_349 nanoseconds.
		Weight::from_parts(42_246_000, 7191)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_sr25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 112_624 nanoseconds.
		Weight::from_parts(113_903_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_ed25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 110_641 nanoseconds.
		Weight::from_parts(113_848_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_account_multisig_ecdsa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 99_859 nanoseconds.
		Weight::from_parts(101_823_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_eth_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `7191`
		// Minimum execution time: 100_902 nanoseconds.
		Weight::from_parts(102_547_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn associate_sender() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 93_066 nanoseconds.
		Weight::from_parts(93_643_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn remove_sender_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 32_778 nanoseconds.
		Weight::from_parts(33_472_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	/// Proof: DidLookup ConnectedAccounts (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	fn remove_account_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 35_213 nanoseconds.
		Weight::from_parts(35_608_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `721`
		//  Estimated: `9798`
		// Minimum execution time: 45_805 nanoseconds.
		Weight::from_parts(46_104_000, 9798)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: DidLookup ConnectedDids (r:1 w:1)
	/// Proof: DidLookup ConnectedDids (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `7191`
		// Minimum execution time: 41_349 nanoseconds.
		Weight::from_parts(42_246_000, 7191)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
