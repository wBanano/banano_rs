//! Banano Types

use data_encoding::{Encoding};
use data_encoding_macro::new_encoding;
use blake2::{
    VarBlake2b,
    digest::{Update, VariableOutput},
};
pub use self::seed::Seed;
pub use self::address::Address;
pub use self::privkey::PrivateKey;
pub use self::pubkey::PublicKey;
pub use self::account::Account;

mod seed;
mod address;
mod privkey;
mod pubkey;
mod account;

const BAN_ENCODING: Encoding = new_encoding! {
	symbols: "13456789abcdefghijkmnopqrstuwxyz",
	check_trailing_bits: false,
};

// the address checksum is the 5 byte hash of the public key reversed
pub(crate) fn compute_address_checksum(key_bytes: &[u8]) -> [u8; 5] {
    let mut hasher = VarBlake2b::new(5).unwrap();
	let mut buf = [0u8; 5];
	hasher.update(key_bytes);
    hasher.finalize_variable(|res| buf.copy_from_slice(res));
	buf.reverse();
	buf
}

#[cfg(test)]
mod tests {
    use data_encoding::HEXLOWER_PERMISSIVE;
    // use hex_literal::hex;
    // use ed25519_dalek::PublicKey;
    use super::*;

    #[test]
	fn can_generate_address_from_seed() {
		let seed = Seed::from("1234567890123456789012345678901234567890123456789012345678901234").unwrap();

		// shamelessly copied from https://github.com/frankh/nano/blob/078a99b8e75bd239e13565312e06258164a781d5/address/address_test.go#L55-L59
		let expected_output = vec![
			"ban_3iwi45me3cgo9aza9wx5f7rder37hw11xtc1ek8psqxw5oxb8cujjad6qp9y",
			"ban_3a9d1h6wt3zp8cqd6dhhgoyizmk1ciemqkrw97ysrphn7anm6xko1wxakaa1",
			"ban_1dz36wby1azyjgh7t9nopjm3k5rduhmntercoz545my9s8nm7gcuthuq9fmq",
			"ban_1fb7kaqaue49kf9w4mb9w3scuxipbdm3ez6ibnri4w8qexzg5f4r7on1dmxb",
			"ban_3h9a64yqueuij1j9odt119r3ymm8n83wyyz7o9u7ram1tgfhsh1zqwjtzid9",
		];

		expected_output.into_iter().enumerate().for_each(|(index, address)| {
			let priv_key = PrivateKey::from_seed(seed.clone(), index as u32);
			let account: Account = priv_key.into();

			assert_eq!(account.address.0, address)
		})
	}

    #[test]
	fn can_convert_address_to_public_key() {
		let addr = Address("ban_3t6k35gi95xu6tergt6p69ck76ogmitsa8mnijtpxm9fkcm736xtoncuohr3".into());
		let public_key = addr.to_public_key().unwrap();
		let p_key_str = HEXLOWER_PERMISSIVE.encode(&public_key.0[..]);
		// shamelessly copied from https://github.com/frankh/nano/blob/078a99b8e75bd239e13565312e06258164a781d5/address/address_test.go#L28-L30
		assert_eq!(p_key_str, "e89208dd038fbb269987689621d52292ae9c35941a7484756ecced92a65093ba")
	}

    #[test]
	fn can_validate_addresses() {
		let addresses = vec![
			"ban_38nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r8",
			"ban_1awsn43we17c1oshdru4azeqjz9wii41dy8npubm4rg11so7dx3jtqgoeahy",
			"ban_3arg3asgtigae3xckabaaewkx3bzsh7nwz7jkmjos79ihyaxwphhm6qgjps4",
			"ban_3pczxuorp48td8645bs3m6c3xotxd3idskrenmi65rbrga5zmkemzhwkaznh",
			"ban_3hd4ezdgsp15iemx7h81in7xz5tpxi43b6b41zn3qmwiuypankocw3awes5k",
			"ban_1anrzcuwe64rwxzcco8dkhpyxpi8kd7zsjc1oeimpc3ppca4mrjtwnqposrs",
		];

		addresses.into_iter().for_each(|addr| {
			Address(addr.into()).to_public_key().expect("Couldn't Validate Address");
		})
	}

    #[test]
	fn can_invalidate_addresses() {
		let addresses = vec![
			"ban_38nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r7",
			"bam_38nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r8",
			"ban38nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r8",
			"ban8nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r8",
			"ban_8nm8t5rimw6h6j7wyokbs8jiygzs7baoha4pqzhfw1k79npyr1km8w6y7r8",
		];

		let output = addresses.into_iter().map(|addr| {
			Address(addr.into()).to_public_key().is_err()
		}).collect::<Vec<_>>();

		assert_eq!(output, vec![true, true, true, true, true])
	}
    
}