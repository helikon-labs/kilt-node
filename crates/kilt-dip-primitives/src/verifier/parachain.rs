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

use did::KeyIdOf;
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_did_lookup::linkable_account::LinkableAccountId;
use pallet_dip_consumer::{traits::IdentityProofVerifier, RuntimeCallOf};
use pallet_dip_provider::IdentityCommitmentOf;
use pallet_web3_names::Web3NameOf;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::marker::PhantomData;

use crate::{
	merkle::RevealedDidKey,
	traits::{DipCallOriginFilter, GetWithArg, GetWithoutArg, Incrementable},
	utils::OutputOf,
	BoundedBlindedValue,
};

/// A KILT-specific DIP identity proof for a sibling consumer that supports
/// versioning.
///
/// For more info, refer to the version-specific proofs.
#[derive(Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, Clone)]
pub enum VersionedDipParachainStateProof<
	RelayBlockHeight,
	ProviderKeyId,
	ProviderAccountId,
	ProviderWeb3Name,
	ProviderBlockNumber,
	ProviderLinkedAccountId,
	LocalBlockNumber,
> {
	V0(
		v0::ParachainDipStateProof<
			RelayBlockHeight,
			ProviderKeyId,
			ProviderAccountId,
			ProviderWeb3Name,
			ProviderBlockNumber,
			ProviderLinkedAccountId,
			LocalBlockNumber,
		>,
	),
}

pub enum DipParachainStateProofVerifierError<
	ParachainHeadMerkleProofVerificationError,
	IdentityCommitmentMerkleProofVerificationError,
	DipProofVerificationError,
	DidSignatureVerificationError,
> {
	UnsupportedVersion,
	ParachainHeadMerkleProof(ParachainHeadMerkleProofVerificationError),
	IdentityCommitmentMerkleProof(IdentityCommitmentMerkleProofVerificationError),
	DipProof(DipProofVerificationError),
	DidSignature(DidSignatureVerificationError),
}

impl<
		ParachainHeadMerkleProofVerificationError,
		IdentityCommitmentMerkleProofVerificationError,
		DipProofVerificationError,
		DidSignatureVerificationError,
	>
	From<
		DipParachainStateProofVerifierError<
			ParachainHeadMerkleProofVerificationError,
			IdentityCommitmentMerkleProofVerificationError,
			DipProofVerificationError,
			DidSignatureVerificationError,
		>,
	> for u16
where
	ParachainHeadMerkleProofVerificationError: Into<u8>,
	IdentityCommitmentMerkleProofVerificationError: Into<u8>,
	DipProofVerificationError: Into<u8>,
	DidSignatureVerificationError: Into<u8>,
{
	fn from(
		value: DipParachainStateProofVerifierError<
			ParachainHeadMerkleProofVerificationError,
			IdentityCommitmentMerkleProofVerificationError,
			DipProofVerificationError,
			DidSignatureVerificationError,
		>,
	) -> Self {
		match value {
			DipParachainStateProofVerifierError::UnsupportedVersion => 1,
			DipParachainStateProofVerifierError::ParachainHeadMerkleProof(error) => {
				u8::MAX as u16 + error.into() as u16
			}
			DipParachainStateProofVerifierError::IdentityCommitmentMerkleProof(error) => {
				u8::MAX as u16 * 2 + error.into() as u16
			}
			DipParachainStateProofVerifierError::DipProof(error) => u8::MAX as u16 * 3 + error.into() as u16,
			DipParachainStateProofVerifierError::DidSignature(error) => u8::MAX as u16 * 4 + error.into() as u16,
		}
	}
}

