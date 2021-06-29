//! Banano API
//!
//! # Example:
//! ```
//! use banano_rs::{
//!   api::Banano,
//!   errors::BananoError,
//!   types::Address 
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let banano = Banano::new("https://kaliumapi.appditto.com/api".into());
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
pub struct Banano {
    rpc_api: String,
}

impl Banano {
    /// Instanciate Banano API using a RPC API URL
    ///
    /// # Example:
    /// ```
    /// let banano = banano_rs::api::Banano::new("https://kaliumapi.appditto.com/api".into());
    /// ```
    pub fn new(rpc_api: String) -> Self {
        Banano {
            rpc_api: rpc_api,
        }
    }

    /// Returns how many RAW is owned and how many have not yet been received by `account`
    pub async fn account_balance(&self, account: &Address) -> Result<AccountBalance, crate::errors::BananoError> {
        let request = json!({
            "action": "account_balance",
            "account": account.0,
        });
        let account_balance: AccountBalance = Client::new()
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_balance)
    }

    /// Get number of blocks for a specific `account`
    pub async fn account_block_count(&self, account: &Address) -> Result<AccountBlockCount, crate::errors::BananoError> {
        let request = json!({
            "action": "account_block_count",
            "account": account.0,
        });
        let account_block: AccountBlockCount = Client::new()
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_block)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Amount;
    use rust_decimal::Decimal;
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn account_balance() {
        let banano = Banano::new("https://kaliumapi.appditto.com/api".into());
        let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
        let account_balance = aw!(banano.account_balance(&address)).unwrap();
        assert_eq!(Amount("9900000000000000000000000000000".into()), account_balance.balance);
        assert_eq!(Decimal::from_str_radix("99".into(), 10).unwrap(), account_balance.balance.as_banano());
        assert_eq!(Amount("0".into()), account_balance.pending);
    }

    #[test]
    fn account_block_count() {
        let banano = Banano::new("https://kaliumapi.appditto.com/api".into());
        let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
        let account_block = aw!(banano.account_block_count(&address)).unwrap();
        assert_eq!(4, account_block.block_count);
    }
}