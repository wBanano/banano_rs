use crate::hexify;
use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use anyhow::anyhow;
use std::{convert::TryFrom, str::FromStr};

hexify!(BlockHash, 32, "Block hash", "40DB7EC1F71F7B3B66982007F20E687148BDB875E533121259C0BF69AEFE88D3");

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BlockType {
    Invalid,
    NotABlock,
    Send,
    Receive,
    Open,
    Change,
    State,
}

impl BlockType {
    pub fn as_u8(&self) -> u8 {
        match self {
            BlockType::Invalid => 0,
            BlockType::NotABlock => 1,
            BlockType::Send => 2,
            BlockType::Receive => 3,
            BlockType::Open => 4,
            BlockType::Change => 5,
            BlockType::State => 6,
        }
    }
}

impl TryFrom<u8> for BlockType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use BlockType::*;
        Ok(match value {
            0 => Invalid,
            1 => NotABlock,
            2 => Send,
            3 => Receive,
            4 => Open,
            5 => Change,
            6 => State,
            _ => return Err(anyhow!("Invalid block type: {}", value)),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Previous {
    Block(BlockHash),
    Open,
}

impl Previous {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Previous::Block(b) => b.as_bytes().to_vec(),
            Previous::Open => BlockHash::zero().as_bytes().to_vec(),
        }
    }
}

impl TryFrom<&[u8]> for Previous {
    type Error = crate::Error;

    fn try_from(slice_of_bytes: &[u8]) -> crate::Result<Self> {
        if slice_of_bytes.iter().all(|&b| b == 0) {
            Ok(Previous::Open)
        } else {
            Ok(Previous::Block(BlockHash::try_from(slice_of_bytes)?))
        }
    }
}

impl FromStr for Previous {
    type Err = crate::Error;

    fn from_str(hex_string: &str) -> crate::Result<Self> {
        if hex_string == "0".repeat(64) {
            Ok(Previous::Open)
        } else {
            Ok(Previous::Block(BlockHash::from_str(hex_string)?))
        }
    }
}
