use crate::lexical_analysis::{Token, TokenType};

use super::expression::{Binary, Expr, Literal, Unary};

struct Parser {
    pub tokens: Vec<Token>,
    pub current: u8,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&self) -> Box<dyn Expr> {
        Self::equality(self)
    }

    fn equality(&self) -> Box<dyn Expr> {
        let mut expr = Self::comparison(self);

        while Self::match_token(
            self,
            Vec::from([TokenType::BangEqual, TokenType::EqualEqual]),
        ) {
            let operator = Self::previous(self);
            let right = Self::comparison(self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            });
        }

        expr
    }

    fn match_token(&self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if Self::check(self, token) {
                Self::advance(self);
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if Self::is_at_end(self) {
            return false;
        }

        Self::peek(self).token_type == token_type
    }

    fn advance(&self) -> TokenType {
        if !Self::is_at_end(self) {
            self.current = self.current + 1;
        }

        Self::previous(self).token_type
    }

    fn is_at_end(&self) -> bool {
        Self::peek(self).token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        match self.tokens.get(self.current as usize) {
            Some(token) => token,
            None => &Token {
                // TODO - I might be wrong here and maybe either nothing should be returned or it should be a different value than Nil - likely Eof
                token_type: TokenType::Nil, // Maybe TokenType::Eof
                lexeme: "".to_string(),
                literal: "".to_string(),
                numeric_literal: 0.0,
                line: 0,
            },
        }
    }

    fn previous(&self) -> &Token {
        match self.tokens.get((self.current - 1) as usize) {
            Some(token) => token,
            None => &Token {
                // TODO - I might be wrong here and maybe either nothing should be returned or it should be a different value than Nil - likely Eof
                token_type: TokenType::Nil,
                lexeme: "".to_string(),
                literal: "".to_string(),
                numeric_literal: 0.0,
                line: 0,
            },
        }
    }

    fn comparison(&self) -> Box<dyn Expr> {
        let mut expr = Self::term(self);

        while Self::match_token(
            self,
            Vec::from([
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]),
        ) {
            let operator = Self::previous(&self);
            let right = Self::term(self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            });
        }

        expr
    }

    fn term(&self) -> Box<dyn Expr> {
        let mut expr = Self::factor(self);

        while Self::match_token(self, Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator = Self::previous(&self);
            let right = Self::factor(&self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            })
        }

        expr
    }

    fn factor(&self) -> Box<dyn Expr> {
        let expr = Self::unary(&self);

        while Self::match_token(self, Vec::from([TokenType::Slash, TokenType::Star])) {
            let operator = Self::previous(&self);
            let right = Self::unary(&self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            });
        }

        expr
    }

    fn unary(&self) -> Box<dyn Expr> {
        if Self::match_token(self, Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = Self::previous(&self);
            let right = Self::unary(&self);
            return Box::new(Unary {
                operator: operator.clone(),
                right,
            });
        }

        Self::primary(&self)
    }

    fn primary(&self) -> Box<dyn Expr> {
        if Self::match_token(self, Vec::from([TokenType::False])) {
            return Box::new(Literal {
                value: "false".to_string(),
            });
        }

        if Self::match_token(self, Vec::from([TokenType::True])) {
            return Box::new(Literal {
                value: "true".to_string(),
            });
        }

        if Self::match_token(self, Vec::from([TokenType::Nil])) {
            return Box::new(Literal {
                value: "true".to_string(),
            });
        }
    }
}
