use crate::Error;
use data_encoding::{HEXUPPER_PERMISSIVE};
use std::ops::{Deref, DerefMut};

/// Banano Seed
#[derive(Clone)]
pub struct Seed(pub [u8; 32]);

impl Seed {
	pub fn from<T: AsRef<[u8]>>(seed: T) -> Result<Self, Error> {
		let seed = seed.as_ref();
		if seed.len() != 64 {
			return Err(Error::SeedLengthError(seed.len()));
		}

		let seed = HEXUPPER_PERMISSIVE.decode(&seed).unwrap();
		let mut seed_bytes = [0u8; 32];
		seed_bytes.copy_from_slice(&seed);
		Ok(Seed(seed_bytes))
	}
}

impl Deref for Seed {
	type Target = [u8; 32];
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Seed {
	fn deref_mut(&mut self) -> &mut [u8; 32] {
		&mut self.0
	}
}
