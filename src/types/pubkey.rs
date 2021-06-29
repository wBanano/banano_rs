use super::privkey::PrivateKey;
use ed25519_dalek::PublicKey;

impl From<PrivateKey> for PublicKey {
	fn from(key: PrivateKey) -> Self {
        key.into()
        // PublicKey::from_secret::<Blake2b>(&key)
	}
}
