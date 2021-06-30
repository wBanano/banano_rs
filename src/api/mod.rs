//! Banano API
//!
//! # Example:
//! ```
//! use banano_rs::{
//!   BananoApi,
//!   Error,
//!   Address 
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let banano = BananoApi::new("https://kaliumapi.appditto.com/api".into());
//!     let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
//!     let account_balance = banano.account_balance(&address).await.unwrap();
//! }
//! ```

use crate::types::{Address};
pub use self::account::*;
use reqwest::Client;
use serde_json::json;

mod account;

/// Banano API
pub struct BananoApi {
    rpc_api: String,
    client: Client,
}

impl BananoApi {
    /// Instanciate Banano API using a RPC API URL
    ///
    /// # Example:
    /// ```
    /// let banano = banano_rs::BananoApi::new("https://kaliumapi.appditto.com/api".into());
    /// ```
    pub fn new(rpc_api: String) -> Self {
        BananoApi {
            rpc_api: rpc_api,
            client: Client::new(),
        }
    }

    /// Returns how many RAW is owned and how many have not yet been received by `account`
    pub async fn account_balance(&self, account: &Address) -> Result<AccountBalance, crate::errors::Error> {
        let request = json!({
            "action": "account_balance",
            "account": account.0,
        });
        let account_balance: AccountBalance = self.client
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_balance)
    }

    /// Get number of blocks for a specific `account`
    pub async fn account_block_count(&self, account: &Address) -> Result<AccountBlockCount, crate::errors::Error> {
        let request = json!({
            "action": "account_block_count",
            "account": account.0,
        });
        let account_block: AccountBlockCount = self.client
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_block)
    }
}

#[cfg(test)]
mod tests {
    use crate::Raw;
    use crate::units::Banano;
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn account_balance() {
        let banano = BananoApi::new("https://kaliumapi.appditto.com/api".into());
        let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
        let account_balance = aw!(banano.account_balance(&address)).unwrap();

        let expected_balance = Banano::new(99).to_raw().unwrap();
        let expected_pending = Raw::zero();
        println!("Expected: {}, Got: {}", expected_balance, account_balance.balance);
        assert!(account_balance.balance.eq(&expected_balance));
        assert!(account_balance.pending.eq(&expected_pending));
    }

    #[test]
    fn account_block_count() {
        let banano = BananoApi::new("https://kaliumapi.appditto.com/api".into());
        let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
        let account_block = aw!(banano.account_block_count(&address)).unwrap();
        assert_eq!(4, account_block.block_count);
    }
}