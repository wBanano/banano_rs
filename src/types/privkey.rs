use super::Seed;
use super::PublicKey;

use ed25519_dalek::{SecretKey, ExpandedSecretKey};

use blake2::{VarBlake2b};
use blake2::digest::{Update, VariableOutput};

use std::ops::{Deref, DerefMut};

const SIGNATURE_LENGTH : usize = 64;

/// Banano PrivateKey
#[derive(Debug, Copy, Clone)]
pub struct PrivateKey(pub(crate)[u8; 32]);

impl PrivateKey {
	pub fn from_seed(seed: Seed, index: u32) -> PrivateKey {

		// Update conflicts with the update trate in blake2::Blake2 so it is shadowed like this
		// instead of imported above.
		let mut blake = VarBlake2b::new(32).expect("Output size was zero");
		blake.update(seed.0);
		blake.update(index.to_be_bytes());
		let mut buf = [0u8; 32];
        blake.finalize_variable(|res| buf.copy_from_slice(res));
		PrivateKey(buf)
	}

	pub fn copy_bytes(&self) -> [u8; 32] {
		self.0.clone()
	}

	// pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> [u8; 64]
	// {
	// 	let mut keys = Vec::new();
	// 	keys.append_elements(+)
	// 	let expanded: ExpandedSecretKey = (&self.0).into();
	// 	// Less complex API but requires more math by deriving the public key each time
	// 	// vs. having a KeyPair type do the signing which is more common.
	// 	let pkey_copy = PrivateKey(SecretKey::from_bytes(&self.copy_bytes()).expect("Could not create private key from bytes"));
	// 	let public_key: PublicKey = pkey_copy.into();
	// 	let public_key: DalicPublicKey = public_key.into();
	// 	self.0.sign
    //     expanded.sign(data.as_ref(), &public_key).to_bytes()

	// }

	pub fn sign<T: AsRef<[u8]>>(&self, message: T) -> [u8; SIGNATURE_LENGTH]
	{
		let secret_key = SecretKey::from_bytes(&self.0).expect("Could not get secret from bytes.");
		let expanded_secret = ExpandedSecretKey::from(&secret_key);
		let signed = expanded_secret.sign(message.as_ref(), &ed25519_dalek::PublicKey::from(&secret_key));
		signed.to_bytes()
	}
}

impl From<PrivateKey> for [u8; 32] {
	fn from(key: PrivateKey) ->  Self {
		key.0
	}
}

impl From<PrivateKey> for PublicKey {
	fn from(key: PrivateKey) -> Self {
		let secret_key = SecretKey::from_bytes(&key.0).expect("Could not get secret from bytes.");
		let pk = ed25519_dalek::PublicKey::from(&secret_key);
		PublicKey(pk.to_bytes())
	}
}


#[cfg(test)]
mod tests {
    use super::*;
	use crate::types::Address;
	use crate::types::PublicKey;
	use crate::Raw;

	use std::str::FromStr;

    #[test]
	/**
	 * A not so random block on the chain to validate signing is working. This block
	 * was from a crypto puzzle where the private key is publicly known already.
	 *
	 * TODO: This test can also be the basis for writing the block hash function.
	{
	"block_account": "ban_3wtsduys8b7jkbfwwfzx3jgpgpsi9b8zurfe9bp1p5cdxkqiz7a5wxcoo7ba",
	"amount": "500000000000000000000000000000",
	"balance": "5500000000000000000000000000000",
	"height": "2",
	"local_timestamp": "1623383021",
	"confirmed": "true",
	"contents": {
		"type": "state",
		"account": "ban_3wtsduys8b7jkbfwwfzx3jgpgpsi9b8zurfe9bp1p5cdxkqiz7a5wxcoo7ba",
		"previous": "04BC2337372B048F838977A0608214253536C82F2895573A381B6070B8C1F279",
		"representative": "ban_1bananobh5rat99qfgt1ptpieie5swmoth87thi74qgbfrij7dcgjiij94xr",
		"balance": "5500000000000000000000000000000",
		"link": "AC9BFECE984B21EBC0663866050FFA110CDA3A3D8EFA65E08B22FCB2AC6FA08D",
		"link_as_account": "ban_3d6uzu9biks3xh18eg581n9zn6aeuax5u5qteqiapaqwpcp8za6fs6fddhjy",
		"signature": "12B1F6AB3A15C11C079791B1301731CBE7878DBFF8D81831675127B10DA2B683BF36BC886BF11A9DC53FF53311E0F11C022722A7B0284EE10699D3315E48140C",
		"work": "f0aabba178b4573e",
		"subtype": "receive"
	},
	"subtype": "receive",
	"pending": "0",
	"source_account": "ban_1m5ngqyrq58egujc3nz67wmtpay3t38bcuu5ymi7qddpzzznt1xii13ippu1",
	"timestamp": "1623383021655"
	}
	*/

