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

use frame_support::sp_runtime::RuntimeDebug;

/// An error in the asset ID parsing logic.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum AssetIdError {
	/// An error in the asset namespace parsing logic.
	Namespace(NamespaceError),
	/// An error in the asset reference parsing logic.
	Reference(ReferenceError),
	/// An error in the asset identifier parsing logic.
	Identifier(IdentifierError),
	/// A generic error not belonging to any of the other categories.
	InvalidFormat,
}

/// An error in the asset namespace parsing logic.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum NamespaceError {
	/// Namespace too long.
	TooLong,
	/// Namespace too short.
	TooShort,
	/// A generic error not belonging to any of the other categories.
	InvalidFormat,
}

/// An error in the asset reference parsing logic.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum ReferenceError {
	/// Reference too long.
	TooLong,
	/// Reference too short.
	TooShort,
	/// A generic error not belonging to any of the other categories.
	InvalidFormat,
}

/// An error in the asset identifier parsing logic.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum IdentifierError {
	/// Identifier too long.
	TooLong,
	/// Identifier too short.
	TooShort,
	/// A generic error not belonging to any of the other categories.
	InvalidFormat,
}

impl From<NamespaceError> for AssetIdError {
	fn from(err: NamespaceError) -> Self {
		Self::Namespace(err)
	}
}

impl From<ReferenceError> for AssetIdError {
	fn from(err: ReferenceError) -> Self {
		Self::Reference(err)
	}
}

impl From<IdentifierError> for AssetIdError {
	fn from(err: IdentifierError) -> Self {
		Self::Identifier(err)
	}
}

// Exported types. This will always only re-export the latest version by
// default.
pub use v1::*;

pub mod v1 {
	use super::{AssetIdError, IdentifierError, NamespaceError, ReferenceError};

	use codec::{Decode, Encode, MaxEncodedLen};
	use scale_info::TypeInfo;

	use frame_support::{sp_runtime::RuntimeDebug, traits::ConstU32, BoundedVec};
	use sp_std::vec::Vec;

	pub const MINIMUM_ASSET_ID_LENGTH: usize = MINIMUM_NAMESPACE_LENGTH + b":".len() + MINIMUM_REFERENCE_LENGTH;
	pub const MAXIMUM_ASSET_ID_LENGTH: usize = MAXIMUM_NAMESPACE_LENGTH + b":".len() + MAXIMUM_REFERENCE_LENGTH + b":".len() + MAXIMUM_IDENTIFIER_LENGTH;

	pub const MINIMUM_NAMESPACE_LENGTH: usize = 3;
	pub const MAXIMUM_NAMESPACE_LENGTH: usize = 8;
	const MAXIMUM_NAMESPACE_LENGTH_U32: u32 = MAXIMUM_NAMESPACE_LENGTH as u32;
	pub const MINIMUM_REFERENCE_LENGTH: usize = 1;
	pub const MAXIMUM_REFERENCE_LENGTH: usize = 64;
	const MAXIMUM_REFERENCE_LENGTH_U32: u32 = MAXIMUM_REFERENCE_LENGTH as u32;
	pub const MINIMUM_IDENTIFIER_LENGTH: usize = 1;
	pub const MAXIMUM_IDENTIFIER_LENGTH: usize = 78;
	const MAXIMUM_IDENTIFIER_LENGTH_U32: u32 = MAXIMUM_IDENTIFIER_LENGTH as u32;

	// 20 bytes -> 40 HEX characters
	const EVM_SMART_CONTRACT_ADDRESS_LENGTH: usize = 40;

	// TODO: Add link to the Asset DID spec once merged.

	/// The Asset ID component as specified in the Asset DID specification.
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum AssetId {
		// A SLIP44 asset reference.
		Slip44(Slip44Reference),
		// An ERC20 asset reference.
		Erc20(EvmSmartContractFungibleReference),
		// An ERC721 asset reference.
		Erc721(EvmSmartContractNonFungibleReference),
		// An ERC1155 asset reference.
		Erc1155(EvmSmartContractNonFungibleReference),
		// A generic asset.
		Generic(GenericAssetId),
	}

