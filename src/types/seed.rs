use crate::errors::BananoError;
use data_encoding::{HEXUPPER_PERMISSIVE};
use std::ops::{Deref, DerefMut};

/// Banano Seed
#[derive(Clone)]
pub struct Seed(pub [u8; 64]);

impl Seed {
    /*
    pub fn from_string(seed: &str) -> Result<Self, BananoError> {
        Seed::from(seed.as_bytes())
    }
    */

	pub fn from<T: AsRef<[u8]>>(seed: T) -> Result<Self, BananoError> {
		let seed = seed.as_ref();
		if seed.len() != 64 {
			return Err(BananoError::SeedLengthError(seed.len()));
		}

        println!("Original Seed (len={}): {:02X?}", seed.len(), seed);
		let seed = HEXUPPER_PERMISSIVE.decode(&seed).unwrap();
        println!("Decoded  Seed (len={}): {:02X?}", seed.len(), seed);

		let mut seed_bytes = [0u8; 64];
		seed_bytes.copy_from_slice(&seed);

		Ok(Seed(seed_bytes))
	}
}

impl Deref for Seed {
	type Target = [u8; 64];
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Seed {
	fn deref_mut(&mut self) -> &mut [u8; 64] {
		&mut self.0
	}
}