/// Proof verifier configured given a specific KILT runtime implementation.
///
/// It is a specialization of the
/// [`GenericVersionedParachainVerifier`] type, with
/// configurations derived from the provided KILT runtime.
///
/// The generic types
/// indicate the following:
/// * `KiltRuntime`: A KILT runtime definition.
/// * `KiltParachainId`: The ID of the specific KILT parachain instance.
/// * `RelayChainInfo`: The type providing information about the relaychain.
/// * `KiltDipMerkleHasher`: The hashing algorithm used by the KILT parachain
///   for the generation of the DIP identity commitment.
/// * `LocalDidCallVerifier`: Logic to map `RuntimeCall`s to a specific DID key
///   relationship. This information is used once the Merkle proof is verified,
///   to filter only the revealed keys that match the provided relationship.
/// * `MAX_REVEALED_KEYS_COUNT`: **OPTIONAL** Max number of DID keys that the
///   verifier will accept revealed as part of the DIP identity proof. It
///   defaults to **10**.
/// * `MAX_REVEALED_ACCOUNTS_COUNT`: **OPTIONAL** Max number of linked accounts
///   that the verifier will accept revealed as part of the DIP identity proof.
///   It defaults to **10**.
/// * `MAX_DID_SIGNATURE_DURATION`: **OPTIONAL** Max number of blocks a
///   cross-chain DID signature is considered fresh. It defaults to **50**.
///
/// It specializes the [`GenericVersionedParachainVerifier`]
/// type by using the following types for its generics:
/// * `RelayChainInfo`: The provided `RelayChainInfo`.
/// * `ChildProviderParachainId`: The provided `KiltParachainId`.
/// * `ChildProviderStateInfo`: The
///   [`ProviderParachainStateInfoViaProviderPallet`] type configured with the
///   provided `KiltRuntime`.
/// * `ProviderDipMerkleHasher`: The provided `KiltDipMerkleHasher`.
/// * `ProviderDidKeyId`: The [`KeyIdOf`] type configured with the provided
///   `KiltRuntime`.
/// * `ProviderAccountId`: The `KiltRuntime::AccountId` type.
/// * `ProviderWeb3Name`: The `KiltRuntime::Web3Name` type.
/// * `ProviderLinkedAccountId`: The [`LinkableAccountId`] type.
/// * `MAX_REVEALED_KEYS_COUNT`: The provided `MAX_REVEALED_KEYS_COUNT`.
/// * `MAX_REVEALED_ACCOUNTS_COUNT`: The provided `MAX_REVEALED_ACCOUNTS_COUNT`.
/// * `LocalContextProvider`: The [`FrameSystemDidSignatureContext`] type
///   configured with the provided `KiltRuntime` and
///   `MAX_DID_SIGNATURE_DURATION`.
/// * `LocalDidCallVerifier`: The provided `LocalDidCallVerifier`.
pub struct KiltVersionedParachainVerifier<
	RelaychainRuntime,
	RelaychainStateRoot,
	const PROVIDER_PARA_ID: u32,
	ProviderRuntime,
	SignedExtra,
	DidCallVerifier,
	const MAX_LEAVES_REVEALED: usize,
	const SIGNATURE_VALIDITY: u32,
>(
	PhantomData<(
		RelaychainRuntime,
		RelaychainStateRoot,
		ProviderRuntime,
		SignedExtra,
		DidCallVerifier,
	)>,
);

impl<
		ConsumerRuntime,
		RelaychainRuntime,
		RelaychainStateRoot,
		const PROVIDER_PARA_ID: u32,
		KiltRuntime,
		SignedExtra,
		DidCallVerifier,
		const MAX_LEAVES_REVEALED: usize,
		const SIGNATURE_VALIDITY: u32,
	> IdentityProofVerifier<ConsumerRuntime>
	for KiltVersionedParachainVerifier<
		RelaychainRuntime,
		RelaychainStateRoot,
		PROVIDER_PARA_ID,
		KiltRuntime,
		SignedExtra,
		DidCallVerifier,
		MAX_LEAVES_REVEALED,
		SIGNATURE_VALIDITY,
	> where
	ConsumerRuntime: pallet_dip_consumer::Config<Identifier = KiltRuntime::Identifier> + pallet_relay_store::Config,
	ConsumerRuntime::LocalIdentityInfo: Incrementable + Default,
	RelaychainRuntime: frame_system::Config,
	RelaychainStateRoot:
		GetWithArg<BlockNumberFor<RelaychainRuntime>, Result = Option<OutputOf<RelaychainRuntime::Hashing>>>,
	KiltRuntime: pallet_dip_provider::Config + did::Config + pallet_web3_names::Config + pallet_did_lookup::Config,
	IdentityCommitmentOf<KiltRuntime>: Into<KiltRuntime::Hash>,
	SignedExtra: GetWithoutArg,
	SignedExtra::Result: Encode,
	DidCallVerifier: DipCallOriginFilter<
		RuntimeCallOf<ConsumerRuntime>,
		OriginInfo = RevealedDidKey<KeyIdOf<KiltRuntime>, BlockNumberFor<KiltRuntime>, KiltRuntime::AccountId>,
	>,
{
	type Error = u16;
	type Proof = VersionedDipParachainStateProof<
		BlockNumberFor<RelaychainRuntime>,
		KeyIdOf<KiltRuntime>,
		KiltRuntime::AccountId,
		BlockNumberFor<KiltRuntime>,
		Web3NameOf<KiltRuntime>,
		LinkableAccountId,
		BlockNumberFor<ConsumerRuntime>,
	>;
	type VerificationResult = ();

	fn verify_proof_for_call_against_details(
		call: &RuntimeCallOf<ConsumerRuntime>,
		subject: &ConsumerRuntime::Identifier,
		submitter: &ConsumerRuntime::AccountId,
		identity_details: &mut Option<ConsumerRuntime::LocalIdentityInfo>,
		proof: Self::Proof,
	) -> Result<Self::VerificationResult, Self::Error> {
		match proof {
			VersionedDipParachainStateProof::V0(v0_proof) => <v0::ParachainVerifier<
				RelaychainRuntime,
				RelaychainStateRoot,
				PROVIDER_PARA_ID,
				KiltRuntime,
				SignedExtra,
				DidCallVerifier,
				MAX_LEAVES_REVEALED,
				SIGNATURE_VALIDITY,
			> as IdentityProofVerifier<ConsumerRuntime>>::verify_proof_for_call_against_details(
				call,
				subject,
				submitter,
				identity_details,
				v0_proof,
			),
		}
	}
}

