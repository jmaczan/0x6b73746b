use crate::error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    White,

    Eof,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, // originally it was Object; likely that this type should be changed to something else once I figure out what it exactly is;
    // turns out it needs to be String for some literals and i32 for others;
    numeric_literal: i32,
    line: u8,
}

impl Token {
    pub fn to_string(&self) -> String {
        self.lexeme.to_owned() + " " + &self.literal.to_owned() // it should start with "self.token_type +" but I didn't manage to parse TokenType to String yet
    }
}

struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !Self::is_at_end(self) {
            self.start = self.current;
            Self::scan_token(self);
        }

        self.append_eof_token();

        &self.tokens
    }

    fn append_eof_token(&mut self) {
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            literal: "".to_owned(), // originally Null
            numeric_literal: 0,     // a stub value
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u8
    }

    fn scan_token(&mut self) {
        // https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs
        // https://craftinginterpreters.com/scanning.html#recognizing-lexemes
        let character = Self::advance(self);
        match character {
            '(' => Self::add_empty_token(self, TokenType::LeftParen),
            ')' => Self::add_empty_token(self, TokenType::RightParen),
            '{' => Self::add_empty_token(self, TokenType::LeftBrace),
            '}' => Self::add_empty_token(self, TokenType::RightBrace),
            ',' => Self::add_empty_token(self, TokenType::Comma),
            '.' => Self::add_empty_token(self, TokenType::Dot),
            '-' => Self::add_empty_token(self, TokenType::Minus),
            '+' => Self::add_empty_token(self, TokenType::Plus),
            ';' => Self::add_empty_token(self, TokenType::Semicolon),
            '*' => Self::add_empty_token(self, TokenType::Star),
            '!' => {
                if Self::match_character(self, '=') {
                    Self::add_empty_token(self, TokenType::BangEqual)
                } else {
                    Self::add_empty_token(self, TokenType::Bang)
                }
            }
            '=' => {
                if Self::match_character(self, '=') {
                    Self::add_empty_token(self, TokenType::EqualEqual)
                } else {
                    Self::add_empty_token(self, TokenType::Equal)
                }
            }
            '<' => {
                if Self::match_character(self, '=') {
                    Self::add_empty_token(self, TokenType::LessEqual)
                } else {
                    Self::add_empty_token(self, TokenType::Equal)
                }
            }
            '>' => {
                if Self::match_character(self, '=') {
                    Self::add_empty_token(self, TokenType::GreaterEqual)
                } else {
                    Self::add_empty_token(self, TokenType::Equal)
                }
            }
            '/' => {
                if Self::match_character(self, '/') {
                    while Self::peek(self) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    Self::add_empty_token(self, TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '"' => Self::string(self),
            '\n' => {
                self.line += 1;
            }
            '0'..='9' => {
                Self::number(self);
            }
            _ => {
                error(self.line, String::from("Unexpected character"));
            }
        };
    }

    fn advance(&self) -> char {
        self.source
            .as_str()
            .chars()
            .nth((&self.current + 1).into())
            .unwrap()
    }

    fn add_empty_token(&mut self, token_type: TokenType) {
        Self::add_token(self, token_type, None, None);
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal: Option<String>,
        numeric_literal: Option<i32>,
    ) {
        let text: Option<&str> = self.source.get(self.start.into()..self.current.into());

        self.tokens.push(Token {
            token_type,
            lexeme: match text {
                Some(text) => text.to_owned(),
                None => "".to_owned(), // maybe it should fail in more explicit manner
            },
            literal: literal.unwrap_or_default().to_owned(), // it makes that literal always have a value; maybe it's not desired behavior
            numeric_literal: numeric_literal.unwrap_or_default().to_owned(), // it makes that numeric_literal always have a value; maybe it's not desired behavior
            line: self.line,
        })
    }

    fn match_character(&mut self, character: char) -> bool {
        if self.is_at_end()
            || self
                .source
                .as_str()
                .chars()
                .nth(self.current.into())
                .unwrap()
                != character
        {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source
            .as_str()
            .chars()
            .nth(self.current.into())
            .unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_owned());
            return;
        }

        self.advance();

        let text = match self.source.get(self.start.into()..self.current.into()) {
            Some(text) => text.to_owned(),
            None => "".to_owned(), // handle it better way; it's still an error situation
        };

        self.add_token(TokenType::String, Some(text), None);
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let number = match self.source.get(self.start.into()..self.current.into()) {
            Some(text) => text.to_owned(),
            None => "".to_owned(), // handle it better way; it's still an error situation
        };

        self.add_token(
            TokenType::Number,
            None,
            Some(number.parse::<i32>().expect("Invalid number")), // handle a failure of parsing a number in better way which wouldn't stop a compiler
        );
    }

    fn is_digit(character: char) -> bool {
        match character {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as u8 {
            return '\0';
        }

        self.source
            .as_str()
            .chars()
            .nth((self.current + 1).into())
            .unwrap()
    }
}
