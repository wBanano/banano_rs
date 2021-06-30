use crate::Error;
use ed25519_dalek::{PublicKey};

/// Address
#[derive(Debug, Clone, PartialEq)]
pub struct Address(pub String);

impl Address {
    /// Compute [PublicKey](PublicKey) from the Banano address
    pub fn to_public_key(&self) -> Result<PublicKey, Error> {
		if let Some("ban_") = self.0.get(..4) {
			if self.0.len() == 64 {
				let mut encoded_addr = String::from(self.0.get(4..56).unwrap());
				encoded_addr.insert_str(0, "1111");
				let checksum = self.0.get(56..).unwrap();
				let pkey_bytes = super::BAN_ENCODING.decode(encoded_addr.as_bytes())?;
				let derived_checksum = super::BAN_ENCODING.encode(&super::compute_address_checksum(&pkey_bytes[3..]));
				if checksum != derived_checksum {
					return Err(Error::InvalidAddress);
				}
				return Ok(PublicKey::from_bytes(&pkey_bytes[3..])?)
			}
			return Err(Error::InvalidAddressLength(self.0.len()));
		}
        Err(Error::InvalidAddress)
	}
}

impl From<PublicKey> for Address {
	fn from(key: PublicKey) -> Self {
		let mut p_key = key.to_bytes().to_vec();
		let mut h = [0u8; 3].to_vec();
		h.append(&mut p_key);
		let checksum = super::BAN_ENCODING.encode(&super::compute_address_checksum(key.as_bytes()));
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
