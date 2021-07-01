use super::privkey::PrivateKey;
use blake2::{Blake2b, Digest};
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
use curve25519_dalek::scalar::Scalar as CurveScaler;

#[derive(Debug, Clone)]
pub struct PublicKey(pub [u8; 32]);

impl From<PrivateKey> for PublicKey {
	fn from(key: PrivateKey) -> Self {
                let mut bhasher = Blake2b::default();
                let bkey: [u8; 32] = key.into();
                bhasher.update(&bkey);
                let hresult = bhasher.finalize();
                let mut scaler = [0u8; 32];
                scaler.copy_from_slice(&hresult.as_slice()[..32]);
                scaler[0] &= 248;
                scaler[31] &= 63;
                scaler[31] |= 64;
                let cscaler = CurveScaler::from_bits(scaler);
                let point = &cscaler * &ED25519_BASEPOINT_TABLE;
                PublicKey(point.compress().to_bytes())
	}
}

impl From<PublicKey> for[u8; 32] {
	fn from(key: PublicKey) -> Self {
                key.0
	}
}