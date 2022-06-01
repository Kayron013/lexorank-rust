use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexBucket(u8);

impl LexBucket {
    pub fn new(value: u8) -> ParseResult<Self> {
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

    fn try_from(value: u8) -> ParseResult<Self> {
        LexBucket::new(value)
    }
}
