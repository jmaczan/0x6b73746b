use crate::lexical_analysis::{Token, TokenType};

use super::expression::{self, Binary, Expr, Grouping, Literal, Unary};

struct Parser {
    pub tokens: Vec<Token>,
    pub current: u8,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Box<dyn Expr> {
        Self::equality(self)
    }

    fn equality(&mut self) -> Box<dyn Expr> {
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

    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
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

    fn advance(&mut self) -> TokenType {
        if !Self::is_at_end(self) {
            self.current = self.current + 1;
        }

        Self::previous(self).token_type
    }

    fn is_at_end(&self) -> bool {
        Self::peek(self).token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        match self.tokens.get(self.current as usize) {
            Some(token) => token.to_owned(),
            None => Token {
                // TODO - I might be wrong here and maybe either nothing should be returned or it should be a different value than Nil - likely Eof
                token_type: TokenType::Nil, // Maybe TokenType::Eof
                lexeme: "".to_string(),
                literal: "".to_string(),
                numeric_literal: 0.0,
                line: 0,
            },
        }
    }

    fn previous(&self) -> Token {
        match self.tokens.get((self.current - 1) as usize) {
            Some(token) => token.to_owned(),
            None => Token {
                // TODO - I might be wrong here and maybe either nothing should be returned or it should be a different value than Nil - likely Eof
                token_type: TokenType::Nil,
                lexeme: "".to_string(),
                literal: "".to_string(),
                numeric_literal: 0.0,
                line: 0,
            },
        }
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
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

    fn term(&mut self) -> Box<dyn Expr> {
        let mut expr = Self::factor(self);

        while Self::match_token(self, Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator = Self::previous(self);
            let right = Self::factor(self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            })
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut expr = Self::unary(self);

        while Self::match_token(self, Vec::from([TokenType::Slash, TokenType::Star])) {
            let operator = Self::previous(self);
            let right = Self::unary(self);
            expr = Box::new(Binary {
                left: expr,
                operator: operator.clone(),
                right,
            });
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if Self::match_token(self, Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = Self::previous(self);
            let right = Self::unary(self);
            return Box::new(Unary {
                operator: operator.clone(),
                right,
            });
        }

        Self::primary(self)
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        if Self::match_token(self, Vec::from([TokenType::False])) {
            return Box::new(Literal {
                value: "false".to_string(), // it's likely that I need to represent Literal in a different way than a String value
            });
        }

        if Self::match_token(self, Vec::from([TokenType::True])) {
            return Box::new(Literal {
                value: "true".to_string(),
            });
        }

        if Self::match_token(self, Vec::from([TokenType::Nil])) {
            return Box::new(Literal {
                value: "nil".to_string(),
            });
        }

        if Self::match_token(self, Vec::from([TokenType::Number, TokenType::String])) {
            return Box::new(Literal {
                value: self.previous().literal.clone(),
            });
        }

        // TODO for super keyword
        // if Self::match_token(self, Vec::from([TokenType::Super])) {
        //     let keyword: &Token = self.previous();
        //     self.consume(TokenType::Dot, "Expect '.' after 'super'.");
        //     let method: &Token = self.consume(TokenType::Identifier, "Expect superclass method name.");
        //     return Box::new(Literal {
        //         value: self.previous().literal
        //     });
        // }

        // TODO: Revert if; commented out to have all paths returning a value
        // if Self::match_token(self, Vec::from([TokenType::LeftParen])) {
        let expr = self.expression();

        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_string(),
        ); // TODO use result of consume, especially if Err was returned
        return Box::new(Grouping { expression: expr });
        // }

        // Err(self.error(self.peek(), "Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<TokenType, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: Token, message: String) -> ParseError {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end".to_string(), message.to_string());
        } else {
            self.report(token.line, format!(" at '{}'", token.lexeme), message);
        }

        ParseError {}
    }

    fn report(&self, line: u8, where_error: String, message: String) {}

    fn synchronize(&mut self) {
        self.advance();

        loop {
            if self.is_at_end() {
                break;
            }

            if self.previous().token_type == TokenType::Semicolon {
                break;
            }

            if [
                TokenType::Class,
                TokenType::Fun,
                TokenType::Var,
                TokenType::For,
                TokenType::If,
                TokenType::While,
                TokenType::Print,
                TokenType::Return,
            ]
            .contains(&self.peek().token_type)
            {
                break;
            }

            self.advance();
        }
    }

    fn parse(&mut self) -> Box<dyn Expr> {
        return self.expression(); // TODO handle Result from primary up to this place
    }
}

struct ParseError {}