pub mod latest {
	pub use super::v0::ParachainDipStateProof;
}

pub mod v0 {
	use super::*;

	use frame_system::pallet_prelude::HeaderFor;
	use pallet_web3_names::Web3NameOf;
	use sp_runtime::traits::{Header, Zero};

	use crate::{
		merkle::RevealedDidKey,
		state_proofs::verify_storage_value_proof,
		verifier::common::{
			calculate_dip_identity_commitment_storage_key_for_runtime, calculate_parachain_head_storage_key,
			v0::{DipMerkleProofAndDidSignature, ParachainRootStateProof},
		},
	};

	/// The expected format of a cross-chain DIP identity proof when the
	/// identity information is bridged from a provider that is a sibling
	/// of the chain where the information is consumed (i.e., consumer
	/// chain).
	#[derive(Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, Clone)]
	pub struct ParachainDipStateProof<
		RelayBlockHeight,
		ProviderKeyId,
		ProviderAccountId,
		ProviderBlockNumber,
		ProviderWeb3Name,
		ProviderLinkedAccountId,
		LocalBlockNumber,
	> {
		pub(crate) para_state_root: ParachainRootStateProof<RelayBlockHeight>,
		pub(crate) dip_identity_commitment: BoundedBlindedValue<u8>,
		pub(crate) did: DipMerkleProofAndDidSignature<
			ProviderKeyId,
			ProviderAccountId,
			ProviderBlockNumber,
			ProviderWeb3Name,
			ProviderLinkedAccountId,
			LocalBlockNumber,
		>,
	}

	pub struct ParachainVerifier<
		RelaychainRuntime,
		RelaychainStateRoot,
		const PROVIDER_PARA_ID: u32,
		ProviderRuntime,
		SignedExtra,
		DidCallVerifier,
		const MAX_LEAVES_REVEALED: usize,
		const SIGNATURE_VALIDITY: u32,
	>(
		PhantomData<(
			RelaychainRuntime,
			RelaychainStateRoot,
			ProviderRuntime,
			SignedExtra,
			DidCallVerifier,
		)>,
	);

