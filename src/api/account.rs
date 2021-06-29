use crate::units::Raw;
use serde::Deserialize;

type BlockCount = u128;

/// Account balance
///
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_balance)
#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    /// Balance amount
    pub balance: Raw,
    /// Pending amount
    pub pending: Raw,
}

/// Account block count
///
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_block_count)
#[derive(Debug, Deserialize)]
pub struct AccountBlockCount {
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub block_count: BlockCount,
}
