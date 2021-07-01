use super::Seed;
use ed25519_dalek::SecretKey;
use blake2::{
    VarBlake2b,
    digest::{Update, VariableOutput},
};
use byteorder::{BigEndian, WriteBytesExt};
use std::ops::{Deref, DerefMut};

/// Banano PrivateKey
#[derive(Debug)]
pub struct PrivateKey(SecretKey);

impl PrivateKey {
	pub fn from_seed(seed: Seed, index: u32) -> PrivateKey {
		let mut blake = VarBlake2b::new(32).unwrap();
		let mut index_buf = Vec::with_capacity(4);
		index_buf.write_u32::<BigEndian>(index).unwrap();
		blake.update(&*seed);
		blake.update(&index_buf);

		let mut buf = [0u8; 32];
        blake.finalize_variable(|res| buf.copy_from_slice(res));
		PrivateKey(SecretKey::from_bytes(&buf).unwrap())
	}
}

impl Deref for PrivateKey {
	type Target = SecretKey;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PrivateKey {
	fn deref_mut(&mut self) -> &mut SecretKey {
		&mut self.0
	}
}

impl From<PrivateKey> for [u8; 32] {
	fn from(key: PrivateKey) ->  Self {
		key.0.to_bytes()
	}
}