use crate::types::{Address, Amount};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BananoError {
    #[error("Web3 provider error")]
    RpcError(#[from] reqwest::Error),
}

pub struct Banano {
    rpc_api: String,
}

impl Banano {
    pub fn new(rpc_api: String) -> Self {
        Banano {
            rpc_api: rpc_api,
        }
    }

    pub async fn account_balance(&self, wallet: &Address) -> Result<AccountBalance, BananoError> {
        let request = json!({
            "action": "account_balance",
            "account": wallet.0,
        });
        let account_balance: AccountBalance = Client::new()
            .post(self.rpc_api.clone())
            .json(&request)
            .send().await?
            .json().await?;
        Ok(account_balance)
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    balance: Amount,
    pending: Amount,
}

#[cfg(test)]
mod tests {
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
        assert_eq!(Decimal::from_str_radix("99".into(), 10).unwrap(), account_balance.balance.to_decimal());
        assert_eq!(Amount("0".into()), account_balance.pending);
    }
    
}