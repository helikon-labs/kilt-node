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

use super::*;

use codec::Encode;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, Zero};
use frame_support::{assert_ok, traits::Currency};
use frame_system::RawOrigin;
use kilt_support::signature::VerifySignature;
use runtime_common::AccountId;
use sp_core::{crypto::KeyTypeId, ecdsa, ed25519, sr25519};
use sp_io::crypto::{ecdsa_generate, ecdsa_sign, ed25519_generate, ed25519_sign, sr25519_generate, sr25519_sign};
use sp_runtime::{traits::IdentifyAccount, MultiSigner, SaturatedConversion};
use sp_std::vec::Vec;

use crate::{
	did_details::{
		DeriveDidCallAuthorizationVerificationKeyRelationship, DidAuthorizedCallOperation, DidDetails, DidPublicKey,
		DidSignature, DidVerificationKey,
	},
	mock_utils::{
		generate_base_did_creation_details, generate_base_did_details, get_key_agreement_keys, get_service_endpoints,
	},
	service_endpoints::DidEndpoint,
};

const DEFAULT_ACCOUNT_ID: &str = "tx_submitter";
const DEFAULT_ACCOUNT_SEED: u32 = 0;
const AUTHENTICATION_KEY_ID: KeyTypeId = KeyTypeId(*b"auth");
const AUTHENTICATION_KEY_ID_ALTERNATIVE: KeyTypeId = KeyTypeId(*b"htua");
const ATTESTATION_KEY_ID: KeyTypeId = KeyTypeId(*b"atte");
const ATTESTATION_KEY_ID_ALTERNATIVE: KeyTypeId = KeyTypeId(*b"etta");
const DELEGATION_KEY_ID: KeyTypeId = KeyTypeId(*b"dele");
const DELEGATION_KEY_ID_ALTERNATIVE: KeyTypeId = KeyTypeId(*b"eled");
const MAX_PAYLOAD_BYTE_LENGTH: u32 = 5 * 1024 * 1024;

fn get_ed25519_public_authentication_key() -> ed25519::Public {
	ed25519_generate(AUTHENTICATION_KEY_ID, None)
}

fn get_sr25519_public_authentication_key() -> sr25519::Public {
	sr25519_generate(AUTHENTICATION_KEY_ID, None)
}

fn get_ecdsa_public_authentication_key() -> ecdsa::Public {
	ecdsa_generate(AUTHENTICATION_KEY_ID, None)
}

// Ecdsa public key is 33 byte long, while Ed25519 and Sr25519 are 32 bytes.
fn get_longest_public_key(key_id: KeyTypeId) -> ecdsa::Public {
	ecdsa_generate(key_id, None)
}

fn make_free_for_did<T: Config>(account: &AccountIdOf<T>) {
	let balance = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::minimum_balance()
		+ <T as Config>::Deposit::get()
		+ <T as Config>::Fee::get();
	<CurrencyOf<T> as Currency<AccountIdOf<T>>>::make_free_balance_be(account, balance);
}

// Must always be dispatched with the DID authentication key
fn generate_base_did_call_operation<T: Config>(
	did: DidIdentifierOf<T>,
	submitter: AccountIdOf<T>,
) -> DidAuthorizedCallOperation<T> {
	let test_call = <T as Config>::Call::get_call_for_did_call_benchmark();

	DidAuthorizedCallOperation {
		did,
		call: test_call,
		tx_counter: 1u64,
		block_number: T::BlockNumber::default(),
		submitter,
	}
}

