use super::container::Container;
use super::error::Error;
use super::error::ParseError;
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
    ($parser:ident) => {
        $parser.get_next_byte()
    };
}

macro_rules! expect_next_bytes {
    ($parser:ident, $( $next_char:expr ),*) => ({
        $(
            match read_byte!($parser) {
                Some($next_char) => {}
                None => return Err(Error::Parsing(ParseError::EndOfBuffer).into()),
                Some(r) => {
                    return Err(Error::Parsing(ParseError::UnexpectedToken(
                        r as char,
                        $parser.curr_line,
                        $parser.curr_column
                    )).into());
                }
            }
        )*
    })
}

macro_rules! equals_in {
    ($compare: expr, $( $char: expr ),*) => {
        match $compare {
            $ ( $char )|* => true,
            _ => false
        }
    };
}

impl Parser {
    /// Creates a new JSON parser.
    #[inline(always)]
    fn new(str_stream: &str) -> Self {
        Self {
            container: str_stream.as_ptr(),
            offset: 0,
            curr_line: 1,
            curr_column: 1,
            len: str_stream.len(),
            num_read: false,
        }
    }

    #[inline]
    fn get_byte(&mut self) -> Option<u8> {
        loop {
            let resp = match self.get_next_byte() {
                Some(value) if (value as char).is_ascii_whitespace() => None,
                None => return None,
                val @ Some(_) => val,
            };
            if resp.is_some() {
                return resp;
            }
        }
    }

    /// Get the next byte from the buffer string
    /// Returns none if length exceeds the length of buffer,
    ///
    /// Returns `Option<u8>`.
    fn get_next_byte(&mut self) -> Option<u8> {
        (self.offset < self.len).then(|| {
            let chr = unsafe { *self.container.add(self.offset) };
            self.offset += 1;

            if chr == b'\n' {
                self.curr_line += 1;
                self.curr_column = 0;
            } else {
                self.curr_column += 1;
            }

            chr
        })
    }