	impl<
			ConsumerRuntime,
			RelaychainRuntime,
			RelaychainStateRoot,
			const PROVIDER_PARA_ID: u32,
			ProviderRuntime,
			SignedExtra,
			DidCallVerifier,
			const MAX_LEAVES_REVEALED: usize,
			const SIGNATURE_VALIDITY: u32,
		> IdentityProofVerifier<ConsumerRuntime>
		for ParachainVerifier<
			RelaychainRuntime,
			RelaychainStateRoot,
			PROVIDER_PARA_ID,
			ProviderRuntime,
			SignedExtra,
			DidCallVerifier,
			MAX_LEAVES_REVEALED,
			SIGNATURE_VALIDITY,
		> where
		ConsumerRuntime:
			pallet_dip_consumer::Config<Identifier = ProviderRuntime::Identifier> + pallet_relay_store::Config,
		ConsumerRuntime::LocalIdentityInfo: Incrementable + Default,
		RelaychainRuntime: frame_system::Config,
		RelaychainStateRoot:
			GetWithArg<BlockNumberFor<RelaychainRuntime>, Result = Option<OutputOf<RelaychainRuntime::Hashing>>>,
		ProviderRuntime:
			pallet_dip_provider::Config + did::Config + pallet_web3_names::Config + pallet_did_lookup::Config,
		IdentityCommitmentOf<ProviderRuntime>: Into<ProviderRuntime::Hash>,
		SignedExtra: GetWithoutArg,
		SignedExtra::Result: Encode,
		DidCallVerifier: DipCallOriginFilter<
			RuntimeCallOf<ConsumerRuntime>,
			OriginInfo = RevealedDidKey<
				KeyIdOf<ProviderRuntime>,
				BlockNumberFor<ProviderRuntime>,
				ProviderRuntime::AccountId,
			>,
		>,
	{
		type Error = u16;
		type Proof = ParachainDipStateProof<
			BlockNumberFor<RelaychainRuntime>,
			KeyIdOf<ProviderRuntime>,
			ProviderRuntime::AccountId,
			BlockNumberFor<ProviderRuntime>,
			Web3NameOf<ProviderRuntime>,
			LinkableAccountId,
			BlockNumberFor<ConsumerRuntime>,
		>;

		type VerificationResult = ();

		fn verify_proof_for_call_against_details(
			call: &RuntimeCallOf<ConsumerRuntime>,
			subject: &<ConsumerRuntime as pallet_dip_consumer::Config>::Identifier,
			submitter: &<ConsumerRuntime>::AccountId,
			identity_details: &mut Option<<ConsumerRuntime as pallet_dip_consumer::Config>::LocalIdentityInfo>,
			proof: Self::Proof,
		) -> Result<Self::VerificationResult, Self::Error> {
			// 1. Verify parachain state is finalized by relay chain and fresh.
			let provider_head_storage_key = calculate_parachain_head_storage_key(PROVIDER_PARA_ID);
			let relaychain_root_at_proof_block = RelaychainStateRoot::get(&proof.para_state_root.relay_block_height)
				.expect("No relaychain block found for given height.");
			let provider_parachain_header =
				verify_storage_value_proof::<_, RelaychainRuntime::Hashing, HeaderFor<ProviderRuntime>>(
					&provider_head_storage_key,
					relaychain_root_at_proof_block,
					proof.para_state_root.proof,
				)
				.expect("Proof verification failed");

			// 2. Verify commitment is included in provider parachain.
			let dip_commitment_storage_key =
				calculate_dip_identity_commitment_storage_key_for_runtime::<ProviderRuntime>(subject, 0);
			let dip_commitment =
				verify_storage_value_proof::<_, ProviderRuntime::Hashing, IdentityCommitmentOf<ProviderRuntime>>(
					&dip_commitment_storage_key,
					*provider_parachain_header.state_root(),
					proof.dip_identity_commitment,
				)
				.expect("Failed to verify DIP Merkle proof.");

			let did_proof = proof.did;

			// 3. Verify DIP Merkle proof.
			let verified_proof = did_proof
				.verify_merkle_proof_against_commitment::<ProviderRuntime::Hashing>(
					&dip_commitment.into(),
					MAX_LEAVES_REVEALED,
				)
				.unwrap_or_else(|_| panic!("Failed to verify DIP Merkle proof"));

			// 4. Verify call is signed by one of the DID keys revealed at step 3.
			let current_block_number = frame_system::Pallet::<ConsumerRuntime>::block_number();
			let consumer_genesis_hash =
				frame_system::Pallet::<ConsumerRuntime>::block_hash(BlockNumberFor::<ConsumerRuntime>::zero());
			let signed_extra = SignedExtra::get();
			let encoded_payload = (call, &identity_details, submitter, consumer_genesis_hash, signed_extra).encode();
			let signing_key = verified_proof
				.extract_signing_key_for_payload(&encoded_payload[..], current_block_number)
				.unwrap_or_else(|_| panic!("Signature is not fresh."));

			let Some(signing_key) = signing_key else {
				panic!("No key found.");
			};

			// Increment the local details
			if let Some(details) = identity_details {
				details.increment();
			} else {
				*identity_details = Some(Default::default());
			};

			DidCallVerifier::check_call_origin_info(call, signing_key)
				.unwrap_or_else(|_| panic!("Failed to verify DID call info."));

			Ok(())
		}
	}
}