	fn can_generate_valid_signature() {
		// use blake2::digest::{Update, VariableOutput};
		use hex_literal::hex;
		use hex as hex_parse;
		// What type of an attack uses the same id as another key on a public-key crypto system.
		let private_seed = Seed(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));
		let private_key = PrivateKey::from_seed(private_seed, 0);
		let expected_signature = String::from("12B1F6AB3A15C11C079791B1301731CBE7878DBFF8D81831675127B10DA2B683BF36BC886BF11A9DC53FF53311E0F11C022722A7B0284EE10699D3315E48140C");

		// The following comment out code has some issue not hashing correctly.  I'm not sure why.

		// let mut preamble = [0u8; 32];
		// preamble[31] = 6u8;


		// let block_account: [u8; 32] = Address::create_public_key(&String::from("ban_3wtsduys8b7jkbfwwfzx3jgpgpsi9b8zurfe9bp1p5cdxkqiz7a5wxcoo7ba"))
		// 					          .unwrap().into();
		// let previous =  hex!("04BC2337372B048F838977A0608214253536C82F2895573A381B6070B8C1F279");
		// let represenative: [u8; 32] = Address::create_public_key(&String::from("ban_1bananobh5rat99qfgt1ptpieie5swmoth87thi74qgbfrij7dcgjiij94xr"))
		// 							  .unwrap().into();
		// // Balance has to be padded to 32 characters
		// let balance : [u8; 16] = Raw::from_str("5500000000000000000000000000000").unwrap().to_u128().to_be_bytes();
		// let link =  hex!("AC9BFECE984B21EBC0663866050FFA110CDA3A3D8EFA65E08B22FCB2AC6FA08D");

		// let hasher = VarBlake2b::new(64).unwrap();

		// println!("{:?} {:?} {:?} {:?} {:?} {:?}", preamble, block_account, previous, represenative, balance, link);
		// hasher.update(preamble);
		// hasher.update(block_account);
		// hasher.update(previous);
		// hasher.update(represenative);
		// hasher.update(balance);
		// hasher.update(link);
		// let mut hash = [0u8; 64];

		/* This is the result of the command to a node for this block.  I do not know why I can't derive the hash correctly, something to solve more generally anyway.
		curl -H "Content-Type: application/json" --data '{ "action": "block_hash", "json_block": "true", "block": {
			"type": "state",
			"account": "ban_3wtsduys8b7jkbfwwfzx3jgpgpsi9b8zurfe9bp1p5cdxkqiz7a5wxcoo7ba",
			"previous": "04BC2337372B048F838977A0608214253536C82F2895573A381B6070B8C1F279",
			"representative": "ban_1bananobh5rat99qfgt1ptpieie5swmoth87thi74qgbfrij7dcgjiij94xr",
			"balance": "5500000000000000000000000000000",
			"link": "AC9BFECE984B21EBC0663866050FFA110CDA3A3D8EFA65E08B22FCB2AC6FA08D",
			"link_as_account": "ban_3d6uzu9biks3xh18eg581n9zn6aeuax5u5qteqiapaqwpcp8za6fs6fddhjy",
			"signature": "12B1F6AB3A15C11C079791B1301731CBE7878DBFF8D81831675127B10DA2B683BF36BC886BF11A9DC53FF53311E0F11C022722A7B0284EE10699D3315E48140C",
			"work": "f0aabba178b4573e",
			"subtype": "receive"
		}}' http://rimmer:7072


		{
			"hash": "938719851C7BAC4700B52FD0C0DD1CB4F47A1577B370F26616FA4D26B800B3E1"
		}
*/
		let hash = hex!("938719851C7BAC4700B52FD0C0DD1CB4F47A1577B370F26616FA4D26B800B3E1");
        //hasher.finalize_variable(|res| hash.copy_from_slice(res));
		// println!("HASH: {}", hex_parse::encode_upper(&hash));
		let signature = hex_parse::encode_upper(private_key.sign(hash));
		println!("{:?}", signature);
		assert_eq!(expected_signature, signature);
	}

	#[test]
	fn validate_private_key(){
		let private_seed = Seed::from("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef").expect("Couldn't create seed.");
		let private_key = PrivateKey::from_seed(private_seed, 0);
		let pubKey = PublicKey::from(private_key);
		assert_eq!(Address::from(pubKey).0, String::from("ban_3wtsduys8b7jkbfwwfzx3jgpgpsi9b8zurfe9bp1p5cdxkqiz7a5wxcoo7ba"));
	}
}