	impl From<Slip44Reference> for AssetId {
		fn from(reference: Slip44Reference) -> Self {
			Self::Slip44(reference)
		}
	}

	impl From<EvmSmartContractFungibleReference> for AssetId {
		fn from(reference: EvmSmartContractFungibleReference) -> Self {
			Self::Erc20(reference)
		}
	}

	impl TryFrom<&[u8]> for AssetId {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			match value {
				// "slip44:" assets -> https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-20.md
				[b's', b'l', b'i', b'p', b'4', b'4', b':', asset_reference @ ..] => {
					Slip44Reference::try_from(asset_reference).map(Self::Slip44)
				}
				// "erc20:" assets -> https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-21.md
				[b'e', b'r', b'c', b'2', b'0', b':', asset_reference @ ..] => {
					EvmSmartContractFungibleReference::try_from(asset_reference).map(Self::Erc20)
				}
				// "erc721:" assets -> https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-22.md
				[b'e', b'r', b'c', b'7', b'2', b'1', b':', asset_reference @ ..] => {
					EvmSmartContractNonFungibleReference::try_from(asset_reference).map(Self::Erc721)
				}
				// "erc1155:" assets-> https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-29.md
				[b'e', b'r', b'c', b'1', b'1', b'5', b'5', b':', asset_reference @ ..] => {
					EvmSmartContractNonFungibleReference::try_from(asset_reference).map(Self::Erc1155)
				}
				// Generic yet valid asset IDs
				asset_id => GenericAssetId::try_from(asset_id).map(Self::Generic),
			}
		}
	}

	impl TryFrom<Vec<u8>> for AssetId {
		type Error = AssetIdError;

		fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
			Self::try_from(&value[..])
		}
	}

	impl TryFrom<&'static str> for AssetId {
		type Error = AssetIdError;

		fn try_from(value: &'static str) -> Result<Self, Self::Error> {
			Self::try_from(value.as_bytes())
		}
	}

	#[cfg(feature = "std")]
	impl TryFrom<String> for AssetId {
		type Error = AssetIdError;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			Self::try_from(value.as_bytes())
		}
	}

	/// A Slip44 asset reference.
	/// It is a modification of the [CAIP-20 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-20.md)
	/// according to the rules defined in the Asset DID method specification.
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Slip44Reference(pub BoundedVec<u8, ConstU32<MAXIMUM_REFERENCE_LENGTH_U32>>);

	impl Slip44Reference {
		/// [CAN PANIC]
		/// Tries to create a Slip44 reference from the provided slice,
		/// panicking if the slice is longer than the maximum length allowed.
		pub(crate) fn from_slice_unchecked(slice: &[u8]) -> Self {
			Self(
				slice
					.to_vec()
					.try_into()
					.expect("Slip44Reference::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for Slip44Reference {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length < MINIMUM_REFERENCE_LENGTH {
				Err(ReferenceError::TooShort.into())
			} else if input_length > MAXIMUM_REFERENCE_LENGTH {
				Err(ReferenceError::TooLong.into())
			} else {
				value.iter().try_for_each(|c| {
					if !(b'0'..=b'9').contains(c) {
						Err(ReferenceError::InvalidFormat)
					} else {
						Ok(())
					}
				})?;
				// Unchecked since we already checked for length
				Ok(Self::from_slice_unchecked(value))
			}
		}
	}

	/// An asset reference that is identifiable only by an EVM smart contract
	/// (e.g., a fungible token). It is a modification of the [CAIP-21 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-21.md)
	/// according to the rules defined in the Asset DID method specification.
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct EvmSmartContractFungibleReference(pub [u8; EVM_SMART_CONTRACT_ADDRESS_LENGTH]);

	impl EvmSmartContractFungibleReference {
		/// [CAN PANIC]
		/// Tries to create an EvmSmartContractFungibleReference reference from
		/// the provided slice, panicking if the slice is longer than the
		/// maximum length allowed.
		pub(crate) fn from_slice_unchecked(slice: &[u8]) -> Self {
			Self(
				slice
					.try_into()
					.expect("EvmSmartContractFungibleReference::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for EvmSmartContractFungibleReference {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			match value {
				// If the prefix is "0x" => parse the address
				[b'0', b'x', contract_address @ ..] => {
					let inner: [u8; EVM_SMART_CONTRACT_ADDRESS_LENGTH] =
						contract_address.try_into().map_err(|_| ReferenceError::InvalidFormat)?;
					inner.iter().try_for_each(|c| {
						if !matches!(c, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
							Err(ReferenceError::InvalidFormat)
						} else {
							Ok(())
						}
					})?;
					// Unchecked since we already checked for length
					Ok(Self::from_slice_unchecked(contract_address))
				}
				// Otherwise fail
				_ => Err(ReferenceError::InvalidFormat.into()),
			}
		}
	}

	/// An asset reference that is identifiable by an EVM smart contract and an
	/// optional identifier (e.g., an NFT collection or instance thereof). It is a modification of the [CAIP-22 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-22.md) and [CAIP-29 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-29.md)
	/// according to the rules defined in the Asset DID method specification.
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct EvmSmartContractNonFungibleReference(
		pub EvmSmartContractFungibleReference,
		pub Option<EvmSmartContractNonFungibleIdentifier>,
	);

	impl EvmSmartContractNonFungibleReference {
		/// [CAN PANIC]
		/// Tries to create an EvmSmartContractNonFungibleReference reference
		/// from the provided raw components, panicking if the any of them is
		/// longer than the maximum length allowed.
		pub(crate) fn from_raw_unchecked(reference: &[u8], id: Option<&[u8]>) -> Self {
			Self(
				EvmSmartContractFungibleReference::from_slice_unchecked(reference),
				id.map(EvmSmartContractNonFungibleIdentifier::from_slice_unchecked),
			)
		}
	}

	impl TryFrom<&[u8]> for EvmSmartContractNonFungibleReference {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let mut components = value.split(|c| *c == b':');

			let reference = components
				.next()
				.ok_or_else(|| ReferenceError::InvalidFormat.into())
				.and_then(EvmSmartContractFungibleReference::try_from)?;

			let id = components
				.next()
				// Transform Option<Result> to Result<Option> and bubble Err case up, keeping Ok(Option) for successful
				// cases.
				.map_or(Ok(None), |id| {
					EvmSmartContractNonFungibleIdentifier::try_from(id).map(Some)
				})?;
			Ok(Self(reference, id))
		}
	}

	/// An asset identifier for an EVM smart contract collection (e.g., an NFT
	/// instance).
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct EvmSmartContractNonFungibleIdentifier(pub BoundedVec<u8, ConstU32<MAXIMUM_IDENTIFIER_LENGTH_U32>>);

	impl EvmSmartContractNonFungibleIdentifier {
		/// [CAN PANIC]
		/// Tries to create an EvmSmartContractNonFungibleIdentifier reference
		/// from the provided slice, panicking if the slice is longer than the
		/// maximum length allowed.
		fn from_slice_unchecked(value: &[u8]) -> Self {
			Self(
				value
					.to_vec()
					.try_into()
					.expect("EvmSmartContractNonFungibleIdentifier::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for EvmSmartContractNonFungibleIdentifier {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length < MINIMUM_IDENTIFIER_LENGTH {
				Err(IdentifierError::TooShort.into())
			} else if input_length > MAXIMUM_IDENTIFIER_LENGTH {
				Err(IdentifierError::TooLong.into())
			} else {
				value.iter().try_for_each(|c| {
					if !matches!(c, b'0'..=b'9') {
						Err(IdentifierError::InvalidFormat)
					} else {
						Ok(())
					}
				})?;
				value
					.to_vec()
					.try_into()
					.map(Self)
					.map_err(|_| IdentifierError::InvalidFormat.into())
			}
		}
	}

	/// A generic asset ID compliant with the [CAIP-19 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-19.md) that cannot be boxed in any of the supported variants.
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct GenericAssetId {
		pub namespace: GenericAssetNamespace,
		pub reference: GenericAssetReference,
		pub id: Option<GenericAssetIdentifier>,
	}

	impl GenericAssetId {
		/// [CAN PANIC]
		/// Tries to create a GenericAssetId identifier from the provided raw
		/// components, panicking if the any of them is longer than the maximum
		/// length allowed.\
		#[allow(dead_code)]
		fn from_raw_unchecked(namespace: &[u8], reference: &[u8], id: Option<&[u8]>) -> Self {
			Self {
				namespace: GenericAssetNamespace::from_slice_unchecked(namespace),
				reference: GenericAssetReference::from_slice_unchecked(reference),
				id: id.map(GenericAssetIdentifier::from_slice_unchecked).or(None),
			}
		}
	}

	impl TryFrom<&[u8]> for GenericAssetId {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length > MAXIMUM_ASSET_ID_LENGTH {
				return Err(AssetIdError::InvalidFormat);
			}

			let mut components = value.split(|c| *c == b':');

			let namespace = components
				.next()
				.ok_or(AssetIdError::InvalidFormat)
				.and_then(GenericAssetNamespace::try_from)?;
			let reference = components
				.next()
				.ok_or(AssetIdError::InvalidFormat)
				.and_then(GenericAssetReference::try_from)?;
			let id = components
				.next()
				// Transform Option<Result> to Result<Option> and bubble Err case up, keeping Ok(Option) for successful
				// cases.
				.map_or(Ok(None), |id| GenericAssetIdentifier::try_from(id).map(Some))?;

			Ok(Self {
				namespace,
				reference,
				id,
			})
		}
	}

	/// A generic asset namespace as defined in the [CAIP-19 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-19.md).
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct GenericAssetNamespace(pub BoundedVec<u8, ConstU32<MAXIMUM_NAMESPACE_LENGTH_U32>>);

	impl GenericAssetNamespace {
		/// [CAN PANIC]
		/// Tries to create a GenericAssetNamespace namespace from the provided
		/// slice, panicking if the slice is longer than the maximum length
		/// allowed.
		fn from_slice_unchecked(value: &[u8]) -> Self {
			Self(
				value
					.to_vec()
					.try_into()
					.expect("GenericAssetNamespace::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for GenericAssetNamespace {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length < MINIMUM_NAMESPACE_LENGTH {
				Err(NamespaceError::TooShort.into())
			} else if input_length > MAXIMUM_NAMESPACE_LENGTH {
				Err(NamespaceError::TooLong.into())
			} else {
				value.iter().try_for_each(|c| {
					if !matches!(c, b'-' | b'a'..=b'z' | b'0'..=b'9') {
						Err(NamespaceError::InvalidFormat)
					} else {
						Ok(())
					}
				})?;
				// Unchecked since we already checked for length
				Ok(Self::from_slice_unchecked(value))
			}
		}
	}

	/// A generic asset reference as defined in the [CAIP-19 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-19.md).
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct GenericAssetReference(pub BoundedVec<u8, ConstU32<MAXIMUM_REFERENCE_LENGTH_U32>>);

	impl GenericAssetReference {
		/// [CAN PANIC]
		/// Tries to create a GenericAssetReference reference from the provided
		/// slice, panicking if the slice is longer than the maximum length
		/// allowed.
		fn from_slice_unchecked(value: &[u8]) -> Self {
			Self(
				value
					.to_vec()
					.try_into()
					.expect("GenericAssetReference::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for GenericAssetReference {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length < MINIMUM_REFERENCE_LENGTH {
				Err(ReferenceError::TooShort.into())
			} else if input_length > MAXIMUM_REFERENCE_LENGTH {
				Err(ReferenceError::TooLong.into())
			} else {
				value.iter().try_for_each(|c| {
					if !matches!(c, b'-' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
						Err(ReferenceError::InvalidFormat)
					} else {
						Ok(())
					}
				})?;
				// Unchecked since we already checked for length
				Ok(Self::from_slice_unchecked(value))
			}
		}
	}

	/// A generic asset identifier as defined in the [CAIP-19 spec](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-19.md).
	#[non_exhaustive]
	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct GenericAssetIdentifier(pub BoundedVec<u8, ConstU32<MAXIMUM_IDENTIFIER_LENGTH_U32>>);

	impl GenericAssetIdentifier {
		/// [CAN PANIC]
		/// Tries to create a GenericAssetIdentifier identifier from the
		/// provided slice, panicking if the slice is longer than the maximum
		/// length allowed.
		fn from_slice_unchecked(value: &[u8]) -> Self {
			Self(
				value
					.to_vec()
					.try_into()
					.expect("GenericAssetIdentifier::from_slice_unchecked should never panic."),
			)
		}
	}

	impl TryFrom<&[u8]> for GenericAssetIdentifier {
		type Error = AssetIdError;

		fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
			let input_length = value.len();
			if input_length < MINIMUM_IDENTIFIER_LENGTH {
				Err(IdentifierError::TooShort.into())
			} else if input_length > MAXIMUM_IDENTIFIER_LENGTH {
				Err(IdentifierError::TooLong.into())
			} else {
				value.iter().try_for_each(|c| {
					if !matches!(c, b'-' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
						Err(IdentifierError::InvalidFormat)
					} else {
						Ok(())
					}
				})?;
				// Unchecked since we already checked for length
				Ok(Self::from_slice_unchecked(value))
			}
		}
	}

	#[cfg(test)]
	mod test {
		use super::*;

		#[test]
		fn test_slip44_assets() {
			let valid_assets = [
				"slip44:60",
				"slip44:0",
				"slip44:2",
				"slip44:714",
				"slip44:234",
				"slip44:134",
				"slip44:0",
				"slip44:9999999999999999999999999999999999999999999999999999999999999999",
			];

			for asset in valid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_ok(),
					"Asset ID {:?} should not fail to parse for slip44 assets",
					asset
				);
			}

			let invalid_assets = [
				// Too short
				"",
				"s",
				"sl",
				"sli",
				"slip",
				"slip4",
				"slip44",
				"slip44:",
				// Not a number
				"slip44:a",
				"slip44::",
				"slip44:›",
				"slip44:😁",
				// Max chars + 1
				"slip44:99999999999999999999999999999999999999999999999999999999999999999",
			];
			for asset in invalid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_err(),
					"Asset ID {:?} should fail to parse for slip44 assets",
					asset
				);
			}
		}

		#[test]
		fn test_erc20_assets() {
			let valid_assets = [
				"erc20:0x6b175474e89094c44da98b954eedeac495271d0f",
				"erc20:0x8f8221AFBB33998D8584A2B05749BA73C37A938A",
			];

			for asset in valid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_ok(),
					"Asset ID {:?} should not fail to parse for erc20 assets",
					asset
				);
			}

			let invalid_assets = [
				// Too short
				"",
				"e",
				"er",
				"erc",
				"erc2",
				"erc20",
				"erc20:",
				// Not valid HEX characters
				"erc20::",
				"erc20:›",
				"erc20:😁",
				// Max chars - 1
				"erc20:0x8f8221AFBB33998D8584A2B05749BA73C37A938",
				// Max chars + 1
				"erc20:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1",
				// Asset ID (not supported for erc20 standard)
				"erc20:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:1",
				// Smart contract without leading `0x`
				"erc20:8f8221AFBB33998D8584A2B05749BA73C37A938A1",
			];
			for asset in invalid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_err(),
					"Asset ID {:?} should fail to parse for erc20 assets",
					asset
				);
			}
		}

		#[test]
		fn test_erc721_assets() {
			let valid_assets = [
			"erc721:0x6b175474e89094c44da98b954eedeac495271d0f",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A:0",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A:999999999999999999999999999999999999999999999999999999999999999999999999",
		];

			for asset in valid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_ok(),
					"Asset ID {:?} should not fail to parse for erc721 assets",
					asset
				);
			}

			let invalid_assets = [
			// Too short
			"",
			"e",
			"er",
			"erc",
			"erc7",
			"erc72",
			"erc721",
			"erc721:",
			// Not valid HEX characters
			"erc721::",
			"erc721:›",
			"erc721:😁",
			// Max chars - 1
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938",
			// Max chars + 1
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1",
			// Wrong asset IDs
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:a",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:9999999999999999999999999999999999999999999999999999999999999999999999999",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:‹",
			"erc721:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:😁",
			// Smart contract without leading `0x`
			"erc721:8f8221AFBB33998D8584A2B05749BA73C37A938A1",
		];
			for asset in invalid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_err(),
					"Asset ID {:?} should fail to parse for erc721 assets",
					asset
				);
			}
		}

		#[test]
		fn test_erc1155_assets() {
			let valid_assets = [
			"erc1155:0x6b175474e89094c44da98b954eedeac495271d0f",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A:0",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A:999999999999999999999999999999999999999999999999999999999999999999999999",
		];

			for asset in valid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_ok(),
					"Asset ID {:?} should not fail to parse for erc1155 assets",
					asset
				);
			}

			let invalid_assets = [
			// Too short
			"",
			"e",
			"er",
			"erc",
			"erc1",
			"erc11",
			"erc115",
			"erc1155",
			"erc1155:",
			// Not valid HEX characters
			"erc1155::",
			"erc1155:›",
			"erc1155:😁",
			// Max chars - 1
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938",
			// Max chars + 1
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1",
			// Wrong asset IDs
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:a",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:9999999999999999999999999999999999999999999999999999999999999999999999999",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:‹",
			"erc1155:0x8f8221AFBB33998D8584A2B05749BA73C37A938A1:😁",
			// Smart contract without leading `0x`
			"erc721:8f8221AFBB33998D8584A2B05749BA73C37A938A1",
		];
			for asset in invalid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_err(),
					"Asset ID {:?} should fail to parse for erc1155 assets",
					asset
				);
			}
		}

		#[test]
		fn test_generic_assets() {
			let valid_assets = [
			"123:a",
			"12345678:-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-",
			"12345678:-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-:-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ012345678901234567890123-",
			"para:411f057b9107718c9624d6aa4a3f23c1",
			"para:kilt-spiritnet",
			"w3n:john-doe",
		];

			for asset in valid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_ok(),
					"Asset ID {:?} should not fail to parse for generic assets",
					asset
				);
			}

			let invalid_assets = [
				// Too short
				"",
				"a",
				"as",
				"as:",
				"‹",
				"‹:",
				"asd:",
				":",
				"::",
				":::",
				"::::",
				"valid:valid:",
				// Too long
				"too-loong:valid",
				"valid:too-loooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
				"valid:valid:too-loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
				// Wrong characters
				"no-val!d:valid",
				"valid:no-val!d",
				"valid:valid:no-val!d",
			];
			for asset in invalid_assets {
				assert!(
					AssetId::try_from(asset.as_bytes()).is_err(),
					"Asset ID {:?} should fail to parse for generic assets",
					asset
				);
			}
		}
	}
}
