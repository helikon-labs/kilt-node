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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/did.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `did`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(129_254_324 as u64)
			// Standard Error: 10_271
			.saturating_add(Weight::from_ref_time(1_213_785 as u64).saturating_mul(n as u64))
			// Standard Error: 3_971
			.saturating_add(Weight::from_ref_time(4_946_458 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(131_947_134 as u64)
			// Standard Error: 10_062
			.saturating_add(Weight::from_ref_time(1_203_011 as u64).saturating_mul(n as u64))
			// Standard Error: 3_890
			.saturating_add(Weight::from_ref_time(5_661_917 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Did DidBlacklist (r:1 w:0)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:25)
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(120_785_283 as u64)
			// Standard Error: 11_980
			.saturating_add(Weight::from_ref_time(1_127_190 as u64).saturating_mul(n as u64))
			// Standard Error: 4_632
			.saturating_add(Weight::from_ref_time(4_638_520 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn delete(c: u32, ) -> Weight {
		Weight::from_ref_time(35_557_626 as u64)
			// Standard Error: 17_121
			.saturating_add(Weight::from_ref_time(1_449_091 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn reclaim_deposit(c: u32, ) -> Weight {
		Weight::from_ref_time(39_526_415 as u64)
			// Standard Error: 20_871
			.saturating_add(Weight::from_ref_time(1_341_463 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ed25519_key() -> Weight {
		Weight::from_ref_time(83_153_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_sr25519_key() -> Weight {
		Weight::from_ref_time(85_417_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ecdsa_key() -> Weight {
		Weight::from_ref_time(73_957_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_authentication_key() -> Weight {
		Weight::from_ref_time(35_365_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_authentication_key() -> Weight {
		Weight::from_ref_time(35_489_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_authentication_key() -> Weight {
		Weight::from_ref_time(35_153_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(35_034_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(34_791_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(34_839_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(32_603_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(32_679_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(32_534_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(34_825_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(34_502_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(34_945_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(32_404_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(32_482_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(32_483_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(33_820_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(34_059_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(34_290_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(32_827_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(32_849_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(32_844_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did ServiceEndpoints (r:1 w:1)
	fn add_service_endpoint() -> Weight {
		Weight::from_ref_time(40_743_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did ServiceEndpoints (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	fn remove_service_endpoint() -> Weight {
		Weight::from_ref_time(34_299_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		Weight::from_ref_time(18_586_357 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(4_855 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		Weight::from_ref_time(18_962_750 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(2_451 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		Weight::from_ref_time(6_344_961 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_482 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(45_930_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(47_692_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	// Storage: Did Did (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn propagate_root() -> Weight {
		Weight::from_ref_time(64_599_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
