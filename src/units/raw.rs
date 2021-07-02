use super::{Banano, Bananoshi, UnboundedRaw};
use crate::encoding::{expect_len, to_hex};
use crate::Error;
use bigdecimal::BigDecimal;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

/// Special bounded container for the smallest unit, raw.
///
/// Can not contain values outside of `0` to [u128::MAX]. To get around this, use [UnboundedRaw] or
/// one of the other denominations: [Banano], [Bananoshi].
///
/// ```
/// use banano_rs::Raw;
///
/// fn main() -> anyhow::Result<()> {
/// use banano_rs::units::Banano;
///     let raw = Raw::new(100000000000000000000000000000u128);
///     assert_eq!(raw.to_banano(), Banano::new(1));
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Raw(pub(crate) u128);

impl Raw {
    pub(crate) const LEN: usize = 16;

    /// Create a new [Raw] instance. The value must be [Into<u128>]. This might change to something
    /// more flexible soon!
    pub fn new<T: Into<u128>>(v: T) -> Self {
        Self(v.into())
    }

    pub fn from_hex(s: &str) -> Result<Self, Error> {
        expect_len(s.len(), Raw::LEN * 2, "Hex raw")?;
        let vec = hex::decode(s.as_bytes()).map_err(|e| Error::FromHexError {
            msg: String::from("Decoding hex raw"),
            source: e,
        })?;
        Ok(Raw::try_from(vec.as_slice())?)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn max() -> Self {
        Self(u128::MAX)
    }

    pub fn to_banano(&self) -> Banano {
        Banano::from(self)
    }

    pub fn to_bananoshi(&self) -> Bananoshi {
        Bananoshi::from(self)
    }

    pub fn to_unbounded(&self) -> UnboundedRaw {
        UnboundedRaw::from(self)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn to_hex_string(&self) -> String {
        to_hex(self.0.to_be_bytes().as_ref())
    }

    pub fn to_u128(&self) -> u128 {
        self.0
    }

    pub fn to_big_decimal(&self) -> BigDecimal {
        // TODO: Don't know why from_u128() doesn't work.
        BigDecimal::from_str(&self.0.to_string()).unwrap()
    }

    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Raw::from)
    }

    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Raw::from)
    }
}

impl FromStr for Raw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u128::from_str(s)?))
    }
}

/// This serializer and deserializer are for strings with decimal numbers. See serialize_to_hex
/// and deserialize_from_hex if you expect your strings to be hex.
impl Serialize for Raw {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Raw {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(Raw::from_str(s).map_err(de::Error::custom)?)
    }
}

pub fn serialize_to_hex<S>(
    raw: &Raw,
    serializer: S,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
where
    S: Serializer,
{
    serializer.serialize_str(raw.to_hex_string().as_str())
}

pub fn deserialize_from_hex<'de, D>(deserializer: D) -> Result<Raw, <D as Deserializer<'de>>::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(Raw::from_hex(s).map_err(de::Error::custom)?)
}

impl Display for Raw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u128> for Raw {
    fn from(v: u128) -> Self {
        Raw(v)
    }
}

impl TryFrom<&[u8]> for Raw {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        expect_len(value.len(), Self::LEN, "Raw")?;
        let mut b = [0u8; 16];
        b.copy_from_slice(value);
        let amount = u128::from_be_bytes(b);
        Ok(Raw(amount))
    }
}

impl TryFrom<&BigDecimal> for Raw {
    type Error = Error;

    /// Convert from BigDecimal into Raw, removing any fraction.
    ///
    /// It's up to the caller to round to a whole number beforehand.
    ///
    /// One Raw is monetarily insignificant, but if you're using fractions and trying to encode
    /// data this might bite you!
    fn try_from(value: &BigDecimal) -> Result<Self, Self::Error> {
        // Remove decimals.
        let value = value.with_scale(0);
        // TODO: Don't use strings here.
        // TODO: from_u128 seems broken so we're using strings.
        Self::from_str(value.to_string().as_str())
    }
}

impl PartialOrd for Raw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

impl PartialOrd<u128> for Raw {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    fn lt(&self, other: &u128) -> bool {
        self.0.lt(other)
    }

    fn le(&self, other: &u128) -> bool {
        self.0.le(other)
    }

    fn gt(&self, other: &u128) -> bool {
        self.0.gt(other)
    }

    fn ge(&self, other: &u128) -> bool {
        self.0.ge(other)
    }
}

impl PartialEq<u128> for Raw {
    fn eq(&self, other: &u128) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn display() {
        assert_eq!(Raw::zero().to_string(), "0");
        assert_eq!(
            Raw::from_str("98765432100123456789").unwrap().to_string(),
            "98765432100123456789"
        );
    }

    #[test]
    fn convert_from_raw() {
        let one_raw = Raw::from(1u128);
        assert_eq!(one_raw.to_string(), "1");
        assert_eq!(one_raw.to_banano().to_string(), "0.00000000000000000000000000001");

        assert_eq!(
            Banano::new(1).to_raw().unwrap(),
            Raw::from_str("100000000000000000000000000000").unwrap()
        );

        assert_eq!(
            Bananoshi::new(1).to_raw().unwrap(),
            Raw::from_str("1000000000000000000000000000").unwrap()
        );

        let max_raw = Raw::from_str("340282366920938463463374607431768211455").unwrap();
        assert_eq!(
            max_raw.to_string(),
            "340282366920938463463374607431768211455"
        );
        assert_eq!(
            max_raw.to_banano().to_string(),
            "3402823669.20938463463374607431768211455"
        );
    }

    #[test]
    fn convert_to_raw() {
        assert_eq!(
            Banano::new(1).to_raw().unwrap().to_string(),
            "100000000000000000000000000000"
        );
    }

    #[test]
    fn eq() {
        assert_eq!(
            Banano::new(1).to_raw().unwrap(),
            Raw::new(100000000000000000000000000000u128)
        );
    }

    #[test]
    fn serialize() {
        let raw1 = Banano::new(1).to_raw().unwrap();
        let bytes = raw1.to_vec();
        let raw2 = Raw::try_from(bytes.as_slice()).unwrap();
        assert_eq!(raw1, raw2);
    }

    #[test]
    fn decimal_json() -> anyhow::Result<()> {
        let raw = Banano::new(1).to_raw().unwrap();
        let json = serde_json::to_string(&raw).unwrap();
        assert_eq!(json, r#""100000000000000000000000000000""#);
        assert_eq!(serde_json::from_str::<Raw>(&json)?, raw);
        Ok(())
    }

    #[test]
    fn negative_unbounded() {
        let mut v = Raw::zero().to_unbounded();
        v -= UnboundedRaw::new(1);
    }
}
