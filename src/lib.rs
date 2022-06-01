mod bucket;
mod error;
mod lexorank;
mod value;

pub use crate::lexorank::LexoRank;
pub use bucket::LexBucket;
use error::ParseError;
pub use value::LexValue;

type ParseResult<T> = std::result::Result<T, ParseError>;
