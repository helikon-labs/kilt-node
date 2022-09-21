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

//! Autogenerated weights for pallet_did_lookup
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-did-lookup
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_did-lookup.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_did_lookup`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did_lookup::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_sr25519() -> Weight {
		Weight::from_ref_time(117_104_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ed25519() -> Weight {
		Weight::from_ref_time(114_593_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_account_multisig_ecdsa() -> Weight {
		Weight::from_ref_time(106_042_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_eth_account() -> Weight {
		Weight::from_ref_time(108_327_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:2)
	fn associate_sender() -> Weight {
		Weight::from_ref_time(58_083_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_sender_association() -> Weight {
		Weight::from_ref_time(39_015_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: DidLookup ConnectedDids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DidLookup ConnectedAccounts (r:0 w:1)
	fn remove_account_association() -> Weight {
		Weight::from_ref_time(40_687_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn transfer_deposit() -> Weight {
		Weight::from_ref_time(40_723_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
