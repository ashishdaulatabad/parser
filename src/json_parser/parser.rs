use crate::common::container::Container;
use crate::common::error::Error;
use crate::common::error::ParseError;
use core::result::Result;

/// Single-threaded parsing module, with an intent to parse the
/// files faster with handling run-time errors (hopefully), considering two modes
/// of parsing:

/// - JSON parsing
/// - Binary Data Parsing (where integers are of fixed 4 bytes)
///
/// Main instance of Parser.
///
/// This is invoked when a user requests loading into memory, called via
/// function `parse_str`
pub struct Parser {
    /// Raw pointer for the actual input
    container: *const u8,
    /// Current byte that the Parser is reading
    curr_byte: u8,
    /// For parsing the file, counting offset
    offset: usize,
    /// Current line: measured by counting \n in the files
    curr_line: usize,
    /// Column number: to encounter error
    curr_column: usize,
    /// Length of the container.
    len: usize,
    /// Adjustment when a certain number is read.
    num_read: bool,
}

macro_rules! read_byte {
    ($parser:ident) => {{
        $parser.get_next_byte()
    }};
}

macro_rules! expect_next_bytes {
    ($parser:ident, $( $next_char:expr ),*) => ({
        $(
            match read_byte!($parser) {
                Some($next_char) => { }
                val @ _ => {
                    return Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                        if val.is_some() { val.unwrap() } else { b'\0' } as char,
                        $parser.curr_line,
                        $parser.curr_column
                    )));
                }
            }
        )*
    })
}

macro_rules! skip_whitespaces {
    ($parser:ident) => {{
        while let Some(b' ' | 9..=13) = Some($parser.curr_byte) {
            read_byte!($parser);
        }
    }};
}

macro_rules! assert_curr_byte {
    ($parser:ident, $byte_val:expr) => {
        if $parser.curr_byte != $byte_val {
            return Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                $parser.curr_byte as char,
                $parser.curr_line,
                $parser.curr_column,
            )));
        }
    };
}

macro_rules! equals_in {
    ($compare: expr, $( $char: expr ),*) => {
        match $compare {
            $ ( $char )|* => true,
            _ => false
        }
    };
}

macro_rules! get_closing_container {
    ($chr:expr) => {
        match $chr {
            b'[' => b']',
            b'{' => b'}',
            b'(' => b')',
            _ => b'\0',
        }
    };
}

impl Parser {
    /// Creates a new JSON parser.
    #[inline(always)]
    fn new(str_stream: &str) -> Self {
        Self {
            container: str_stream.as_ptr(),
            curr_byte: b' ',
            offset: 0,
            curr_line: 1,
            curr_column: 1,
            len: str_stream.len(),
            num_read: false,
        }
    }

    /// Get the next byte from the buffer string
    /// Returns none if length exceeds the length of buffer,
    ///
    /// Returns `Option<u8>`.
    #[inline]
    fn get_next_byte(&mut self) -> Option<u8> {
        if self.offset < self.len {
            unsafe {
                self.curr_byte = *self.container.add(self.offset);
            }
            self.offset += 1;
            if self.curr_byte == b'\n' {
                self.curr_line += 1;
                self.curr_column = 0;
            } else {
                self.curr_column += 1;
            }
            Some(self.curr_byte)
        } else {
            None
        }
    }

