use crate::Error;
use super::pubkey::PublicKey;
use std::{convert::TryInto, ops::Deref};
use serde::{Serialize, Deserialize};

/// Address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    /// Compute [PublicKey] from the Banano address
    pub fn to_public_key(&self) -> Result<PublicKey, Error> {
		Address::create_public_key(&self.0)
	}

	pub fn create_public_key(pub_str :&String) -> Result<PublicKey, Error> {
		if pub_str.get(..4).unwrap() != "ban_" {
			return Err(Error::InvalidAddress);
		}

		if pub_str.len() != 64 {
			return Err(Error::InvalidAddressLength(pub_str.len()));
		}

		let mut encoded_addr = String::from(pub_str.get(4..56).unwrap());
		encoded_addr.insert_str(0, "1111");
		let checksum = pub_str.get(56..).unwrap();
		let pkey_bytes = super::BAN_ENCODING.decode(encoded_addr.as_bytes())?;
		let derived_checksum = super::BAN_ENCODING.encode(&super::compute_address_checksum(&pkey_bytes[3..]));
		if checksum != derived_checksum {
			return Err(Error::InvalidAddress);
		}
		Ok(super::pubkey::PublicKey(pkey_bytes[3..].try_into().expect("Not enough bytes for key")))
	}

}

impl From<PublicKey> for Address {
	fn from(key: PublicKey) -> Self {
		let b_key: [u8; 32] = key.into();
		let mut p_key = b_key.to_vec();
		let mut h = [0u8; 3].to_vec();
		h.append(&mut p_key);
		let checksum = super::BAN_ENCODING.encode(&super::compute_address_checksum(&b_key));
		let address = {
			let encoded_addr = super::BAN_ENCODING.encode(&h);
			let mut addr = String::from("ban_");
			addr.push_str(encoded_addr.get(4..).unwrap());
			addr.push_str(&checksum);
			addr
		};

		Address(address)
	}
}

impl Deref for Address {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
