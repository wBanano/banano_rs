use crate::{Address, types::BlockHash, units::Raw};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::serde_as;

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

/// Account Info
///
/// [Nano documentation](https://docs.nano.org/commands/rpc-protocol/#account_info)
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct AccountInfo {
	//#[serde(with = "serde_with::rust::display_fromstr")]
    pub frontier: BlockHash,
	//#[serde(with = "serde_with::rust::display_fromstr")]
	pub open_block: BlockHash,
	//#[serde(with = "serde_with::rust::display_fromstr")]
	pub representative_block: BlockHash,
	/// Only available when the `representative` field is set to `true` in the request
	pub representative: Option<Address>,
	pub balance: Raw,
	#[serde_as(as = "serde_with::TimestampSeconds<String>")]
	pub modified_timestamp: DateTime<Utc>,
	#[serde(with = "serde_with::rust::display_fromstr")]
	pub block_count: BlockCount,
	#[serde(with = "serde_with::rust::display_fromstr")]
	pub account_version: u8,
	#[serde(with = "serde_with::rust::display_fromstr")]
	pub confirmation_height: u128,
	/// Only available for version 21.0+ which is the block hash at that confirmation height
	pub confirmation_height_frontier: Option<String>,
}
