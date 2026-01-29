use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    InvalidToken(char),
    InvalidLiteral,
    LiteralOutOfRange,
    RangeError,
}

pub type CalcResult<T> = Result<T, CalcError>;

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidToken(ch) => write!(f, "invalid token '{ch}'"),
            CalcError::InvalidLiteral => write!(f, "invalid literal"),
            CalcError::LiteralOutOfRange => write!(f, "literal out of range"),
            CalcError::RangeError => write!(f, "range error"),
        }
    }
}

impl std::error::Error for CalcError {}
