use super::Address;
use super::PrivateKey;
use ed25519_dalek::{
    PublicKey,
};

/// Banano account, made of a [PublicKey](PublicKey) and an [Address](Address)
#[derive(Debug, Clone)]
pub struct Account {
	pub public_key: PublicKey,
	pub address: Address,
}

impl From<PublicKey> for Account {
	fn from(key: PublicKey) -> Self {
		Account {
			public_key: key.clone(),
			address: key.into(),
		}
	}
}

impl From<PrivateKey> for Account {
	fn from(key: PrivateKey) -> Self {
		let public_key: PublicKey = key.into();
		Account {
			public_key: public_key.clone(),
			address: public_key.into(),
		}
	}
}
