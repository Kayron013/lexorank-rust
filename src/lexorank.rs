use super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexoRank {
    bucket: LexBucket,
    rank: LexValue,
}

impl LexoRank {
    pub fn new(bucket: LexBucket, rank: LexValue) -> Self {
        LexoRank { bucket, rank }
    }

    pub fn from_string(value: &str) -> ParseResult<Self> {
        let parts = value.split('|').collect::<Vec<&str>>();
        let bucket = LexBucket::new(parts[0].parse::<u8>()?)?;
        let rank = LexValue::new(parts[1])?;

        Ok(LexoRank::new(bucket, rank))
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

    pub fn between(&self, rank2: &Self) -> Option<Self> {
        self.rank
            .between(&rank2.rank)
            .map(|rank| LexoRank::new(self.bucket, rank))
    }
}

impl TryFrom<&str> for LexoRank {
    type Error = ParseError;

    fn try_from(value: &str) -> ParseResult<Self> {
        LexoRank::from_string(value)
    }
}

impl fmt::Display for LexoRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}|{}", self.bucket.value(), self.rank.value())
    }
}
