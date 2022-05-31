mod error;
mod tests;

use error::ParseError;
use helpers::{decrement_char, increment_char};
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
        for (i, c) in self.0.chars().rev().enumerate() {
            let i = self.0.len() - i - 1;

            if c == 'z' {
                continue;
            };

            let next_c = increment_char(&c).expect(&format!("failed to increment char '{}'", c));
            let new_value = self.0.substring(0, i).to_owned() + &next_c.to_string();
            return LexValue(new_value);
        }

        let new_value = self.0.to_owned() + "1";
        return LexValue(new_value);
    }

    pub fn prev(&self) -> Self {
        let length = self.0.len();
        let c = self.0.chars().rev().nth(0).unwrap();

        if c != '1' {
            let new_value = self.0.substring(0, length - 1).to_owned()
                + &decrement_char(&c)
                    .expect(&format!("failed to decrement char '{}'", c))
                    .to_string();

            return LexValue(new_value);
        }

        for (i, c) in self.0.chars().rev().enumerate() {
            // Skip last char which is '1'
            if i == 0 {
                continue;
            }

            // Use correct string index
            let i = self.0.len() - i - 1;

            // Must have non-zero leading chars in order to truncate
            if c == '0' {
                continue;
            };

            let new_value = self.0.substring(0, i + 1).to_owned();
            return LexValue(new_value);
        }

        // Can't decrement char or truncate so slap a 0 on the front
        let new_value = "0".to_owned() + &self.0;
        return LexValue(new_value);
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

    pub fn next(&self) -> Self {
        LexoRank::new(self.bucket, self.rank.next())
    }

    pub fn prev(&self) -> Self {
        LexoRank::new(self.bucket, self.rank.prev())
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

mod helpers {
    pub fn increment_char(c: &char) -> Option<char> {
        if c == &'z' {
            None
        } else if c == &'9' {
            Some('a')
        } else {
            Some(char::from_u32(*c as u32 + 1).unwrap())
        }
    }

    pub fn decrement_char(c: &char) -> Option<char> {
        if c == &'0' {
            None
        } else if c == &'a' {
            Some('9')
        } else {
            Some(char::from_u32(*c as u32 - 1).unwrap())
        }
    }
}
