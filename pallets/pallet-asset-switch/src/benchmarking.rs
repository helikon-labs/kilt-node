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

use frame_benchmarking::v2::instance_benchmarks;
use xcm::{VersionedAssetId, VersionedInteriorMultiLocation, VersionedMultiAsset, VersionedMultiLocation};

pub struct PartialBenchmarkInfo {
	pub beneficiary: Option<VersionedInteriorMultiLocation>,
	pub destination: Option<VersionedMultiLocation>,
	pub remote_fee: Option<VersionedMultiAsset>,
	pub remote_asset_id: Option<VersionedAssetId>,
}

struct BenchmarkInfo {
	beneficiary: VersionedInteriorMultiLocation,
	destination: VersionedMultiLocation,
	remote_fee: VersionedMultiAsset,
	remote_asset_id: VersionedAssetId,
}

/// Helper trait implementable by the runtime to set some additional state
/// before the pallet benchmarks are run.
///
/// This is highly dependent on the runtime logic.
/// If no special conditions are to be met, it can simply be a no-op and return
/// `None`.
pub trait BenchmarkHelper {
	fn setup() -> Option<PartialBenchmarkInfo>;
}

impl BenchmarkHelper for () {
	fn setup() -> Option<PartialBenchmarkInfo> {
		None
	}
}

#[instance_benchmarks(where LocalCurrencyBalanceOf<T, I>: Into<u128>)]
mod benchmarks {
	use frame_support::traits::{
		fungible::{Inspect as InspectFungible, Mutate as MutateFungible},
		EnsureOrigin,
	};
	use frame_system::RawOrigin;
	use sp_runtime::traits::{TryConvert, Zero};
	use sp_std::{boxed::Box, vec};
	use xcm::{
		v3::{AssetId, Fungibility, Junction, Junctions, MultiAsset, MultiLocation, XcmContext},
		VersionedAssetId, VersionedInteriorMultiLocation, VersionedMultiAsset, VersionedMultiLocation,
	};
	use xcm_executor::traits::TransactAsset;

	use crate::{
		benchmarking::{BenchmarkHelper, BenchmarkInfo, PartialBenchmarkInfo},
		Call, Config, LocalCurrencyBalanceOf, Pallet, SwitchPairStatus,
	};

	const fn default_info() -> BenchmarkInfo {
		const DEFAULT_RESERVE_LOCATION: MultiLocation = MultiLocation {
			parents: 1,
			interior: Junctions::X1(Junction::Parachain(1_000)),
		};
		const DEFAULT_REMOTE_ASSET_ID: AssetId = AssetId::Concrete(DEFAULT_RESERVE_LOCATION);
		const DEFAULT_REMOTE_FEE: MultiAsset = MultiAsset {
			id: DEFAULT_REMOTE_ASSET_ID,
			fun: Fungibility::Fungible(100_000),
		};
		const DEFAULT_BENEFICIARY_JUNCTION: Junctions = Junctions::X1(Junction::AccountId32 {
			network: None,
			id: [0; 32],
		});

		BenchmarkInfo {
			beneficiary: VersionedInteriorMultiLocation::V3(DEFAULT_BENEFICIARY_JUNCTION),
			destination: VersionedMultiLocation::V3(DEFAULT_RESERVE_LOCATION),
			remote_asset_id: VersionedAssetId::V3(DEFAULT_REMOTE_ASSET_ID),
			remote_fee: VersionedMultiAsset::V3(DEFAULT_REMOTE_FEE),
		}
	}

	// Return the default info if the helper trait returns `None` or fills any
	// `None` field with a default value.
	fn fill_with_defaults(benchmark_info: Option<PartialBenchmarkInfo>) -> BenchmarkInfo {
		let default = default_info();

		let Some(benchmark_info) = benchmark_info else {
			return default;
		};

		BenchmarkInfo {
			beneficiary: benchmark_info.beneficiary.unwrap_or(default.beneficiary.into()),
			destination: benchmark_info.destination.unwrap_or(default.destination.into()),
			remote_asset_id: benchmark_info.remote_asset_id.unwrap_or(default.remote_asset_id.into()),
			remote_fee: benchmark_info.remote_fee.unwrap_or(default.remote_fee.into()),
		}
	}

	/// Write a switch pair into storage using the benchmark constants and the
	/// `remote_fee` asset as returned by the benchmark helper, or the default
	/// one otherwise.
	fn configure_switch_pair<T, I>() -> BenchmarkInfo
	where
		T: Config<I>,
		I: 'static,
		LocalCurrencyBalanceOf<T, I>: Into<u128>,
	{
		let benchmark_info = <T as Config<I>>::BenchmarkHelper::setup();
		let BenchmarkInfo {
			beneficiary,
			destination,
			remote_asset_id,
			remote_fee,
		} = fill_with_defaults(benchmark_info);

		Pallet::<T, I>::force_set_switch_pair(
			T::RuntimeOrigin::from(RawOrigin::Root),
			Box::new(destination.clone()),
			Box::new(remote_asset_id.clone()),
			Box::new(remote_fee.clone()),
			u128::MAX,
			u128::zero(),
		)
		.unwrap();
		assert!(Pallet::<T, I>::switch_pair().is_some());

		BenchmarkInfo {
			beneficiary,
			destination,
			remote_asset_id,
			remote_fee,
		}
	}

