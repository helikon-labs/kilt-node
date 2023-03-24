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

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};

use cumulus_pallet_parachain_system::{
	register_validate_block, ParachainSetCode, RelayChainStateProof, RelayNumberStrictlyIncreases,
};
use cumulus_primitives_core::CollationInfo;
use cumulus_primitives_timestamp::InherentDataProvider;
use did::{DidRawOrigin, EnsureDidOrigin, KeyIdOf};
use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{ConstU32, ConstU64, ConstU8, Everything},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
		IdentityFee, Weight,
	},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	ChainContext, EnsureRoot,
};
use pallet_balances::AccountData;
use pallet_collator_selection::IdentityCollator;
use pallet_dip_sender::traits::IdentityProvider;
use pallet_session::{FindAccountFromAuthorIndex, PeriodicSessions};
use pallet_transaction_payment::{CurrencyAdapter, FeeDetails, RuntimeDispatchInfo};
use runtime_common::dip::sender::{CompleteMerkleProof, DidMerkleProof, DidMerkleRootGenerator};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::SlotDuration;
use sp_core::{crypto::KeyTypeId, ConstU128, ConstU16, OpaqueMetadata};
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, OpaqueKeys},
	transaction_validity::{TransactionSource, TransactionValidity},
	AccountId32, ApplyExtrinsicResult, MultiSignature, OpaqueExtrinsic,
};
use sp_std::{collections::btree_set::BTreeSet, prelude::*, time::Duration};
use sp_version::RuntimeVersion;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

mod dip;
mod xcm_config;
pub use crate::{dip::*, xcm_config::*};

pub type AccountId = AccountId32;
pub type Address = MultiAddress<AccountId, ()>;
pub type Balance = u128;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type BlockNumber = u32;
pub type DidIdentifier = AccountId;
pub type Hash = sp_core::H256;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Index = u32;
pub type Signature = MultiSignature;

pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
pub type Executive = frame_executive::Executive<Runtime, Block, ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;
pub type NodeBlock = generic::Block<Header, OpaqueExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

pub const MILLISECS_PER_BLOCK: u64 = 12000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;

pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = UNIT / 1_000;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = NodeBlock,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// System
		System: frame_system = 0,
		ParachainSystem: cumulus_pallet_parachain_system = 1,
		Timestamp: pallet_timestamp = 2,
		ParachainInfo: parachain_info = 3,
		Sudo: pallet_sudo = 4,

		// Money
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,

		// Collators
		Authorship: pallet_authorship = 20,
		CollatorSelection: pallet_collator_selection = 21,
		Session: pallet_session = 22,
		Aura: pallet_aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// XCM
		XcmpQueue: cumulus_pallet_xcmp_queue = 30,
		DmpQueue: cumulus_pallet_dmp_queue = 31,
		PolkadotXcm: pallet_xcm = 32,
		CumulusXcm: cumulus_pallet_xcm = 33,

		// DID
		Did: did = 40,

		// DIP
		DipSender: pallet_dip_sender = 50,
	}
);

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("dip-sender-runtime-template"),
	impl_name: create_runtime_str!("dip-sender-runtime-template"),
	authoring_version: 1,
	spec_version: 11100,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(block: &Block, relay_state_proof: &RelayChainStateProof) -> CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			InherentDataProvider::from_relay_chain_slot_and_duration(relay_chain_slot, Duration::from_secs(6))
				.create_inherent_data()
				.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}

pub const SS58_PREFIX: u16 = 100;
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
	BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
	.base_block(BlockExecutionWeight::get())
	.for_class(DispatchClass::all(), |weights| {
		weights.base_extrinsic = ExtrinsicBaseWeight::get();
	})
	.for_class(DispatchClass::Normal, |weights| {
		weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
	})
	.for_class(DispatchClass::Operational, |weights| {
		weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
		weights.reserved = Some(
			MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
		);
	})
	.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
	.build_or_panic();
}

impl frame_system::Config for Runtime {
	type AccountData = AccountData<Balance>;
	type AccountId = AccountId;
	type BaseCallFilter = Everything;
	type BlockHashCount = ConstU32<256>;
	type BlockLength = RuntimeBlockLength;
	type BlockNumber = BlockNumber;
	type BlockWeights = RuntimeBlockWeights;
	type DbWeight = RocksDbWeight;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Index = Index;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type MaxConsumers = ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ParachainSetCode<Self>;
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ConstU16<SS58_PREFIX>;
	type SystemWeightInfo = ();
	type Version = Version;
}

