use crate::{
	mock::{
		network::MockNetworkPolkadot,
		para_chains::{AssetHubPolkadot, AssetHubPolkadotPallet, Spiritnet},
		relay_chains::Polkadot,
	},
	tests::spiritnet::did_pallets::utils::{
		construct_xcm_message, create_mock_ctype, create_mock_did, get_asset_hub_sovereign_account,
		get_sibling_destination_spiritnet,
	},
};
use frame_support::{assert_ok, traits::fungible::Mutate};
use kilt_asset_dids::AssetDid as AssetIdentifier;
use parity_scale_codec::Encode;
use runtime_common::{constants::KILT, AccountId, Balance};
use sp_core::H256;
use sp_runtime::BoundedVec;
use xcm::{DoubleEncoded, VersionedXcm};
use xcm_emulator::{assert_expected_events, OriginKind, Parachain, TestExt};

fn get_xcm_message_add_public_credential(
	origin_kind: OriginKind,
	withdraw_balance: Balance,
	ctype_hash: H256,
) -> VersionedXcm<()> {
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	let subject_id = AssetIdentifier::ether_currency();

	let credential = public_credentials::mock::generate_base_public_credential_creation_op::<spiritnet_runtime::Runtime>(
		BoundedVec::try_from(subject_id.encode()).unwrap(),
		ctype_hash,
		Default::default(),
	);

	let call: DoubleEncoded<()> = <Spiritnet as Parachain>::RuntimeCall::Did(did::Call::dispatch_as {
		did_identifier: asset_hub_sovereign_account,
		call: Box::new(<Spiritnet as Parachain>::RuntimeCall::PublicCredentials(
			public_credentials::Call::add {
				credential: Box::new(credential),
			},
		)),
	})
	.encode()
	.into();

	construct_xcm_message(origin_kind, withdraw_balance, call)
}

#[test]
fn test_create_public_credential_from_asset_hub() {
	MockNetworkPolkadot::reset();

	let sudo_origin = <AssetHubPolkadot as Parachain>::RuntimeOrigin::root();
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();
	let ctype_hash_value = H256([0; 32]);

	let init_balance = KILT * 10;

	let xcm_claim_w3n_call =
		get_xcm_message_add_public_credential(OriginKind::SovereignAccount, KILT, ctype_hash_value);

	let destination = get_sibling_destination_spiritnet();

	Spiritnet::execute_with(|| {
		create_mock_did();
		create_mock_ctype(ctype_hash_value.clone());
		<spiritnet_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
	});

	AssetHubPolkadot::execute_with(|| {
		assert_ok!(<AssetHubPolkadot as AssetHubPolkadotPallet>::PolkadotXcm::send(
			sudo_origin,
			Box::new(destination),
			Box::new(xcm_claim_w3n_call)
		));

		type RuntimeEvent = <AssetHubPolkadot as Parachain>::RuntimeEvent;
		assert_expected_events!(
			AssetHubPolkadot,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	Spiritnet::execute_with(|| {
		type SpiritnetRuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

		assert_expected_events!(
			Spiritnet,
			vec![
				SpiritnetRuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. }) => {},
				SpiritnetRuntimeEvent::Did(did::Event::DidCallDispatched(account, result)) => {
					account: account == &asset_hub_sovereign_account,
					result: result.is_ok(),
				},
				SpiritnetRuntimeEvent::PublicCredentials(public_credentials::Event::CredentialStored{ subject_id: _, credential_id: _ }) => {

				},
			]
		);
	});

	Polkadot::execute_with(|| {
		assert_eq!(Polkadot::events().len(), 0);
	});
}

#[test]
fn test_create_public_credential_from_asset_hub_unsuccessful() {
	let origin_kind_list = vec![OriginKind::Native, OriginKind::Superuser, OriginKind::Xcm];

	let sudo_origin = <AssetHubPolkadot as Parachain>::RuntimeOrigin::root();
	let init_balance = KILT * 100;
	let ctype_hash_value = H256([0; 32]);

	let destination = get_sibling_destination_spiritnet();
	let asset_hub_sovereign_account = get_asset_hub_sovereign_account();

	for origin_kind in origin_kind_list {
		MockNetworkPolkadot::reset();

		Polkadot::execute_with(|| {
			create_mock_did();
			create_mock_ctype(ctype_hash_value.clone());
			<spiritnet_runtime::Balances as Mutate<AccountId>>::set_balance(&asset_hub_sovereign_account, init_balance);
		});

		let xcm_claim_w3n_call = get_xcm_message_add_public_credential(origin_kind, KILT, ctype_hash_value);

		AssetHubPolkadot::execute_with(|| {
			assert_ok!(<AssetHubPolkadot as AssetHubPolkadotPallet>::PolkadotXcm::send(
				sudo_origin.clone(),
				Box::new(destination.clone()),
				Box::new(xcm_claim_w3n_call.clone())
			));

			type RuntimeEvent = <AssetHubPolkadot as Parachain>::RuntimeEvent;
			assert_expected_events!(
				AssetHubPolkadot,
				vec![
					RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
				]
			);
		});

		Spiritnet::execute_with(|| {
			type SpiritnetRuntimeEvent = <Spiritnet as Parachain>::RuntimeEvent;

			let is_event_present = Spiritnet::events().iter().any(|event| match event {
				SpiritnetRuntimeEvent::Did(did::Event::DidCallDispatched(_, _)) => true,
				SpiritnetRuntimeEvent::DidLookup(pallet_did_lookup::Event::AssociationEstablished(_, _)) => true,
				_ => false,
			});

			assert!(!is_event_present)
		});

		Polkadot::execute_with(|| {
			assert_eq!(Polkadot::events().len(), 0);
		});
	}
}
