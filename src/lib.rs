mod bucket;
mod error;
mod lexorank;
mod value;

pub use bucket::LexBucket;
use error::ParseError;
pub use lexorank::LexoRank;
pub use value::LexValue;

type Result<T> = std::result::Result<T, ParseError>;
