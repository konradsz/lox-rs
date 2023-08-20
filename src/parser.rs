use crate::{
    expr::{Expr, LiteralType},
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize, // TODO: interior mutability? peek?
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().to_owned();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().to_owned();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().to_owned();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().to_owned();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().to_owned();
            let right = self.unary();
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(&[TokenType::False]) {
            Expr::Literal {
                value: LiteralType::Boolean(false),
            }
        } else if self.match_types(&[TokenType::True]) {
            Expr::Literal {
                value: LiteralType::Boolean(true),
            }
        } else if self.match_types(&[TokenType::Nil]) {
            Expr::Literal {
                value: LiteralType::Null,
            }
        } else if self.match_types(&[TokenType::String, TokenType::Number]) {
            Expr::Literal {
                value: self.previous().literal.clone().unwrap(),
            }
        } else if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.expression();
            if self.check(&TokenType::RightParen) {
                self.advance();
            } else {
                // TODO: better error handling
                panic!("Expect ')' after expression");
            }
            Expr::Grouping {
                expression: Box::new(expr),
            }
        } else {
            panic!("Expect expression");
        }
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek() == t;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &TokenType {
        &self.tokens[self.current].token_type
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].token_type == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
