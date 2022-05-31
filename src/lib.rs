mod error;
mod tests;

use error::ParseError;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, result};
use substring::Substring;

type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexBucket(u8);

impl LexBucket {
    pub fn new(value: u8) -> Result<Self> {
        if value > 2 {
            Err(ParseError(format!(
                "LexoRank bucket value must be between 0 and 2 inclusive. Found: {}",
                value
            )))
        } else {
            Ok(LexBucket(value))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn next(&self) -> Self {
        if self.0 == 2 {
            LexBucket(0)
        } else {
            LexBucket(self.0 + 1)
        }
    }

    pub fn prev(&self) -> Self {
        if self.0 == 0 {
            LexBucket(2)
        } else {
            LexBucket(self.0 - 1)
        }
    }
}

impl TryFrom<u8> for LexBucket {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self> {
        LexBucket::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexValue(String);

impl LexValue {
    pub fn new(value: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9a-z]*[1-9a-z]$").unwrap();
        }

        if !RE.is_match(value) {
            Err(ParseError(format!(
                "Lexorank value must only include 0-9 and a-z and must not end with 0. Found: {}",
                value,
            )))
        } else {
            Ok(LexValue(value.to_owned()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn next(&self) -> Self {
        todo!()
    }

    pub fn prev(&self) -> Self {
        todo!()
    }
}

impl TryFrom<&str> for LexValue {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self> {
        LexValue::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexoRank {
    bucket: LexBucket,
    rank: LexValue,
}

impl LexoRank {
    pub fn new(bucket: LexBucket, rank: LexValue) -> Self {
        LexoRank { bucket, rank }
    }

    pub fn from_string(value: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-2]\|[0-9a-z]*[1-9a-z]$").unwrap();
        }

        if !RE.is_match(value) {
            Err(ParseError(format!(
                "Cannot create LexoRank from invalid string. Found: {}",
                value,
            )))
        } else {
            let bucket = value.substring(0, 1).parse().unwrap();
            let rank = value.substring(2, value.len());
            Ok(LexoRank::new(LexBucket(bucket), LexValue(rank.to_owned())))
        }
    }

    pub fn bucket(&self) -> &LexBucket {
        &self.bucket
    }

    pub fn rank(&self) -> &LexValue {
        &self.rank
    }
}

impl TryFrom<&str> for LexoRank {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self> {
        LexoRank::from_string(value)
    }
}

impl fmt::Display for LexoRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}|{}", self.bucket.value(), self.rank.value())
    }
}