    /// Parsing bytestream
    /// Parse the file from an input stream: taking unsafe route
    #[inline(always)]
    pub fn parse_str(&mut self) -> Result<Container, Error> {
        skip_whitespaces!(self);
        match self.curr_byte {
            b'\'' | b'"' => self.read_string_in_quotes(self.curr_byte),
            b'[' | b'(' => {
                self.read_array_or_set(get_closing_container!(self.curr_byte))
            }
            b'{' => self.read_objects(),
            b'0'..=b'9' | b'+' | b'-' => self.read_number(self.curr_byte),
            _ => Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                self.curr_byte as char,
                self.curr_line,
                self.curr_column,
            ))),
        }
    }

    /// Read string values that are stored
    ///
    fn read_string_in_quotes(
        &mut self,
        end_quote_expected: u8,
    ) -> Result<Container, Error> {
        // Current byte is a quote, read and move to next one
        let mut start: usize = self.offset;
        let mut final_string = "".to_owned();

        loop {
            match read_byte!(self) {
                // Handle this by storing current slice and create a new slice again.
                Some(b'\\') => {
                    unsafe {
                        final_string.push_str(core::str::from_utf8_unchecked(
                            core::slice::from_raw_parts(
                                self.container.add(start),
                                self.offset - start - 1,
                            ),
                        ));
                    }
                    read_byte!(self);
                    match self.curr_byte {
                        b'\'' | b'"' => {
                            if self.curr_byte == end_quote_expected {
                                final_string.push(end_quote_expected as char);
                            }
                        }
                        b'r' => final_string.push('\r'),
                        b't' => final_string.push('\t'),
                        b'n' => final_string.push('\n'),
                        _ => {}
                    }
                    start = self.offset;
                }
                Some(b'\'' | b'"') => {
                    if self.curr_byte == end_quote_expected {
                        unsafe {
                            final_string.push_str(
                                core::str::from_utf8_unchecked(
                                    core::slice::from_raw_parts(
                                        self.container.add(start),
                                        self.offset - start - 1,
                                    ),
                                ),
                            );
                        }
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(Container::String(final_string))
    }

    /// Parse values to store in an array
    fn read_array_or_set(
        &mut self,
        end_bracket_expected: u8,
    ) -> Result<Container, Error> {
        // Current byte is a quote, read and move to next one
        let mut array_container: Container = if end_bracket_expected == b']' {
            Container::new_array()
        } else {
            Container::new_set()
        };

        'parsing_array: loop {
            read_byte!(self);
            skip_whitespaces!(self);
            let curr_container = match self.curr_byte {
                b'\'' | b'"' => self.read_string_in_quotes(self.curr_byte),
                b'[' | b'(' => self
                    .read_array_or_set(get_closing_container!(self.curr_byte)),
                b'{' => self.read_objects(),
                b't' => {
                    expect_next_bytes!(self, b'r', b'u', b'e');
                    Ok(Container::Boolean(true))
                }
                b'f' => {
                    expect_next_bytes!(self, b'a', b'l', b's', b'e');
                    Ok(Container::Boolean(false))
                }
                b'n' => {
                    expect_next_bytes!(self, b'u', b'l', b'l');
                    Ok(Container::Null)
                }
                b']' | b')' | b'}' => {
                    if self.curr_byte == end_bracket_expected {
                        break;
                    } else {
                        Err(Error::ParsingError(
                            ParseError::ContainerParanthesisMismatchError {
                                opening_container: end_bracket_expected as char,
                                closing_container: self.curr_byte as char,
                            },
                        ))
                    }
                }
                b'0'..=b'9' | b'.' | b'-' | b'+' => {
                    self.read_number(self.curr_byte)
                }
                _ => {
                    Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                        self.curr_byte as char,
                        self.curr_line,
                        self.curr_column,
                    )))
                }
            }?;
            array_container.push(curr_container);

            if !self.num_read {
                read_byte!(self);
            } else {
                self.num_read = false;
            }

            skip_whitespaces!(self);
            match self.curr_byte {
                b',' => continue 'parsing_array,
                b']' | b')' => {
                    if self.curr_byte == end_bracket_expected {
                        break;
                    } else {
                        return Err(Error::ParsingError(
                            ParseError::ContainerParanthesisMismatchError {
                                opening_container: end_bracket_expected as char,
                                closing_container: self.curr_byte as char,
                            },
                        ));
                    }
                } // End of current array/set
                _ => {
                    return Err(Error::ParsingError(
                        ParseError::UnexpectedTokenError(
                            self.curr_byte as char,
                            self.curr_line,
                            self.curr_column,
                        ),
                    ));
                }
            }
        }
        Ok(array_container)
    }

    /// Parsing through the object.
    fn read_objects(&mut self) -> Result<Container, Error> {
        let mut object_container = Container::new_object();
        'parsing_objects: loop {
            // First: read the key
            read_byte!(self);
            skip_whitespaces!(self);
            let verification = match self.curr_byte {
                b'\'' | b'\"' => self.read_string_in_quotes(self.curr_byte),
                b'}' => break,
                _ => {
                    Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                        self.curr_byte as char,
                        self.curr_line,
                        self.curr_column,
                    )))
                }
            }?;

            // Skip inverted commas or brackets
            read_byte!(self);
            skip_whitespaces!(self);
            assert_curr_byte!(self, b':');
            // Skip colon
            read_byte!(self);
            skip_whitespaces!(self);

            let assoc_value = match self.curr_byte {
                b'\'' | b'\"' => self.read_string_in_quotes(self.curr_byte),
                b'{' => self.read_objects(),
                b'[' | b'(' => self
                    .read_array_or_set(get_closing_container!(self.curr_byte)),
                b']' | b')' | b'}' => {
                    if self.curr_byte == b'}' {
                        Err(Error::ParsingError(
                            ParseError::InvalidKeyValueFormatError {
                                reading_key: verification.as_string().unwrap(),
                            },
                        ))
                    } else {
                        Err(Error::ParsingError(
                            ParseError::ContainerParanthesisMismatchError {
                                opening_container: '{',
                                closing_container: self.curr_byte as char,
                            },
                        ))
                    }
                }
                b't' => {
                    expect_next_bytes!(self, b'r', b'u', b'e');
                    Ok(Container::Boolean(true))
                }
                b'f' => {
                    expect_next_bytes!(self, b'a', b'l', b's', b'e');
                    Ok(Container::Boolean(false))
                }
                b'n' => {
                    expect_next_bytes!(self, b'u', b'l', b'l');
                    Ok(Container::Null)
                }
                b'0'..=b'9' | b'.' | b'-' | b'+' => {
                    self.read_number(self.curr_byte)
                }
                _ => {
                    Err(Error::ParsingError(ParseError::UnexpectedTokenError(
                        self.curr_byte as char,
                        self.curr_line,
                        self.curr_column,
                    )))
                }
            }?;
            object_container.insert_str(
                verification.as_string().unwrap().as_str(),
                assoc_value,
            );

            if !self.num_read {
                read_byte!(self);
            } else {
                self.num_read = false;
            }

            skip_whitespaces!(self);

            match self.curr_byte {
                b',' => continue 'parsing_objects,
                b'}' => break,
                b']' | b')' => {
                    return Err(Error::ParsingError(
                        ParseError::ContainerParanthesisMismatchError {
                            opening_container: '{',
                            closing_container: self.curr_byte as char,
                        },
                    ));
                }
                _ => {
                    return Err(Error::ParsingError(
                        ParseError::UnexpectedTokenError(
                            self.curr_byte as char,
                            self.curr_line,
                            self.curr_column,
                        ),
                    ));
                }
            }
        }

        Ok(object_container)
    }

    /// Read a number from given input
    /// Returns Error if an unexpected token occurs.
    #[allow(unused_variables)]
    fn read_number(&mut self, byte_read: u8) -> Result<Container, Error> {
        let (mut read_dot, sign_read, mut prev_byte) = (
            byte_read == b'.',
            if byte_read == b'-' { b'-' } else { b'+' },
            byte_read,
        );
        let abrupt_end;
        let (mut read_exp, mut sign_exp, start) =
            (false, false, self.offset - 1);

        loop {
            match read_byte!(self) {
                Some(b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-') => {
                    if (read_dot && self.curr_byte == b'.')
                        || (read_exp
                            && (equals_in!(self.curr_byte, b'.', b'e', b'E')))
                    {
                        return Err(Error::ParsingError(
                            ParseError::InvalidNumberParseError(
                                self.curr_byte as char,
                            ),
                        ));
                    }

                    if read_exp
                        && !equals_in!(prev_byte, b'e', b'E')
                        && equals_in!(self.curr_byte, b'+', b'-')
                    {
                        return Err(Error::ParsingError(
                            ParseError::InvalidNumberParseError(
                                self.curr_byte as char,
                            ),
                        ));
                    }

                    read_exp |= equals_in!(self.curr_byte, b'e', b'E');
                    sign_exp |= read_exp
                        && equals_in!(prev_byte, b'e', b'E')
                        && equals_in!(self.curr_byte, b'-', b'+');
                    read_dot |= read_exp || self.curr_byte == b'.';
                }
                val @ Some(b' ' | 9..=13 | b',' | b']' | b'}' | b')')
                | val @ None => {
                    (self.num_read, abrupt_end) = (true, val.is_none());
                    break;
                }
                _ => {
                    return Err(Error::ParsingError(
                        ParseError::InvalidNumberParseError(
                            self.curr_byte as char,
                        ),
                    ));
                }
            }
            prev_byte = self.curr_byte;
        }
        let str_slice = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.container.add(start),
                self.offset - start - 1 + if abrupt_end { 1 } else { 0 },
            ))
        };

        if read_dot || read_exp {
            Ok(Container::Decimal(str_slice.parse::<f64>().unwrap()))
        } else if sign_read == b'-' {
            Ok(Container::Number(str_slice.parse::<i64>().unwrap()))
        } else {
            Ok(Container::Unsigned(str_slice.parse::<u64>().unwrap()))
        }
    }
}

/// Read the files in byte form
/// For testing purpose: as it might be fastest
#[inline(always)]
#[allow(unused)]
pub fn read_str(input_str: &str) -> Result<Container, Error> {
    parse_str(input_str)
}
/// Parsing bytestream
/// Parse the file from an input stream: taking unsafe route
#[inline(always)]
pub fn parse_str(input_str: &str) -> Result<Container, Error> {
    Parser::new(input_str).parse_str()
}
