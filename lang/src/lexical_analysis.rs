use std::collections::HashMap;

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
    While,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String, // originally it was Object; likely that this type should be changed to something else once I figure out what it exactly is;
    // turns out it needs to be String for some literals and i32 for others;
    pub numeric_literal: f32,
    pub line: u8,
}

impl Token {
    pub fn to_string(&self) -> String {
        self.lexeme.to_owned() + " " + &self.literal.to_owned() // it should start with "self.token_type +" but I didn't manage to parse TokenType to String yet
    }
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
    keywords: HashMap<&'static str, TokenType>, // not sure about &'static, maybe it needs to be <String, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
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
            numeric_literal: 0.0,   // a stub value
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u8
            || self
                .source
                .as_str()
                .chars()
                .nth((self.current).into())
                .unwrap()
                == '\0'
    }

    fn scan_token(&mut self) {
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
            character if Self::is_digit(character) => {
                Self::number(self);
            }
            character if Self::is_alpha(character) => {
                Self::identifier(self);
            }
            _ => {
                error(self.line, String::from("Unexpected character"));
            }
        };
    }

    fn advance(&mut self) -> char {
        let next_character = self
            .source
            .as_str()
            .chars()
            .nth((self.current).into())
            .unwrap_or_default();

        self.current += 1;
        next_character
    }

    fn add_empty_token(&mut self, token_type: TokenType) {
        Self::add_token(self, token_type, None, None);
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal: Option<String>,
        numeric_literal: Option<f32>,
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
            Some(number.parse::<f32>().expect("Invalid number")), // handle a failure of parsing a number in better way which wouldn't stop a compiler
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

    fn is_alpha(character: char) -> bool {
        match character {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn is_alphanumeric(character: char) -> bool {
        Self::is_alpha(character) || Self::is_digit(character)
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = match self.source.get(self.start.into()..self.current.into()) {
            Some(text) => text,
            None => "", // handle it better way; it's still an error situation
        };

        let token_type = match self.keywords.get(text) {
            Some(token_type) => *token_type,
            None => TokenType::Identifier,
        };

        Self::add_empty_token(self, token_type);
    }
}
