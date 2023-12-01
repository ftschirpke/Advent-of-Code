use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Math Error: {0}")]
    MathError(String),
    #[error("Logic Error: {0}")]
    LogicError(String),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown error")]
    UnknownError,
}
