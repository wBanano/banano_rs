use ed25519_dalek::PublicKey as DalekPublicKey;
use crate::Error;
use ed25519_dalek::{Verifier, Signature};
use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
	pub fn to_ed25519_public_key(&self) -> DalekPublicKey {
		DalekPublicKey::from_bytes(&self.0).expect("Could not get private key from bytes.")

	}
	pub fn verify(&self, message: &[u8], signature: &[u8; 64]) -> Result<(), Error> {
		let key = self.to_ed25519_public_key();
		key.verify(message, &Signature::try_from(&signature[..]).expect("Could not create signature from bytes."))
		.map_err(|e| Error::SignatureError())
	}

}

impl From<PublicKey> for[u8; 32] {
	fn from(key: PublicKey) -> Self {
		key.0
	}
}

impl From<PublicKey> for DalekPublicKey {
	fn from(key: PublicKey) -> Self {
		DalekPublicKey::from_bytes(&key.0[..]).expect("Could not get public key from bytes.")
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn can_validate_signatures() {
		use crate::types::Seed;
		use crate::types::PrivateKey;
		use crate::types::PublicKey;
		use hex_literal::hex;

		let hash = hex!("938719851C7BAC4700B52FD0C0DD1CB4F47A1577B370F26616FA4D26B800B3E1");
		let seed = Seed([0u8; 32]);
		let private_key = PrivateKey::from_seed(seed, 0);
		let public_key = PublicKey::from(private_key);
		let signature = private_key.sign(hash);
		assert_eq!((), public_key.verify(&hash, &signature).unwrap());

	}
}
