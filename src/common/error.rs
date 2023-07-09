/// An error service whenever parser encounters certain discrepancies.
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Raised whenever a certain token is not accepted
    UnexpectedTokenError(char),
    /// Raised whenever parser reaches the end of the
    /// buffer without proper handling, but might allow
    /// creating the object even after failure.
    EndOfBufferError,
    /// On Parsing Object, Array, or Set, raises an error when
    /// parathesis are mismatched
    ContainerParanthesisMismatchError {
        opening_container: char,
        closing_container: char,
    },
    /// Invalid key value formatting, while reading key
    InvalidKeyValueFormatError { reading_key: String },
    /// Invalid token while parsing number
    InvalidNumberParseError(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedTokenError(chr) => {
                f.write_str(format!("Unexpected character found: {}", chr).as_str())
            }
            ParseError::ContainerParanthesisMismatchError {
                opening_container,
                closing_container,
            } => f.write_str(
                format!(
                    "The opening bracket '{}' and closing bracket '{}' do not match",
                    opening_container, closing_container
                )
                .as_str(),
            ),
            ParseError::InvalidKeyValueFormatError { reading_key } => f.write_str(
                format!(
                    "Error while reading value while reading key: {}",
                    reading_key
                )
                .as_str(),
            ),
            ParseError::InvalidNumberParseError(invalid_char) => f.write_str(
                format!(
                    "Error while reading number: found character {}",
                    invalid_char
                )
                .as_str(),
            ),
            ParseError::EndOfBufferError => {
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
    ParsingError(ParseError),
    /// Raised when the command does not exist/implemented in the
    /// application
    CommandNotFoundError(String),
    /// Argument error
    ///
    /// Todo: print help along with the error.
    ArgError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::ParsingError(ref error_value) => {
                f.write_str(format!("\x1b[1;31mParse Error\x1b[0m:\n{}", error_value).as_str())
            }
            Error::CommandNotFoundError(ref value) => f.write_str(
                format!(
                    "\x1b[1;31mError\x1b[0m: no command named '{}' exists",
                    value
                )
                .as_str(),
            ),
            Error::ArgError(ref value) => f.write_str(
                format!(
                    "\x1b[1;31mArgument Error\x1b[0m: invalid argument '{}'",
                    value
                )
                .as_str(),
            ),
        }
    }
}
