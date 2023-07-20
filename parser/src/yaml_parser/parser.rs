/// Single-threaded parsing module, with an intent to parse the
/// files faster with handling run-time errors (hopefully), considering two modes
/// of parsing:

/// - JSON parsing
/// - Binary Data Parsing (where integers are of fixed 4 bytes)
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
        let chr = $parser.get_next_byte();
        if chr == Option::None {
            return Result::Error(Error::ParsingError(ParseError::EndOfBufferError));
        }
        chr.unwrap()
    }};
}

macro_rules! expect_next_bytes {
    ($parser:ident, $( $next_char:expr ),*) => ({
        $( let next_byte = read_byte!($parser);
            if next_byte != $next_char {
            return Result::Error(Error::ParsingError(ParseError::UnexpectedTokenError(next_byte as char)));
        } )*
    })
}

macro_rules! skip_whitespaces {
    ($parser:ident) => {{
        loop {
            match $parser.curr_byte {
                b' ' | 09..=13 => {
                    read_byte!($parser);
                }
                _ => break,
            }
        }
    }};
}

macro_rules! assert_curr_byte {
    ($parser:ident, $byte_val:expr) => {
        if $parser.curr_byte != $byte_val {
            return Result::Error(Error::ParsingError(ParseError::UnexpectedTokenError(
                $parser.curr_byte as char,
            )));
        }
    };
}

macro_rules! get_closing_container {
    ($chr:expr) => {
        if $chr == b'[' {
            b']'
        } else if $chr == b'{' {
            b'}'
        } else if $chr == b'(' {
            b')'
        } else {
            b'\0'
        }
    };
}

impl Parser {
    fn new(str_stream: &str) -> Parser {
        Parser {
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
    /// Returns Value wrapped with Option if exist.
    #[inline(always)]
    fn get_next_byte(&mut self) -> Option<u8> {
        if self.offset < self.len {
            unsafe {
                self.curr_byte = *self.container.offset(self.offset as isize);
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
            Option::None
        }
    }

    // pub fn parse_str(&mut self) -> Result<Container> {}
}