parameter_types! {
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
	type DmpMessageHandler = DmpQueue;
	type OnSystemEvent = ();
	type OutboundXcmpMessageSource = XcmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type RuntimeEvent = RuntimeEvent;
	type SelfParaId = ParachainInfo;
	type XcmpMessageHandler = XcmpQueue;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = ConstU64<{ MILLISECS_PER_BLOCK / 2 }>;
	type Moment = u64;
	type OnTimestampSet = Aura;
	type WeightInfo = ();
}

impl parachain_info::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type FeeMultiplierUpdate = ();
	type LengthToFee = IdentityFee<Balance>;
	type OperationalFeeMultiplier = ConstU8<1>;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = IdentityFee<Balance>;
}

impl pallet_authorship::Config for Runtime {
	type EventHandler = (CollatorSelection,);
	type FindAuthor = FindAccountFromAuthorIndex<Self, Aura>;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
}

impl pallet_collator_selection::Config for Runtime {
	type Currency = Balances;
	type PotId = PotId;
	type KickThreshold = ConstU32<{ 6 * HOURS }>;
	type MaxCandidates = ConstU32<1_000>;
	type MaxInvulnerables = ConstU32<100>;
	type MinCandidates = ConstU32<5>;
	type RuntimeEvent = RuntimeEvent;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type ValidatorId = AccountId;
	type ValidatorIdOf = IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

impl pallet_session::Config for Runtime {
	type Keys = SessionKeys;
	type NextSessionRotation = PeriodicSessions<ConstU32<HOURS>, ConstU32<0>>;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type SessionManager = CollatorSelection;
	type ShouldEndSession = PeriodicSessions<ConstU32<HOURS>, ConstU32<0>>;
	type ValidatorId = AccountId;
	type ValidatorIdOf = IdentityCollator;
	type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl did::DeriveDidCallAuthorizationVerificationKeyRelationship for RuntimeCall {
	fn derive_verification_key_relationship(&self) -> did::DeriveDidCallKeyRelationshipResult {
		Ok(did::DidVerificationKeyRelationship::Authentication)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn get_call_for_did_call_benchmark() -> Self {
		RuntimeCall::System(frame_system::Call::remark {
			remark: b"test-remark".to_vec(),
		})
	}
}

parameter_types! {
	#[derive(Debug, Clone, Eq, PartialEq)]
	pub const MaxTotalKeyAgreementKeys: u32 = 1;
}

impl did::Config for Runtime {
	type Currency = Balances;
	type Deposit = ConstU128<UNIT>;
	type DidIdentifier = DidIdentifier;
	type EnsureOrigin = EnsureDidOrigin<DidIdentifier, AccountId>;
	type Fee = ConstU128<MILLIUNIT>;
	type FeeCollector = ();
	type MaxBlocksTxValidity = ConstU32<HOURS>;
	type MaxNewKeyAgreementKeys = ConstU32<1>;
	type MaxNumberOfServicesPerDid = ConstU32<1>;
	type MaxNumberOfTypesPerService = ConstU32<1>;
	type MaxNumberOfUrlsPerService = ConstU32<1>;
	type MaxPublicKeysPerDid = ConstU32<4>;
	type MaxServiceIdLength = ConstU32<100>;
	type MaxServiceTypeLength = ConstU32<100>;
	type MaxServiceUrlLength = ConstU32<100>;
	type MaxTotalKeyAgreementKeys = MaxTotalKeyAgreementKeys;
	type OriginSuccess = DidRawOrigin<AccountId, DidIdentifier>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type WeightInfo = ();
}

impl_runtime_apis! {
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> SlotDuration {
			SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(
			call: RuntimeCall,
			len: u32,
		) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(
			call: RuntimeCall,
			len: u32,
		) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl kilt_runtime_api_dip_sender::DipSender<Block, DidIdentifier, KeyIdOf<Runtime>, BTreeSet<KeyIdOf<Runtime>>, CompleteMerkleProof<Hash, DidMerkleProof<Runtime>>, ()> for Runtime {
		fn generate_proof(identifier: DidIdentifier, keys: BTreeSet<KeyIdOf<Runtime>>) -> Result<CompleteMerkleProof<Hash, DidMerkleProof<Runtime>>, ()> {
			if let Ok(Some((did_details, _))) = <Runtime as pallet_dip_sender::Config>::IdentityProvider::retrieve(&identifier) {
				DidMerkleRootGenerator::<Runtime>::generate_proof(&did_details, keys.iter())
			} else {
				Err(())
			}
		}
	}
}
