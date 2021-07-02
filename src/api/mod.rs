//! Banano API
//!
//! # Example:
//! ```
//! use banano_rs::{
//!   BananoApi,
//!   Address
//! };
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let banano = BananoApi::new("https://kaliumapi.appditto.com/api".into());
//!     let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
//!     let account_balance = banano.account_balance(&address).await?;
//!		Ok(())
//! }
//! ```

use crate::{Error, types::Address};
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
    pub async fn account_balance(&self, account: &Address) -> Result<AccountBalance, Error> {
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

	/// Returns frontier, open block, change representative block, balance, last modified timestamp from local database & block count for account.
	/// Only works for accounts that have received their first transaction and have an entry on the ledger, will return "Account not found"
	/// otherwise. To open an account, use `receive`.
    pub async fn account_info(&self, account: &Address) -> Result<AccountInfo, crate::errors::Error> {
        let request = json!({
            "action": "account_info",
            "account": account.0,
			"representative": true,
        });
        let account_info: AccountInfo = self.client
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_info)
    }
}

#[cfg(test)]
mod tests {
    use crate::Raw;
    use crate::types::BlockHash;
    use crate::units::Banano;
    use super::*;
	use chrono::*;
	use std::str::FromStr;

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

	#[test]
    fn account_info() {
        let banano = BananoApi::new("https://kaliumapi.appditto.com/api".into());
        let address = Address("ban_1hgtqu7cmgxb66ta4gxt7coimqcxp86nzi5b7u14ip9zzpqr16a3dbqdja1f".into());
        let account_info = aw!(banano.account_info(&address)).unwrap();

        let expected_balance = Banano::new(99).to_raw().unwrap();
        assert!(account_info.balance.eq(&expected_balance));
		assert_eq!(Address("ban_1fomoz167m7o38gw4rzt7hz67oq6itejpt4yocrfywujbpatd711cjew8gjj".into()), account_info.representative.unwrap());
		assert_eq!(Utc.ymd(2021, 6, 21), account_info.modified_timestamp.date());
		let expected_representative_block = BlockHash::from_str("40DB7EC1F71F7B3B66982007F20E687148BDB875E533121259C0BF69AEFE88D3").unwrap();
		assert_eq!(expected_representative_block, account_info.representative_block);
    }

}
