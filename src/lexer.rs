use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerError {
    UnclosedStringLiteral,
    UnknownEscapeSequence(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // Special
    EOF,
    ID(String),

    // Literals
    Int(i32),        // 123
    Float(f32),      // 45.32f
    Char(char),      // 'a'
    String(String),  // "Hello, World!"

    // Operators
    Plus,            // +
    Minus,           // -
    Multiply,        // *
    Divide,          // /
    Mod,             // %
    And,             // &
    Or,              // |
    Xor,             // ^
    ShiftLeft,       // <<
    ShiftRight,      // >>
    Equal,           // =
    EqualEqual,      // ==
    NotEqual,        // !=
    Less,            // <
    LessEqual,       // <=
    Greater,         // >
    GreaterEqual,    // >=
    AndAnd,          // &&
    OrOr,            // ||
    PlusPlus,        // ++
    MinusMinus,      // --
    PlusEqual,       // +=
    MinusEqual,      // -=
    MultiplyEqual,   // *=
    DivideEqual,     // /=
    ModEqual,        // %=
    OrEqual,         // |=
    XorEqual,        // ^=
    ShiftLeftEqual,  // <<=
    ShiftRightEqual, // >>=  `ShREq` operator :)
    
    // Separators
    OParen,          // (
    CParen,          // )
    OCurly,          // {
    CCurly,          // }
    Comma,           // ,
    SemiColon,       // ;
}

#[derive(Debug, Clone)]
pub struct Location {
    pub filepath: String,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}:{}", self.filepath, self.row + 1, self.col + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    filepath: String,

    cur: usize, // Cursor
    row: usize, // Current row
    bol: usize, // Start of current row
}

impl Lexer {
    pub fn new(source: String, filepath: String) -> Self {
        Self {
            source,
            filepath,
            cur: 0,
            row: 0,
            bol: 0,
        }
    }

    pub fn get_token(&mut self) -> Result<Token, LexerError> {
        self.trim_left();
        if self.is_empty() { return Ok(Token::EOF); }

        let cur_char = self.get_char(0).unwrap();

        // TODO: add comments support
        
        if cur_char.is_alphabetic() {
            let start: usize = self.cur;
            while !self.is_empty() && self.get_char(0).unwrap().is_alphanumeric() {
                self.chop_char();
            }

            return Ok(Token::ID(self.source[start..self.cur].to_string()))
        }

        if cur_char.is_ascii_digit() {
            let start = self.cur;
            while !self.is_empty() && self.get_char(0).unwrap().is_ascii_digit() {
                self.chop_char();
            }
            let value: i32 = self.source[start..self.cur].parse().unwrap();
            return Ok(Token::Int(value));
        }

        if cur_char == '"' {
            use std::vec::Vec;
            self.chop_char();
            let mut string: Vec<char> = Vec::new();
            loop {
                if self.is_empty() { return Err(LexerError::UnclosedStringLiteral); }
                let ch = self.get_char(0).unwrap();
                if ch == '"' { break; }

                if ch == '\\' {
                    if self.is_empty() { return Err(LexerError::UnclosedStringLiteral); }
                    self.chop_char();
                    string.push(match self.get_char(0).unwrap() {
                        // TODO: add support for \nnn, \xhh…, \uhhhh, \Uhhhhhhhh
                        // https://en.wikipedia.org/wiki/Escape_sequences_in_C#Escape_sequences
                        'a' => 0x07 as char, // Alert (Beep, Bell) - Added in C89
                        'b' => 0x08 as char, // Backspace
                        'e' => 0x1B as char, // Escape character
                        'f' => 0x0C as char, // Formfeed Page Break
                        'v' => 0x0B as char, // Vertical Tab
                        
                        '?' => '?',  // Question mark (used to avoid trigraphs) https://en.wikipedia.org/wiki/Digraphs_and_trigraphs_(programming)#C
                        'n' => '\n', // Newline
                        'r' => '\r', // Carriage Return
                        't' => '\t', // Horizontal Tab
                        
                        '\'' => '\'',
                        '"' => '"',
                        '\\' => '\\',
                        _ => return Err(
                            LexerError::UnknownEscapeSequence(format!("\\{}", self.get_char(0).unwrap()))
                        ),
                    });
                } else {
                    string.push(ch);
                }
                self.chop_char();
            }
            self.chop_char();
            return Ok(Token::String(string.into_iter().collect::<String>()));
        }

        self.chop_char();

        return Ok(match cur_char {
            '(' => Token::OParen,
            ')' => Token::CParen,
            '{' => Token::OCurly,
            '}' => Token::CCurly,
            ';' => Token::SemiColon,
            ',' => Token::Comma,

            '=' => {
                if self.is_empty() || self.get_char(0).unwrap() != '=' {
                    Token::Equal
                } else {
                    Token::EqualEqual
                }
            },
            '+' => {
                if !self.is_empty() {
                    match self.get_char(0).unwrap() {
                        '+' => Token::PlusPlus,
                        '=' => Token::PlusEqual,
                        _   => Token::Plus,
                    }
                } else {
                    Token::Plus
                }
            },
            '-' => {
                if !self.is_empty() {
                    match self.get_char(0).unwrap() {
                        '-' => Token::MinusMinus,
                        '=' => Token::MinusEqual,
                        _   => Token::Minus,
                    }
                } else {
                    Token::Minus
                }
            },
            '*' => {
                if self.is_empty() || self.get_char(0).unwrap() != '=' {
                    Token::Multiply
                } else {
                    Token::MultiplyEqual
                }
            },
            '/' => {
                if self.is_empty() || self.get_char(0).unwrap() != '=' {
                    Token::Divide
                } else {
                    Token::DivideEqual
                }
            },
            '%' => {
                if self.is_empty() || self.get_char(0).unwrap() != '=' {
                    Token::Mod
                } else {
                    Token::ModEqual
                }
            }
            // TODO: add proper error reporting
            _ => panic!("Unknown symbol {cur_char}"),
        });
    }

    pub fn get_location(&self) -> Location {
        Location { filepath: self.filepath.clone(), row: self.row, col: self.cur - self.bol }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn chop_char(&mut self) {
        if !self.is_empty() {
            let c: char = self.get_char(0).unwrap();
            self.cur += 1;
            if c == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.get_char(0).unwrap().is_whitespace() {
            self.chop_char();
        }
    }

    fn drop_line(&mut self) {
        while !self.is_empty() && self.get_char(0).unwrap() == '\n' { self.chop_char(); }
        if !self.is_empty() { self.chop_char(); }
    }

    fn get_char(&self, index: usize) -> Option<char> {
        self.source.chars().nth(self.cur + index)
    }
}