	#[benchmark]
	fn set_switch_pair() {
		let origin = <T as Config<I>>::SwitchOrigin::try_successful_origin().unwrap();
		let (reserve_location, remote_asset_id, remote_fee) = {
			let BenchmarkInfo {
				destination,
				remote_asset_id,
				remote_fee,
				..
			} = fill_with_defaults(<T as Config<I>>::BenchmarkHelper::setup());
			(Box::new(destination), Box::new(remote_asset_id), Box::new(remote_fee))
		};

		#[extrinsic_call]
		Pallet::<T, I>::set_switch_pair(
			origin as T::RuntimeOrigin,
			reserve_location,
			remote_asset_id,
			remote_fee,
			u128::MAX,
			u128::zero(),
		);

		assert!(Pallet::<T, I>::switch_pair().is_some());
	}

	#[benchmark]
	fn force_set_switch_pair() {
		let origin: T::RuntimeOrigin = RawOrigin::Root.into();
		let (reserve_location, remote_asset_id, remote_fee) = {
			let BenchmarkInfo {
				destination,
				remote_asset_id,
				remote_fee,
				..
			} = fill_with_defaults(<T as Config<I>>::BenchmarkHelper::setup());
			(Box::new(destination), Box::new(remote_asset_id), Box::new(remote_fee))
		};

		#[extrinsic_call]
		Pallet::<T, I>::force_set_switch_pair(
			origin as T::RuntimeOrigin,
			reserve_location,
			remote_asset_id,
			remote_fee,
			u128::MAX,
			u128::zero(),
		);

		assert!(Pallet::<T, I>::switch_pair().is_some());
	}

	#[benchmark]
	fn force_unset_switch_pair() {
		let origin: T::RuntimeOrigin = RawOrigin::Root.into();
		configure_switch_pair::<T, I>();

		#[extrinsic_call]
		Pallet::<T, I>::force_unset_switch_pair(origin as T::RuntimeOrigin);

		assert!(Pallet::<T, I>::switch_pair().is_none());
	}

	#[benchmark]
	fn pause_switch_pair() {
		let origin = <T as Config<I>>::PauseOrigin::try_successful_origin().unwrap();
		configure_switch_pair::<T, I>();

		#[extrinsic_call]
		Pallet::<T, I>::pause_switch_pair(origin as T::RuntimeOrigin);

		assert_eq!(Pallet::<T, I>::switch_pair().unwrap().status, SwitchPairStatus::Paused);
	}

	#[benchmark]
	fn resume_switch_pair() {
		let origin = <T as Config<I>>::SwitchOrigin::try_successful_origin().unwrap();
		configure_switch_pair::<T, I>();

		#[extrinsic_call]
		Pallet::<T, I>::resume_switch_pair(origin as T::RuntimeOrigin);

		assert_eq!(Pallet::<T, I>::switch_pair().unwrap().status, SwitchPairStatus::Running);
	}

	#[benchmark]
	fn update_remote_fee() {
		let origin = <T as Config<I>>::FeeOrigin::try_successful_origin().unwrap();
		let BenchmarkInfo { remote_fee, .. } = configure_switch_pair::<T, I>();
		let remote_fee = Box::new(remote_fee);
		let remote_fee_2 = remote_fee.clone();

		#[extrinsic_call]
		Pallet::<T, I>::update_remote_fee(origin as T::RuntimeOrigin, remote_fee);

		assert_eq!(Pallet::<T, I>::switch_pair().unwrap().remote_fee, *remote_fee_2);
	}

	#[benchmark]
	fn switch() {
		let origin = <T as Config<I>>::SubmitterOrigin::try_successful_origin().unwrap();
		let BenchmarkInfo {
			beneficiary,
			destination,
			remote_fee,
			remote_asset_id,
		} = configure_switch_pair::<T, I>();
		Pallet::<T, I>::resume_switch_pair(<T as Config<I>>::SwitchOrigin::try_successful_origin().unwrap()).unwrap();
		let account_id = <T as Config<I>>::SubmitterOrigin::ensure_origin(origin.clone()).unwrap();
		let pool_account = Pallet::<T, I>::pool_account_id_for_remote_asset(&remote_asset_id).unwrap();
		let minimum_balance = <T as Config<I>>::LocalCurrency::minimum_balance();
		// Set submitter balance to ED + 1_000 and pool balance to ED
		{
			<T as Config<I>>::LocalCurrency::set_balance(&account_id, minimum_balance + 1_000u32.into());
			<T as Config<I>>::LocalCurrency::set_balance(&pool_account, minimum_balance);
		}
		// Set submitter's fungible balance to the XCM fee
		let local_account_id_junction = <T as Config<I>>::AccountIdConverter::try_convert(account_id).unwrap();
		{
			<T as Config<I>>::AssetTransactor::deposit_asset(
				&remote_fee.try_into().unwrap(),
				&(local_account_id_junction.into()),
				&XcmContext::with_message_id(Default::default()),
			)
			.unwrap();
		}

		// Push the beneficiary to the returned `destination` value.
		let beneficiary = Box::new(
			MultiLocation::try_from(destination)
				.unwrap()
				.appended_with(Junctions::try_from(beneficiary).unwrap())
				.unwrap()
				.into(),
		);
		let amount = 1_000u32.into();

		#[extrinsic_call]
		Pallet::<T, I>::switch(origin as T::RuntimeOrigin, amount, beneficiary);

		assert_eq!(
			<T as Config<I>>::LocalCurrency::balance(&pool_account),
			minimum_balance + 1_000u32.into()
		);
	}

	#[cfg(test)]
	mod benchmark_tests {
		use crate::Pallet;

		frame_benchmarking::impl_benchmark_test_suite!(
			Pallet,
			crate::mock::ExtBuilder::default().build_with_keystore(),
			crate::mock::MockRuntime
		);
	}
}
