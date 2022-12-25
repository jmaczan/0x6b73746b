use crate::lexical_analysis::{TokenType, Token};

use super::expression::{Binary, Expr, Literal};

struct Parser {
    pub tokens: Vec<Token>,
    pub current: u8
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    fn expression() {
        Self::equality()
    }

    fn equality() -> Box<dyn Expr> {
        let mut expr = Self::comparison();

        while Self::match_token(Vec::from([TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = Self::previous();
            let right = Self::comparison();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right
            };
        }

        Box::new(expr)
    }

    fn match_token(tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if Self::check(token) {
                Self::advance();
                true
            }
        }

        false
    }

    fn check(token: TokenType) -> bool {
        if Self::is_at_end() {
            false
        }

        Self::peek().type == type
    }

    fn advance() -> TokenType {
        if !Self::is_at_end() {
            self.current = self.current + 1;
        }

        Self::previous()
    }

    fn is_at_end() -> bool {
        Self::peek().type == '\0'
    }

    fn peek(&self) -> TokenType {
        self.tokens[self.current]
    }

    fn previous(&self) -> TokenType {
        self.tokens[self.current - 1]
    }

    fn comparison(&self) {
        let mut expr = Self::term();

        while match_token(Vec::from([TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual])) {
            let operator = Self::previous(&self);
            let right = Self::term();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right
            }
        }

        expr
    }

    fn term(&self) {
        let mut expr = Self::factor();

        while match_token(Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator = Self::previous(&self);
            let right = Self::factor(&self);
            expr = Binary {
                left: expr,
                operator,
                right
            }
        }

        expr
    }

    fn factor(&self) {
        let expr = Self::unary(&self);

        while match_token(Vec::from([TokenType::Slash, TokenType::Star])) {
            let operator = Self::previous(&self);
            let right = Self::unary(&self);
            expr = Binary {
                left: expr,
                operator,
                right
            }
        }

        expr
    }

    fn unary(&self) {
        if Self::match_token(Vec::from([TokenType::Bang, TokenType:: Minus])) {
            let operator = Self::previous(&self);
            let right = Self::unary(&self);
            Unary {
                operator,
                right
            }
        }

        Self::primary(&self)
    }

    fn primary(&self) {
        if Self::match_token(Vec::from([TokenType::False])) {
            Literal {
                value: "false".to_string()
            }
        }

        if Self::match_token(Vec::from([TokenType::False])) {
            // TODO
        }
    }
}
