use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ParseError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError => f.write_str("Error occured during parsing."),
        }
    }
}

impl std::error::Error for Error {}
