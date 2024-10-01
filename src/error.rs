/// An error service whenever parser encounters certain discrepancies.
#[derive(Debug, Clone)]
#[allow(unused)]
pub enum ParseError {
    /// Raised whenever a certain token is not accepted
    UnexpectedToken(char, usize, usize),
    /// Invalid UTF-8 character
    InvalidUTF8Parsing,
    /// Nested Depth Exceeded
    NestedDepthExceeded(u16),
    /// Raised whenever parser reaches the end of the
    /// buffer without proper handling, but might allow
    /// creating the object even after failure.
    EndOfBuffer,
    /// On Parsing Object, Array, or Set, raises an error when
    /// parathesis are mismatched
    ContainerParanthesisMismatch {
        opening_container: char,
        closing_container: char,
    },
    /// Invalid key value formatting, while reading key
    InvalidKeyValueFormat { reading_key: String },
    /// Invalid token while parsing number
    InvalidNumberParse(char),
}

impl core::error::Error for ParseError {}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(chr, line, col) => f.write_str(
                format!(
                    "Unexpected character found: {} at line {}, col: {}",
                    chr, line, col
                )
                .as_str(),
            ),
            ParseError::NestedDepthExceeded(c) => f.write_str(format!("NestedDepthExceeded, >{c}").as_str()),
            ParseError::InvalidUTF8Parsing => f.write_str("Invalid UTF-8 Value found while decoding strings."),
            ParseError::ContainerParanthesisMismatch {
                opening_container,
                closing_container,
            } => f.write_str(
                format!(
                    "The opening bracket '{}' and closing bracket '{}' do not match",
                    opening_container, closing_container
                )
                .as_str(),
            ),
            ParseError::InvalidKeyValueFormat { reading_key } => f.write_str(
                format!(
                    "Error while reading value while reading key: {}",
                    reading_key
                )
                .as_str(),
            ),
            ParseError::InvalidNumberParse(invalid_char) => f.write_str(
                format!(
                    "Error while reading number: found character {}",
                    invalid_char
                )
                .as_str(),
            ),
            ParseError::EndOfBuffer => {
                f.write_str("The buffer ended before operating on storage.")
            }
        }
    }
}

/// This is a method to handle errors that are generated throughout
/// the session.
#[derive(Debug, Clone)]
pub enum Error {
    /// Raised whenever the errors are raised are
    /// related to parsing
    Parsing(ParseError),
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Parsing(ref error_value) => f.write_str(
                format!("\x1b[1;31mParse Error\x1b[0m:\n{}", error_value)
                    .as_str(),
            ),
        }
    }
}
