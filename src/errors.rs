use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ArithmeticError {
    Empty,
    IncompleteParsing(String, usize),
    InvalidToken(String),
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticError::Empty => write!(f, "Parsed input was empty."),
            ArithmeticError::IncompleteParsing(input, last_position) => {
                writeln!(f, "{}", input)?;
                write!(f, "{}^", "-".repeat(*last_position + 1))
            }
            ArithmeticError::InvalidToken(token) => write!(f, "Invalid token: {token}"),
        }
    }
}

impl Error for ArithmeticError {}
