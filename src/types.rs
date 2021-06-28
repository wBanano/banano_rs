//! Banano Types

use serde::Deserialize;
use rust_decimal::Decimal;

/// Address
#[derive(Debug, Clone, PartialEq)]
pub struct Address(pub String);

#[derive(Debug, Clone, Deserialize, PartialEq)]
/// Amount
///
/// `Amount` is by default a [String](String) having the raw value.
///
/// # Example:
/// ```
/// let amount = banano_rs::types::Amount("9900000000000000000000000000000".into());
/// ```
///
pub struct Amount(pub String);

impl Amount {
    /// Return the amount as a [Decimal](Decimal), ignoring raw.
    ///
    /// # Example:
    /// ```
    /// let amount_raw = banano_rs::types::Amount("9900000000000000000000000000000".into());
    /// let bananos = amount_raw.as_banano();
    /// ```
    pub fn as_banano(&self) -> Decimal {
        if self.0 == "0" {
            return Decimal::from(0)
        }
        
        let mut balance: String = self.0.clone();
        balance.truncate(balance.len() - 11);

        let mut balance: Decimal = Decimal::from_str_radix(balance.as_str(), 10).unwrap();
        balance.set_scale(18).unwrap();

        balance
    }
}
