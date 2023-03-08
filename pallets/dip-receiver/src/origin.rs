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

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::EnsureOrigin, RuntimeDebug};
use scale_info::TypeInfo;
use sp_std::marker::PhantomData;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DipOrigin<DidIdentifier, AccountId> {
	pub did_identifier: DidIdentifier,
	pub account_address: AccountId,
}

pub struct EnsureDipOrigin<DidIdentifier, AccountId>(PhantomData<(DidIdentifier, AccountId)>);

impl<OuterOrigin, DidIdentifier, AccountId> EnsureOrigin<OuterOrigin> for EnsureDipOrigin<DidIdentifier, AccountId>
where
	OuterOrigin:
		From<DipOrigin<DidIdentifier, AccountId>> + Into<Result<DipOrigin<DidIdentifier, AccountId>, OuterOrigin>>,
	AccountId: From<[u8; 32]>,
	DidIdentifier: From<[u8; 32]>,
{
	type Success = DipOrigin<DidIdentifier, AccountId>;

	fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
		o.into()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<OuterOrigin, ()> {
		Ok(OuterOrigin::from(DipOrigin {
			did_identifier: DidIdentifier::from([0u8; 32]),
			account_address: AccountId::from([0u8; 32]),
		}))
	}
}

impl<DidIdentifier, AccountId> kilt_support::traits::CallSources<AccountId, DidIdentifier>
	for DipOrigin<DidIdentifier, AccountId>
where
	DidIdentifier: Clone,
	AccountId: Clone,
{
	fn sender(&self) -> AccountId {
		self.account_address.clone()
	}

	fn subject(&self) -> DidIdentifier {
		self.did_identifier.clone()
	}
}
