use crate::types::Amount;
use serde::Deserialize;
use serde_aux::prelude::*;

type BlockCount = u128;

/// Account balance
///
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_balance)
#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    /// Balance amount
    pub balance: Amount,
    /// Pending amount
    pub pending: Amount,
}

/// Account block count
///
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_block_count)
#[derive(Debug, Deserialize)]
pub struct AccountBlockCount {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub block_count: BlockCount,
}
