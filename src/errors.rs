//! Errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BananoError {
    #[error("Web3 provider error")]
    RpcError(#[from] reqwest::Error),
    #[error("Decoding error")]
    DecodeError(#[from] data_encoding::DecodeError),
    #[error("PublicKey error")]
    PublicKeyError(#[from] ed25519_dalek::ed25519::Error),
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("Invalid Address length")]
    InvalidAddressLength(usize),
    #[error("Invalid Seed length")]
    SeedLengthError(usize),
}