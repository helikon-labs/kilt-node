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

use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use sp_runtime::DispatchError;
use xcm::v3::{Fungibility, MultiAsset};

use crate::{
	mock::{ExtBuilder, MockRuntime, System, ASSET_HUB_LOCATION, REMOTE_ERC20_ASSET_ID, XCM_ASSET_FEE},
	Error, Event, Pallet, SwapPair, SwapPairInfoOf,
};

#[test]
fn successful() {
	// Setting the fee to a new value generates an event.
	let new_fee = MultiAsset {
		fun: Fungibility::Fungible(1),
		..XCM_ASSET_FEE
	};
	ExtBuilder::default()
		.with_swap_pair_info(SwapPairInfoOf::<MockRuntime> {
			pool_account: [0u8; 32].into(),
			remote_asset_balance: 1_000,
			remote_asset_id: REMOTE_ERC20_ASSET_ID.into(),
			remote_fee: XCM_ASSET_FEE.into(),
			remote_reserve_location: ASSET_HUB_LOCATION.into(),
			status: Default::default(),
		})
		.build()
		.execute_with(|| {
			assert_ok!(Pallet::<MockRuntime>::update_remote_fee(
				RawOrigin::Root.into(),
				Box::new(new_fee.clone().into())
			));
			assert_eq!(
				SwapPair::<MockRuntime>::get().unwrap().remote_fee,
				new_fee.clone().into()
			);
			assert!(System::events().into_iter().map(|e| e.event).any(|e| e
				== Event::<MockRuntime>::SwapPairFeeUpdated {
					old: XCM_ASSET_FEE.into(),
					new: new_fee.clone().into()
				}
				.into()));
		});
	// Setting the fee to the same value does not generate an event.
	ExtBuilder::default()
		.with_swap_pair_info(SwapPairInfoOf::<MockRuntime> {
			pool_account: [0u8; 32].into(),
			remote_asset_balance: 1_000,
			remote_asset_id: REMOTE_ERC20_ASSET_ID.into(),
			remote_fee: XCM_ASSET_FEE.into(),
			remote_reserve_location: ASSET_HUB_LOCATION.into(),
			status: Default::default(),
		})
		.build()
		.execute_with(|| {
			assert_ok!(Pallet::<MockRuntime>::update_remote_fee(
				RawOrigin::Root.into(),
				Box::new(XCM_ASSET_FEE.into())
			));
			assert_eq!(SwapPair::<MockRuntime>::get().unwrap().remote_fee, XCM_ASSET_FEE.into());
			assert!(System::events().into_iter().map(|e| e.event).all(|e| e
				!= Event::<MockRuntime>::SwapPairFeeUpdated {
					old: XCM_ASSET_FEE.into(),
					new: XCM_ASSET_FEE.into(),
				}
				.into()));
		});
}

#[test]
fn fails_on_invalid_origin() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Pallet::<MockRuntime>::update_remote_fee(RawOrigin::None.into(), Box::new(XCM_ASSET_FEE.into()),),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn fails_on_non_existing_swap_pair() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Pallet::<MockRuntime>::update_remote_fee(RawOrigin::Root.into(), Box::new(XCM_ASSET_FEE.into()),),
			Error::<MockRuntime>::NotFound
		);
	});
}
