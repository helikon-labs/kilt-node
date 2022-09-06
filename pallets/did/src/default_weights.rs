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
//! DATE: 2021-10-27, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=1
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/did/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight;
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight;
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight;
	fn delete(c: u32, ) -> Weight;
	fn reclaim_deposit(c: u32, ) -> Weight;
	fn submit_did_call_ed25519_key() -> Weight;
	fn submit_did_call_sr25519_key() -> Weight;
	fn submit_did_call_ecdsa_key() -> Weight;
	fn set_ed25519_authentication_key() -> Weight;
	fn set_sr25519_authentication_key() -> Weight;
	fn set_ecdsa_authentication_key() -> Weight;
	fn set_ed25519_delegation_key() -> Weight;
	fn set_sr25519_delegation_key() -> Weight;
	fn set_ecdsa_delegation_key() -> Weight;
	fn remove_ed25519_delegation_key() -> Weight;
	fn remove_sr25519_delegation_key() -> Weight;
	fn remove_ecdsa_delegation_key() -> Weight;
	fn set_ed25519_attestation_key() -> Weight;
	fn set_sr25519_attestation_key() -> Weight;
	fn set_ecdsa_attestation_key() -> Weight;
	fn remove_ed25519_attestation_key() -> Weight;
	fn remove_sr25519_attestation_key() -> Weight;
	fn remove_ecdsa_attestation_key() -> Weight;
	fn add_ed25519_key_agreement_key() -> Weight;
	fn add_sr25519_key_agreement_key() -> Weight;
	fn add_ecdsa_key_agreement_key() -> Weight;
	fn remove_ed25519_key_agreement_key() -> Weight;
	fn remove_sr25519_key_agreement_key() -> Weight;
	fn remove_ecdsa_key_agreement_key() -> Weight;
	fn add_service_endpoint() -> Weight;
	fn remove_service_endpoint() -> Weight;
	fn signature_verification_sr25519(l: u32, ) -> Weight;
	fn signature_verification_ed25519(l: u32, ) -> Weight;
	fn signature_verification_ecdsa(l: u32, ) -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_ed25519_keys(_n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(436_583_000 as u64)
			// Standard Error: 1_760_000
			.saturating_add(Weight::from_ref_time(9_755_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(354_663_000 as u64)
			// Standard Error: 649_000
			.saturating_add(Weight::from_ref_time(3_972_000 as u64).saturating_mul(n as u64))
			// Standard Error: 243_000
			.saturating_add(Weight::from_ref_time(19_126_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn create_ecdsa_keys(_n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(706_144_000 as u64)
			// Standard Error: 434_000
			.saturating_add(Weight::from_ref_time(10_592_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn delete(c: u32, ) -> Weight {
		Weight::from_ref_time(37_058_000 as u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(2_646_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		Weight::from_ref_time(75_498_000 as u64)
			// Standard Error: 90_000
			.saturating_add(Weight::from_ref_time(1_372_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		Weight::from_ref_time(94_698_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		Weight::from_ref_time(88_897_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		Weight::from_ref_time(250_190_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		Weight::from_ref_time(86_282_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		Weight::from_ref_time(124_704_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		Weight::from_ref_time(121_388_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(113_934_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(99_958_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(121_038_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(106_690_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(110_408_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(103_364_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(114_004_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(102_743_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(99_226_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(106_430_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(101_260_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(102_934_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(106_259_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(109_486_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(100_199_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(99_136_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(114_495_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(74_631_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn add_service_endpoint() -> Weight {
		Weight::from_ref_time(52_909_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn remove_service_endpoint() -> Weight {
		Weight::from_ref_time(61_756_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		Weight::from_ref_time(123_502_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		Weight::from_ref_time(130_465_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		Weight::from_ref_time(286_956_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_ed25519_keys(_n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(436_583_000 as u64)
			// Standard Error: 1_760_000
			.saturating_add(Weight::from_ref_time(9_755_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(354_663_000 as u64)
			// Standard Error: 649_000
			.saturating_add(Weight::from_ref_time(3_972_000 as u64).saturating_mul(n as u64))
			// Standard Error: 243_000
			.saturating_add(Weight::from_ref_time(19_126_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn create_ecdsa_keys(_n: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(706_144_000 as u64)
			// Standard Error: 434_000
			.saturating_add(Weight::from_ref_time(10_592_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn delete(c: u32, ) -> Weight {
		Weight::from_ref_time(37_058_000 as u64)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(2_646_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		Weight::from_ref_time(75_498_000 as u64)
			// Standard Error: 90_000
			.saturating_add(Weight::from_ref_time(1_372_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		Weight::from_ref_time(94_698_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		Weight::from_ref_time(88_897_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		Weight::from_ref_time(250_190_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		Weight::from_ref_time(86_282_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		Weight::from_ref_time(124_704_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		Weight::from_ref_time(121_388_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(113_934_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(99_958_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(121_038_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		Weight::from_ref_time(106_690_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		Weight::from_ref_time(110_408_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		Weight::from_ref_time(103_364_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(114_004_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(102_743_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(99_226_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		Weight::from_ref_time(106_430_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		Weight::from_ref_time(101_260_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		Weight::from_ref_time(102_934_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(106_259_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(109_486_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(100_199_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(99_136_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		Weight::from_ref_time(114_495_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		Weight::from_ref_time(74_631_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn add_service_endpoint() -> Weight {
		Weight::from_ref_time(52_909_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn remove_service_endpoint() -> Weight {
		Weight::from_ref_time(61_756_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		Weight::from_ref_time(123_502_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		Weight::from_ref_time(130_465_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		Weight::from_ref_time(286_956_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
}
