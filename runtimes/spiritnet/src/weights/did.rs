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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/did.rs
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
		Weight::from_ref_time(139_062_000 as u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(1_602_000 as u64).saturating_mul(n as u64))
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(6_002_000 as u64).saturating_mul(c as u64))
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
		Weight::from_ref_time(142_600_000 as u64)
			// Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(1_587_000 as u64).saturating_mul(n as u64))
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(6_483_000 as u64).saturating_mul(c as u64))
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
		Weight::from_ref_time(132_375_000 as u64)
			// Standard Error: 37_000
			.saturating_add(Weight::from_ref_time(1_497_000 as u64).saturating_mul(n as u64))
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(5_600_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn delete(c: u32, ) -> Weight {
		Weight::from_ref_time(35_098_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(1_080_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn reclaim_deposit(c: u32, ) -> Weight {
		Weight::from_ref_time(37_604_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_114_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ed25519_key() -> Weight {
		Weight::from_ref_time(81_034_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_sr25519_key() -> Weight {
		Weight::from_ref_time(83_045_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ecdsa_key() -> Weight {
		Weight::from_ref_time(72_485_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_authentication_key() -> Weight {
		Weight::from_ref_time(37_241_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_authentication_key() -> Weight {
		Weight::from_ref_time(37_473_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_authentication_key() -> Weight {
		Weight::from_ref_time(37_498_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(36_794_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(36_806_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(36_349_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(34_065_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(34_433_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(33_775_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(36_696_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(36_951_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn set_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(36_352_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(34_194_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(34_191_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(34_179_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(35_896_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(36_379_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn add_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(36_056_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(34_384_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(34_868_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:1)
	fn remove_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(34_643_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did ServiceEndpoints (r:1 w:1)
	fn add_service_endpoint() -> Weight {
		Weight::from_ref_time(37_784_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did ServiceEndpoints (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	fn remove_service_endpoint() -> Weight {
		Weight::from_ref_time(31_680_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		Weight::from_ref_time(66_507_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(4_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		Weight::from_ref_time(64_220_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		Weight::from_ref_time(55_901_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn transfer_deposit() -> Weight {
		Weight::from_ref_time(61_756_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
