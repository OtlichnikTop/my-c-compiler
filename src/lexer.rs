use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerError {
    UnterminatedStringLiteral,
    UnknownEscapeSequence(String),
    UnknownToken(char),
}

#[derive(Debug, Clone)]
pub enum Token<'src> {
    // Special
    EOF,
    ID(&'src str),

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
    Arrow,           // ->
    
    // Separators
    OParen,          // (
    CParen,          // )
    OCurly,          // {
    CCurly,          // }
    Comma,           // ,
    SemiColon,       // ;
}

impl<'src> PartialEq for Token<'src> {
    fn eq(&self, other: &Self) -> bool {
        use std::mem;
        mem::discriminant(self) == mem::discriminant(other)
    }
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
pub struct Lexer<'src> {
    source: &'src str,
    filepath: String,

    cur: usize, // Cursor
    row: usize, // Current row
    bol: usize, // Start of current row
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str, filepath: String) -> Self {
        Self {
            source,
            filepath,
            cur: 0,
            row: 0,
            bol: 0,
        }
    }

    pub fn expect_token(&mut self, expected_token: Token) -> Result<Option<Token<'src>>, LexerError> {
        match self.get_token() {
            Ok(token) => Ok(if token == expected_token {
                Some(token)
            } else {
                None
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_token(&mut self) -> Result<Token<'src>, LexerError> {
        self.trim_left();
        if self.is_empty() { return Ok(Token::EOF); }

        // TODO: add comments support. Maybe remove those in preprocessor stage???

        let first_char = self.get_char().unwrap();

        match first_char {
            c if c.is_alphabetic() || c == '_' => self.lex_id(),
            c if c.is_ascii_digit()            => self.lex_number(),
            '\''                               => self.lex_char(),
            '"'                                => self.lex_string(),
            _                                  => self.lex_operator_or_separator(),
        }
        
        /*
        return Ok(match cur_char {
            '(' => Token::OParen,
            ')' => Token::CParen,
            '{' => Token::OCurly,
            '}' => Token::CCurly,
            ';' => Token::SemiColon,
            ',' => Token::Comma,

            '=' => {
                if self.is_empty() || self.get_char().unwrap() != '=' {
                    Token::Equal
                } else {
                    Token::EqualEqual
                }
            },
            '+' => {
                if !self.is_empty() {
                    match self.get_char().unwrap() {
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
                    match self.get_char().unwrap() {
                        '-' => Token::MinusMinus,
                        '=' => Token::MinusEqual,
                        _   => Token::Minus,
                    }
                } else {
                    Token::Minus
                }
            },
            '*' => {
                if self.is_empty() || self.get_char().unwrap() != '=' {
                    Token::Multiply
                } else {
                    Token::MultiplyEqual
                }
            },
            '/' => {
                if self.is_empty() || self.get_char().unwrap() != '=' {
                    Token::Divide
                } else {
                    Token::DivideEqual
                }
            },
            '%' => {
                if self.is_empty() || self.get_char().unwrap() != '=' {
                    Token::Mod
                } else {
                    Token::ModEqual
                }
            },
            _ => return Err(LexerError::UnknownToken(cur_char)),
    });
         */
    }

    pub fn get_location(&self) -> Location {
        Location { filepath: self.filepath.clone(), row: self.row, col: self.cur - self.bol }
    }

    fn lex_id(&mut self) -> Result<Token<'src>, LexerError> {
        let start: usize = self.cur;
        self.consume_while(|c| c.is_alphanumeric() || c == '_');
        let text = &self.source[start..self.cur];
        return Ok(Token::ID(text));
    }

    fn lex_number(&mut self) -> Result<Token<'src>, LexerError> {
        let start: usize = self.cur;
        self.consume_while(|c| c.is_ascii_digit());
        // TODO: add support for floats, doubles, hexadecimals, octals, etc.
        let text = &self.source[start..self.cur];
        let value: i32 = text.parse().unwrap();
        return Ok(Token::Int(value));
    }

    fn lex_char(&mut self) -> Result<Token<'src>, LexerError> {
        todo!("lex_char")
    }
    
    fn lex_string(&mut self) -> Result<Token<'src>, LexerError> {
        self.chop_char(); // Skip opening `"`

        let mut string_content: Vec<char> = Vec::new();
        
        while !self.is_empty() {
            let ch: char = self.get_char().unwrap();

            if ch == '"' {
                self.chop_char(); // Skip closing `"`
                let string_content: String = string_content.into_iter().collect();
                return Ok(Token::String(string_content));
            }

            if ch == '\\' {
                self.chop_char(); // Skip `\`
                if self.is_empty() { return Err(LexerError::UnterminatedStringLiteral); }

                let real_char = self.lex_escape_sequence()?;

                string_content.push(real_char);

                self.chop_char();
                continue;
            }
            
            string_content.push(ch);
            self.chop_char();
        }
    
        return Err(LexerError::UnterminatedStringLiteral);
    }

    fn lex_escape_sequence(&mut self) -> Result<char, LexerError> {
        // TODO: add support for \nnn, \xhh…, \uhhhh, \Uhhhhhhhh
        // https://en.wikipedia.org/wiki/Escape_sequences_in_C#Escape_sequences
        return Ok(
            match self.get_char().unwrap() {
                'a' => 0x07 as char, // Alert (Beep, Bell) - Added in C89
                'b' => 0x08 as char, // Backspace
                'e' => 0x1B as char, // Escape character
                'f' => 0x0C as char, // Formfeed Page Break
                'v' => 0x0B as char, // Vertical Tab
                
                '?' => '?',          // Question mark (used to avoid trigraphs)
                // https://en.wikipedia.org/wiki/Digraphs_and_trigraphs_(programming)#C
                
                'n' => '\n',         // Newline
                'r' => '\r',         // Carriage Return
                't' => '\t',         // Horizontal Tab
                
                '\'' => '\'',        // '
                '"' => '"',          // "
                '\\' => '\\',        // \
                
                _ => return Err(
                    LexerError::UnknownEscapeSequence(format!("\\{}", self.get_char().unwrap()))
                ),
            }
        );
    }

    fn lex_operator_or_separator(&mut self) -> Result<Token<'src>, LexerError> {
        let cur_char: char = self.get_char().unwrap();
        self.chop_char();
        
        return Ok(
            match cur_char {
                '(' => Token::OParen,
                ')' => Token::CParen,
                '{' => Token::OCurly,
                '}' => Token::CCurly,
                ';' => Token::SemiColon,
                ',' => Token::Comma,

                '=' => {
                    if self.is_empty() { return Ok(Token::Equal); }

                    let cur_char: char = self.get_char().unwrap();
                    if cur_char.is_whitespace() || cur_char.is_alphanumeric() {
                        return Ok(Token::Equal);
                    }

                    if cur_char == '=' { return Ok(Token::EqualEqual); }
                    
                    return Err(LexerError::UnknownToken(cur_char))
                },
                
                _   => return Err(LexerError::UnknownToken(cur_char)),
            }
        );
    }

    fn consume_while<P>(&mut self, predicate: P) where P: Fn(char) -> bool {
        while !self.is_empty() && predicate(self.get_char().unwrap()) {
            self.chop_char();
        }
    }
        
    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn chop_char(&mut self) {
        if !self.is_empty() {
            let c: char = self.get_char().unwrap();
            self.cur += 1;
            if c == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.get_char().unwrap().is_whitespace() {
            self.chop_char();
        }
    }

    fn drop_line(&mut self) {
        while !self.is_empty() && self.get_char().unwrap() == '\n' { self.chop_char(); }
        if !self.is_empty() { self.chop_char(); }
    }

    fn get_char(&self) -> Option<char> {
        self.source.chars().nth(self.cur)
    }
}
