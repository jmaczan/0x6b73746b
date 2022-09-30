use std::{error::Error, ops::Sub};

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
    literal: String, // originally Object
    line: u8,
}

impl Token {
    pub fn to_string(&self) -> String {
        //self.token_type +
        self.lexeme.to_owned() + " " + &self.literal.to_owned()
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

    pub fn scan_tokens(&self) -> &Vec<Token> {
        while !Self::is_at_end(self) {
            self.start = self.current;
            scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            literal: "".to_owned(), // originally Null
            line: self.line,
        });

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u8
    }

    fn scan_token(&self) {
        // https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs
        // https://craftinginterpreters.com/scanning.html#recognizing-lexemes
        let character = Self::advance(self);
    }

    fn advance(&self) -> char {
        self.source
            .as_str()
            .chars()
            .nth((&self.current + 1).into())
            .unwrap()
    }

    fn add_empty_token(&self, token_type: TokenType) {
        Self::add_token(self, token_type, None);
    }

    fn add_token(&self, token_type: TokenType, literal: Option<String>) {
        let text = self.source.get(self.start.into()..self.current.into());
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        })
    }
}
