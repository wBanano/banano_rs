//! Banano Types

use serde::Deserialize;
use rust_decimal::Decimal;

/// Address
#[derive(Debug, Clone, PartialEq)]
pub struct Address(pub String);

/// Amount
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Amount(pub String);

impl Amount {
    pub fn to_decimal(&self) -> Decimal {
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