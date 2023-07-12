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

use sp_std::marker::PhantomData;

pub trait IdentityProofVerifier<Call, Subject> {
	type Error;
	type IdentityDetails;
	type Proof;
	type Submitter;
	type VerificationResult;

	fn verify_proof_for_call_against_details(
		call: &Call,
		subject: &Subject,
		submitter: &Self::Submitter,
		identity_details: &mut Option<Self::IdentityDetails>,
		proof: &Self::Proof,
	) -> Result<Self::VerificationResult, Self::Error>;
}

// Always returns success.
pub struct SuccessfulProofVerifier<IdentityDetails, Proof, Submitter>(PhantomData<(IdentityDetails, Proof, Submitter)>);
impl<Call, Subject, IdentityDetails, Proof, Submitter> IdentityProofVerifier<Call, Subject>
	for SuccessfulProofVerifier<IdentityDetails, Proof, Submitter>
{
	type Error = ();
	type IdentityDetails = IdentityDetails;
	type Proof = Proof;
	type Submitter = Submitter;
	type VerificationResult = ();

	fn verify_proof_for_call_against_details(
		_call: &Call,
		_subject: &Subject,
		_submitter: &Self::Submitter,
		_identity_details: &mut Option<Self::IdentityDetails>,
		_proof: &Self::Proof,
	) -> Result<Self::VerificationResult, Self::Error> {
		Ok(())
	}
}
