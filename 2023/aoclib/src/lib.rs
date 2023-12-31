use std::io;
use std::num::ParseIntError;
use std::string::ParseError;

use thiserror::Error;

pub mod direction;
pub mod grid;
pub mod position;

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

impl From<ParseError> for AocError {
    fn from(e: ParseError) -> Self {
        AocError::ParseError(format!("{}", e))
    }
}

impl From<ParseIntError> for AocError {
    fn from(e: ParseIntError) -> Self {
        AocError::ParseError(format!("(Integer) {}", e))
    }
}

impl<I: std::fmt::Debug> From<nom::Err<nom::error::Error<I>>> for AocError {
    fn from(e: nom::Err<nom::error::Error<I>>) -> Self {
        AocError::ParseError(format!("(Nom) {}", e))
    }
}
