use crate::types::Amount;
use serde::Deserialize;

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
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_block_count)
#[derive(Debug, Deserialize)]
pub struct AccountBlockCount {
    pub block_count: u64,
}