// Generate details that have the specified authentication key, max key
// agreement keys, and the longest type of key for both delegation and
// attestation.
fn generate_full_did_details<T: Config>(auth_key: DidVerificationKey) -> DidDetails<T>
where
	<T as frame_system::Config>::AccountId: From<runtime_common::AccountId>,
{
	let block_number = T::BlockNumber::zero();
	let mut did_details = generate_base_did_details::<T>(auth_key);
	assert_ok!(did_details.add_key_agreement_keys(
		// MaxTotalKeyAgreementKeys max value assumes no attestation nor delegation key is present
		get_key_agreement_keys::<T>(T::MaxTotalKeyAgreementKeys::get() - 2),
		block_number
	));
	assert_ok!(did_details.update_attestation_key(
		DidVerificationKey::from(get_longest_public_key(ATTESTATION_KEY_ID)),
		block_number
	));
	assert_ok!(did_details.update_delegation_key(
		DidVerificationKey::from(get_longest_public_key(DELEGATION_KEY_ID)),
		block_number
	));

	did_details
}

fn save_service_endpoints<T: Config>(did_subject: &DidIdentifierOf<T>, endpoints: &[DidEndpoint<T>]) {
	for endpoint in endpoints.iter() {
		ServiceEndpoints::<T>::insert(&did_subject, &endpoint.id, endpoint.clone());
	}
	DidEndpointsCount::<T>::insert(&did_subject, endpoints.len().saturated_into::<u32>());
}

