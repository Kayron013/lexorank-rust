mod bucket;
pub mod error;
mod lexorank;
mod rank;

pub use crate::lexorank::LexoRank;
pub use bucket::Bucket;
use error::ParseError;
pub use rank::Rank;

type ParseResult<T> = std::result::Result<T, ParseError>;