    /// Parsing bytestream
    /// Parse the file from an input stream: taking unsafe route
    #[inline(always)]
    pub fn parse_str(
        &mut self,
    ) -> Result<Container, Box<dyn core::error::Error>> {
        let answer = match self.get_next_byte() {
            Some(b'\'' | b'"') => Ok(self.read_string_in_quotes()?),
            Some(b'[') => Ok(self.read_array()?),
            Some(b'{') => Ok(self.read_objects()?),
            val @ Some(b'0'..=b'9' | b'+' | b'-') => {
                self.read_number(val.unwrap())
            }
            Some(b't') => {
                expect_next_bytes!(self, b'r', b'u', b'e');

                if let Some(chr) = self.get_byte() {
                    Err(Error::Parsing(ParseError::UnexpectedToken(
                        chr as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into())
                } else {
                    Ok(Container::Boolean(true))
                }
            }
            Some(b'f') => {
                expect_next_bytes!(self, b'a', b'l', b's', b'e');

                if let Some(chr) = self.get_byte() {
                    Err(Error::Parsing(ParseError::UnexpectedToken(
                        chr as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into())
                } else {
                    Ok(Container::Boolean(false))
                }
            }
            Some(b'n') => {
                expect_next_bytes!(self, b'u', b'l', b'l');

                if let Some(chr) = self.get_byte() {
                    Err(Error::Parsing(ParseError::UnexpectedToken(
                        chr as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into())
                } else {
                    Ok(Container::Null)
                }
            }
            None => Err(Error::Parsing(ParseError::EndOfBuffer).into()),
            Some(c) => Err(Error::Parsing(ParseError::UnexpectedToken(
                c as char,
                self.curr_line,
                self.curr_column,
            ))
            .into()),
        };

        if let Some(chr) = self.get_byte() {
            Err(Error::Parsing(ParseError::UnexpectedToken(
                chr as char,
                self.curr_line,
                self.curr_column
            )).into())
        } else {
            answer
        }
    }

    /// Read string values that are stored
    ///
    fn read_string_in_quotes(
        &mut self,
    ) -> Result<Container, Box<dyn core::error::Error>> {
        // Current byte is a quote, read and move to next one
        let (mut start, mut final_string) = (self.offset, "".to_owned());

        loop {
            match self.get_byte() {
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

                    match self.get_byte() {
                        Some(b'"') => {
                            final_string.push('"');
                        }
                        Some(b'r') => final_string.push('\r'),
                        Some(b't') => final_string.push('\t'),
                        Some(b'n') => final_string.push('\n'),
                        None => {
                            return Err(
                                Error::Parsing(ParseError::EndOfBuffer).into()
                            )
                        }
                        Some(c) => {
                            final_string.push('\\');
                            final_string.push(c as char);
                        }
                    }
                    start = self.offset;
                }
                Some(b'"') => {
                    unsafe {
                        final_string.push_str(core::str::from_utf8_unchecked(
                            core::slice::from_raw_parts(
                                self.container.add(start),
                                self.offset - start - 1,
                            ),
                        ));
                    }
                    break;
                }
                None => return Err(
                    Error::Parsing(ParseError::EndOfBuffer).into()
                ),
                _ => {}
            }
        }

        Ok(Container::String(final_string))
    }

    /// Parse values to store in an array
    fn read_array(&mut self) -> Result<Container, Box<dyn core::error::Error>> {
        // Current byte is a quote, read and move to next one
        let mut array_container = Container::new_array();

        'parsing_array: loop {
            let curr_container = match self.get_byte() {
                Some(b'"') => self.read_string_in_quotes(),
                Some(b'[') => self.read_array(),
                Some(b'{') => self.read_objects(),
                Some(b't') => {
                    expect_next_bytes!(self, b'r', b'u', b'e');
                    Ok(Container::Boolean(true))
                }
                Some(b'f') => {
                    expect_next_bytes!(self, b'a', b'l', b's', b'e');
                    Ok(Container::Boolean(false))
                }
                Some(b'n') => {
                    expect_next_bytes!(self, b'u', b'l', b'l');
                    Ok(Container::Null)
                }
                Some(b']') => break,
                Some(b'}') => Err(Error::Parsing(
                    ParseError::ContainerParanthesisMismatch {
                        opening_container: ']',
                        closing_container: '}',
                    },
                )
                .into()),
                val @ Some(b'0'..=b'9' | b'.' | b'-' | b'+') => {
                    self.read_number(val.unwrap())
                }
                None => Err(Error::Parsing(ParseError::EndOfBuffer).into()),
                Some(c) => Err(Error::Parsing(ParseError::UnexpectedToken(
                    c as char,
                    self.curr_line,
                    self.curr_column,
                ))
                .into()),
            }?;
            array_container.push(curr_container);

            match self.get_byte() {
                Some(b',') => continue 'parsing_array,
                Some(b']') => break,
                Some(b'}') => {
                    return Err(Error::Parsing(
                        ParseError::ContainerParanthesisMismatch {
                            opening_container: ']',
                            closing_container: '}',
                        },
                    )
                    .into());
                }
                None => {
                    return Err(Error::Parsing(ParseError::EndOfBuffer).into())
                }
                Some(c) => {
                    return Err(Error::Parsing(ParseError::UnexpectedToken(
                        c as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into());
                }
            }
        }

        Ok(array_container)
    }

    /// Parsing through the object.
    fn read_objects(
        &mut self,
    ) -> Result<Container, Box<dyn core::error::Error>> {
        let mut object_container = Container::new_object();
        'parsing_objects: loop {
            // First: read the key
            let verification = match self.get_byte() {
                Some(b'"') => self.read_string_in_quotes(),
                Some(b'}') => break,
                None => {
                    return Err(Error::Parsing(ParseError::EndOfBuffer).into())
                }
                Some(c) => Err(Error::Parsing(ParseError::UnexpectedToken(
                    c as char,
                    self.curr_line,
                    self.curr_column,
                ))
                .into()),
            }?;

            // Skip inverted commas or brackets
            match self.get_byte() {
                Some(b':') => {}
                None => {
                    return Err(Error::Parsing(ParseError::EndOfBuffer).into())
                }
                Some(other) => {
                    return Err(Error::Parsing(ParseError::UnexpectedToken(
                        other as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into())
                }
            }

            let assoc_value = match self.get_byte() {
                Some(b'"') => self.read_string_in_quotes(),
                Some(b'{') => self.read_objects(),
                Some(b'[') => self.read_array(),
                Some(b'}') => {
                    Err(Error::Parsing(ParseError::InvalidKeyValueFormat {
                        reading_key: verification.get_string().unwrap(),
                    })
                    .into())
                }
                Some(b']') => Err(Error::Parsing(
                    ParseError::ContainerParanthesisMismatch {
                        opening_container: '{',
                        closing_container: ']',
                    },
                )
                .into()),
                Some(b't') => {
                    expect_next_bytes!(self, b'r', b'u', b'e');
                    Ok(Container::Boolean(true))
                }
                Some(b'f') => {
                    expect_next_bytes!(self, b'a', b'l', b's', b'e');
                    Ok(Container::Boolean(false))
                }
                Some(b'n') => {
                    expect_next_bytes!(self, b'u', b'l', b'l');
                    Ok(Container::Null)
                }
                val @ Some(b'0'..=b'9' | b'.' | b'-' | b'+') => {
                    self.read_number(val.unwrap())
                }
                None => {
                    return Err(Error::Parsing(ParseError::EndOfBuffer).into())
                }
                Some(c) => Err(Error::Parsing(ParseError::UnexpectedToken(
                    c as char,
                    self.curr_line,
                    self.curr_column,
                ))
                .into()),
            }?;
            object_container.insert_str(
                verification.get_string().unwrap().as_str(),
                assoc_value,
            );

            match self.get_byte() {
                Some(b',') => continue 'parsing_objects,
                Some(b'}') => break,
                Some(b']') => {
                    return Err(Error::Parsing(
                        ParseError::ContainerParanthesisMismatch {
                            opening_container: '{',
                            closing_container: ']',
                        },
                    )
                    .into());
                }
                None => {
                    return Err(Error::Parsing(ParseError::EndOfBuffer).into())
                }
                Some(c) => {
                    return Err(Error::Parsing(ParseError::UnexpectedToken(
                        c as char,
                        self.curr_line,
                        self.curr_column,
                    ))
                    .into());
                }
            }
        }

        Ok(object_container)
    }

    /// Read a number from given input
    /// Returns Error if an unexpected token occurs.
    #[allow(unused_variables)]
    fn read_number(
        &mut self,
        byte_read: u8,
    ) -> Result<Container, Box<dyn core::error::Error>> {
        let (mut read_dot, sign_read, mut prev_byte) = (
            byte_read == b'.',
            if byte_read == b'-' { b'-' } else { b'+' },
            byte_read,
        );
        let abrupt_end;
        let (mut read_exp, mut sign_exp, start) =
            (false, false, self.offset - 1);

        loop {
            prev_byte = match self.get_byte() {
                val @ Some(b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-') => {
                    let chr = val.unwrap();
                    if (read_dot && chr == b'.')
                        || (read_exp && (equals_in!(chr, b'.', b'e', b'E')))
                    {
                        return Err(Error::Parsing(
                            ParseError::InvalidNumberParse(chr as char),
                        )
                        .into());
                    }

                    if read_exp
                        && !equals_in!(prev_byte, b'e', b'E')
                        && equals_in!(chr, b'+', b'-')
                    {
                        return Err(Error::Parsing(
                            ParseError::InvalidNumberParse(chr as char),
                        )
                        .into());
                    }

                    read_exp |= equals_in!(chr, b'e', b'E');
                    sign_exp |= read_exp
                        && equals_in!(prev_byte, b'e', b'E')
                        && equals_in!(chr, b'-', b'+');
                    read_dot |= read_exp || chr == b'.';
                    chr
                }
                val @ Some(b' ' | 9..=13 | b',' | b']' | b'}') | val @ None => {
                    (self.num_read, abrupt_end) = (true, val.is_none());
                    break;
                }
                Some(c) => {
                    return Err(Error::Parsing(
                        ParseError::InvalidNumberParse(c as char),
                    )
                    .into());
                }
            };
        }
        if !abrupt_end {
            self.offset -= 1;
        }
        let str_slice = unsafe {
            core::str::from_utf8_unchecked(
                core::slice::from_raw_parts(
                    self.container.add(start),
                    self.offset - start,
                )
                .trim_ascii(),
            )
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
pub fn read_str(
    input_str: &str,
) -> Result<Container, Box<dyn core::error::Error>> {
    parse_str(input_str)
}
/// Parsing bytestream
/// Parse the file from an input stream: taking unsafe route
#[inline(always)]
pub fn parse_str(
    input_str: &str,
) -> Result<Container, Box<dyn core::error::Error>> {
    Parser::new(input_str).parse_str()
}