benchmarks! {
	where_clause {
		where
		T::DidIdentifier: From<AccountId>,
		<T as frame_system::Config>::Origin: From<RawOrigin<T::DidIdentifier>>,
		<T as frame_system::Config>::AccountId: From<AccountId>,
	}

	/* create extrinsic */
	create_ed25519_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		make_free_for_did::<T>(&submitter);

		let did_public_auth_key = get_ed25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key).into_account().into();

		let did_creation_details = generate_base_did_creation_details::<T>(did_subject.clone(), submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let did_creation_signature = DidSignature::from(ed25519_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_creation_details.encode().as_ref()).expect("Failed to create DID signature from raw ed25519 signature."));
	}: create(origin, did_creation_details, did_creation_signature)
	verify {
		let stored_did = Did::<T>::get(&did_subject).expect("New DID should be stored on chain.");
		let stored_key_agreement_keys_ids = stored_did.key_agreement_keys;

		let expected_authentication_key_id = utils::calculate_key_id::<T>(&DidVerificationKey::from(did_public_auth_key).into());

		assert_eq!(
			stored_did.authentication_key,
			expected_authentication_key_id
		);
		assert_eq!(stored_did.last_tx_counter, 0u64);
	}

	create_sr25519_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		make_free_for_did::<T>(&submitter);

		let did_public_auth_key = get_sr25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key).into_account().into();

		let did_creation_details = generate_base_did_creation_details::<T>(did_subject.clone(), submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let did_creation_signature = DidSignature::from(sr25519_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_creation_details.encode().as_ref()).expect("Failed to create DID signature from raw sr25519 signature."));
	}: create(origin, did_creation_details, did_creation_signature)
	verify {
		let stored_did = Did::<T>::get(&did_subject).expect("New DID should be stored on chain.");
		let stored_key_agreement_keys_ids = stored_did.key_agreement_keys;

		let expected_authentication_key_id = utils::calculate_key_id::<T>(&DidVerificationKey::from(did_public_auth_key).into());

		assert_eq!(
			stored_did.authentication_key,
			expected_authentication_key_id
		);
		assert_eq!(stored_did.last_tx_counter, 0u64);
	}

	create_ecdsa_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		make_free_for_did::<T>(&submitter);

		let did_public_auth_key = get_ecdsa_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key.clone()).into_account().into();

		let did_creation_details = generate_base_did_creation_details::<T>(did_subject.clone(), submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let did_creation_signature = DidSignature::from(ecdsa_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_creation_details.encode().as_ref()).expect("Failed to create DID signature from raw ecdsa signature."));
	}: create(origin, did_creation_details, did_creation_signature)
	verify {
		let stored_did = Did::<T>::get(&did_subject).expect("New DID should be stored on chain.");
		let stored_key_agreement_keys_ids = stored_did.key_agreement_keys;

		let expected_authentication_key_id = utils::calculate_key_id::<T>(&DidVerificationKey::from(did_public_auth_key).into());

		assert_eq!(
			stored_did.authentication_key,
			expected_authentication_key_id
		);
		assert_eq!(stored_did.last_tx_counter, 0u64);
	}

	delete {
		let c in 1 .. T::MaxNumberOfServicesPerDid::get();

		let did_public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(did_public_auth_key));

		let service_endpoints = get_service_endpoints::<T>(
			c,
			T::MaxServiceIdLength::get(),
			T::MaxNumberOfTypesPerService::get(),
			T::MaxServiceTypeLength::get(),
			T::MaxNumberOfUrlsPerService::get(),
			T::MaxServiceUrlLength::get(),
		);

		Did::<T>::insert(&did_subject, did_details);
		save_service_endpoints(&did_subject, &service_endpoints);
		let origin = RawOrigin::Signed(did_subject.clone());
	}: _(origin, c)
	verify {
		assert!(
			Did::<T>::get(&did_subject).is_none()
		);
		assert_eq!(
			DidEndpointsCount::<T>::get(&did_subject), 0
		);
		assert_eq!(
			ServiceEndpoints::<T>::iter_prefix(&did_subject).count(),
			0
		);
	}

	reclaim_deposit {
		let c in 1 .. T::MaxNumberOfServicesPerDid::get();

		let did_public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(did_public_auth_key));

		let service_endpoints = get_service_endpoints::<T>(
			c,
			T::MaxServiceIdLength::get(),
			T::MaxNumberOfTypesPerService::get(),
			T::MaxServiceTypeLength::get(),
			T::MaxNumberOfUrlsPerService::get(),
			T::MaxServiceUrlLength::get(),
		);

		Did::<T>::insert(&did_subject, did_details.clone());
		save_service_endpoints(&did_subject, &service_endpoints);
		let origin = RawOrigin::Signed(did_details.deposit.owner);
	}: _(origin, did_subject.clone(), c)
	verify {
		assert!(
			Did::<T>::get(&did_subject).is_none()
		);
		assert_eq!(
			DidEndpointsCount::<T>::get(&did_subject), 0
		);
		assert_eq!(
			ServiceEndpoints::<T>::iter_prefix(&did_subject).count(),
			0
		);
	}

	/* submit_did_call extrinsic */
	submit_did_call_ed25519_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		let did_public_auth_key = get_ed25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(did_public_auth_key));
		Did::<T>::insert(&did_subject, did_details);

		let did_call_op = generate_base_did_call_operation::<T>(did_subject, submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let op = Box::new(did_call_op.clone());
		let did_call_signature = DidSignature::from(ed25519_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_call_op.encode().as_ref()).expect("Failed to create DID signature from raw ed25519 signature."));
	}: submit_did_call(origin, op, did_call_signature)

	submit_did_call_sr25519_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		let did_public_auth_key = get_sr25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(did_public_auth_key));
		Did::<T>::insert(&did_subject, did_details);

		let did_call_op = generate_base_did_call_operation::<T>(did_subject, submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let op = Box::new(did_call_op.clone());
		let did_call_signature = DidSignature::from(sr25519_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_call_op.encode().as_ref()).expect("Failed to create DID signature from raw sr25519 signature."));
	}: submit_did_call(origin, op, did_call_signature)

	submit_did_call_ecdsa_key {
		let submitter: AccountIdOf<T> = account(DEFAULT_ACCOUNT_ID, 0, DEFAULT_ACCOUNT_SEED);

		let did_public_auth_key = get_ecdsa_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(did_public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(did_public_auth_key.clone()));
		Did::<T>::insert(&did_subject, did_details);

		let did_call_op = generate_base_did_call_operation::<T>(did_subject, submitter.clone());

		let origin = RawOrigin::Signed(submitter);
		let op = Box::new(did_call_op.clone());
		let did_call_signature = DidSignature::from(ecdsa_sign(AUTHENTICATION_KEY_ID, &did_public_auth_key, did_call_op.encode().as_ref()).expect("Failed to create DID signature from raw ecdsa signature."));
	}: submit_did_call(origin, op, did_call_signature)

	set_authentication_key {
		let old_did_public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(old_did_public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(old_did_public_auth_key));

		Did::<T>::insert(&did_subject, did_details);

		let origin = RawOrigin::Signed(did_subject.clone());
		let new_did_public_auth_key = DidVerificationKey::from(get_longest_public_key(AUTHENTICATION_KEY_ID_ALTERNATIVE));
		let new_did_public_auth_key_clone = new_did_public_auth_key.clone();
	}: _(origin, new_did_public_auth_key_clone)
	verify {
		let auth_key_id = utils::calculate_key_id::<T>(&DidPublicKey::from(new_did_public_auth_key));
		assert_eq!(Did::<T>::get(&did_subject).unwrap().authentication_key, auth_key_id);
	}

	set_delegation_key {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));

		Did::<T>::insert(&did_subject, did_details);

		let origin = RawOrigin::Signed(did_subject.clone());
		let new_did_public_del_key = DidVerificationKey::from(get_longest_public_key(DELEGATION_KEY_ID_ALTERNATIVE));
		let new_did_public_del_key_clone = new_did_public_del_key.clone();
	}: _(origin, new_did_public_del_key_clone)
	verify {
		let new_delegation_key_id = utils::calculate_key_id::<T>(&DidPublicKey::from(new_did_public_del_key));
		assert_eq!(Did::<T>::get(&did_subject).unwrap().delegation_key, Some(new_delegation_key_id));
	}

	remove_delegation_key {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));
		let old_delegation_key_id = did_details.delegation_key.unwrap();

		Did::<T>::insert(&did_subject, did_details);
		let origin = RawOrigin::Signed(did_subject.clone());
	}: _(origin)
	verify {
		let did_details = Did::<T>::get(&did_subject).unwrap();
		assert!(did_details.delegation_key.is_none());
		assert!(!did_details.public_keys.contains_key(&old_delegation_key_id));
	}

	set_attestation_key {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));

		Did::<T>::insert(&did_subject, did_details);

		let origin = RawOrigin::Signed(did_subject.clone());
		let new_did_public_att_key = DidVerificationKey::from(get_longest_public_key(ATTESTATION_KEY_ID_ALTERNATIVE));
		let new_did_public_att_key_clone = new_did_public_att_key.clone();
	}: _(origin, new_did_public_att_key_clone)
	verify {
		let new_attestation_key_id = utils::calculate_key_id::<T>(&DidPublicKey::from(new_did_public_att_key));
		assert_eq!(Did::<T>::get(&did_subject).unwrap().attestation_key, Some(new_attestation_key_id));
	}

	remove_attestation_key {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));
		let old_attestation_key_id = did_details.attestation_key.unwrap();

		Did::<T>::insert(&did_subject, did_details);
		let origin = RawOrigin::Signed(did_subject.clone());
	}: _(origin)
	verify {
		let did_details = Did::<T>::get(&did_subject).unwrap();
		assert!(did_details.attestation_key.is_none());
		assert!(!did_details.public_keys.contains_key(&old_attestation_key_id));
	}

	add_key_agreement_key {
		let block_number = T::BlockNumber::zero();
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();
		let mut key_agreement_keys = get_key_agreement_keys::<T>(T::MaxTotalKeyAgreementKeys::get());

		// remove first entry
		let new_key_agreement_key = *key_agreement_keys.clone().into_inner().iter().next().unwrap();
		assert!(key_agreement_keys.remove(&new_key_agreement_key));

		// fill up public keys to its max minus one size (due to removal of new key_agreement_key)
		// no attestation or delegation key is stored, so # of key agreement keys = max # of public keys - 1 (authentication key)
		let mut did_details = generate_base_did_details::<T>(DidVerificationKey::from(public_auth_key));
		assert_ok!(did_details.add_key_agreement_keys(key_agreement_keys, block_number));

		Did::<T>::insert(&did_subject, did_details);
		let origin = RawOrigin::Signed(did_subject.clone());
	}: _(origin, new_key_agreement_key)
	verify {
		let new_key_agreement_key_id = utils::calculate_key_id::<T>(&DidPublicKey::from(new_key_agreement_key));
		assert!(Did::<T>::get(&did_subject).unwrap().key_agreement_keys.contains(&new_key_agreement_key_id));
	}

	remove_key_agreement_key {
		let block_number = T::BlockNumber::zero();
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();
		let key_agreement_keys = get_key_agreement_keys::<T>(T::MaxTotalKeyAgreementKeys::get());

		// get first entry
		let key_agreement_key = *key_agreement_keys.clone().into_inner().iter().next().unwrap();
		let key_agreement_key_id = utils::calculate_key_id::<T>(&DidPublicKey::from(key_agreement_key));

		// fill up public keys to its max minus one size (due to removal of new key_agreement_key)
		// no attestation or delegation key is stored, so # of key agreement keys = max # of public keys - 1 (authentication key)
		let mut did_details = generate_base_did_details::<T>(DidVerificationKey::from(public_auth_key));
		assert_ok!(did_details.add_key_agreement_keys(key_agreement_keys, block_number));

		Did::<T>::insert(&did_subject, did_details);
		let origin = RawOrigin::Signed(did_subject.clone());
	}: _(origin, key_agreement_key_id)
	verify {
		assert!(!Did::<T>::get(&did_subject).unwrap().key_agreement_keys.contains(&key_agreement_key_id));
	}

	add_service_endpoint {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();
		// Max allowed - 1.
		let old_service_endpoints = get_service_endpoints::<T>(
			T::MaxNumberOfServicesPerDid::get() - 1,
			T::MaxServiceIdLength::get(),
			T::MaxNumberOfTypesPerService::get(),
			T::MaxServiceTypeLength::get(),
			T::MaxNumberOfUrlsPerService::get(),
			T::MaxServiceUrlLength::get(),
		);
		// New endpoint with max length and count for all the properties.
		let mut new_service_endpoint = get_service_endpoints::<T>(
			1,
			T::MaxServiceIdLength::get(),
			T::MaxNumberOfTypesPerService::get(),
			T::MaxServiceTypeLength::get(),
			T::MaxNumberOfUrlsPerService::get(),
			T::MaxServiceUrlLength::get(),
		)[0].clone();
		// Changing from the default ID otherwise it would be the same as the one first one in `old_service_endpoints`.
		new_service_endpoint.id = b"new_id".to_vec().try_into().unwrap();

		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));
		Did::<T>::insert(&did_subject, did_details);
		save_service_endpoints(&did_subject, &old_service_endpoints);
		let origin = RawOrigin::Signed(did_subject.clone());
		let new_service_endpoint_clone = new_service_endpoint.clone();
	}: _(origin, new_service_endpoint_clone)
	verify {
		assert_eq!(
			ServiceEndpoints::<T>::get(&did_subject, &new_service_endpoint.id),
			Some(new_service_endpoint)
		);
		assert_eq!(
			DidEndpointsCount::<T>::get(&did_subject),
			T::MaxNumberOfServicesPerDid::get()
		);
		assert_eq!(
			ServiceEndpoints::<T>::iter_prefix(&did_subject).count(),
			T::MaxNumberOfServicesPerDid::get().saturated_into::<usize>()
		);
	}

	remove_service_endpoint {
		let public_auth_key = get_longest_public_key(AUTHENTICATION_KEY_ID);
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();
		// All set to max.
		let old_service_endpoints = get_service_endpoints::<T>(
			T::MaxNumberOfServicesPerDid::get(),
			T::MaxServiceIdLength::get(),
			T::MaxNumberOfTypesPerService::get(),
			T::MaxServiceTypeLength::get(),
			T::MaxNumberOfUrlsPerService::get(),
			T::MaxServiceUrlLength::get(),
		);
		let endpoint_id = old_service_endpoints[0].id.clone();

		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));
		Did::<T>::insert(&did_subject, did_details);
		save_service_endpoints(&did_subject, &old_service_endpoints);
		let origin = RawOrigin::Signed(did_subject.clone());
		let endpoint_id_clone = endpoint_id.clone();
	}: _(origin, endpoint_id_clone)
	verify {
		assert!(
			ServiceEndpoints::<T>::get(&did_subject, &endpoint_id).is_none()
		);
		assert_eq!(
			DidEndpointsCount::<T>::get(&did_subject),
			T::MaxNumberOfServicesPerDid::get() - 1
		);
		assert_eq!(
			ServiceEndpoints::<T>::iter_prefix(&did_subject).count(),
			T::MaxNumberOfServicesPerDid::get().saturated_into::<usize>() - 1
		);
	}

	signature_verification_sr25519 {
		let l in 1 .. MAX_PAYLOAD_BYTE_LENGTH;

		let payload: Vec<u8> = (0u8..u8::MAX).cycle().take(l.try_into().unwrap()).collect();
		let block_number = T::BlockNumber::zero();

		let public_auth_key = get_sr25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));

		Did::<T>::insert(&did_subject, did_details);
		let signature = sr25519_sign(AUTHENTICATION_KEY_ID, &public_auth_key, &payload).expect("Failed to create DID signature from raw sr25519 signature.");
		let did_signature = DidSignature::Sr25519(signature);
	}: {
		DidSignatureVerify::<T>::verify(&did_subject, &payload, &did_signature).expect("should verify");
	}
	verify {}
	signature_verification_ed25519 {
		let l in 1 .. MAX_PAYLOAD_BYTE_LENGTH;

		let payload: Vec<u8> = (0u8..u8::MAX).cycle().take(l.try_into().unwrap()).collect();
		let block_number = T::BlockNumber::zero();

		let public_auth_key = get_ed25519_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key));

		Did::<T>::insert(&did_subject, did_details);
		let signature = ed25519_sign(AUTHENTICATION_KEY_ID, &public_auth_key, &payload).expect("Failed to create DID signature from raw ed25519 signature.");
		let did_signature = DidSignature::Ed25519(signature);
	}: {
		DidSignatureVerify::<T>::verify(&did_subject, &payload, &did_signature).expect("should verify");
	}
	verify {}
	signature_verification_ecdsa {
		let l in 1 .. MAX_PAYLOAD_BYTE_LENGTH;

		let payload: Vec<u8> = (0u8..u8::MAX).cycle().take(l.try_into().unwrap()).collect();
		let block_number = T::BlockNumber::zero();

		let public_auth_key = get_ecdsa_public_authentication_key();
		let did_subject: DidIdentifierOf<T> = MultiSigner::from(public_auth_key.clone()).into_account().into();

		// Fill up DID details to account for maximum decoding time.
		let did_details = generate_full_did_details::<T>(DidVerificationKey::from(public_auth_key.clone()));

		Did::<T>::insert(&did_subject, did_details);
		let signature = ecdsa_sign(AUTHENTICATION_KEY_ID, &public_auth_key, &payload).expect("Failed to create DID signature from raw ecdsa signature.");
		let did_signature = DidSignature::Ecdsa(signature);
	}: {
		DidSignatureVerify::<T>::verify(&did_subject, &payload, &did_signature).expect("should verify");
	}
	verify {}
}

impl_benchmark_test_suite! {
	Pallet,
	crate::mock::ExtBuilder::default().build_with_keystore(),
	crate::mock::Test
}
