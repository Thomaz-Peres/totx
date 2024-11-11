use crate::{
    ast::Expression,
    token::{Token, TokenEnum}
};

#[derive(Debug, Clone)]
struct ParserError {
    token: Token,
    message: String,
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison().unwrap();

        while self.matching(vec![TokenEnum::BangEqual, TokenEnum::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison().unwrap();
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term().unwrap();

        while self.matching(vec![TokenEnum::Greater, TokenEnum::GreaterEqual, TokenEnum::Less, TokenEnum::LessEqual]) {
            let operator = self.previous();
            let right = self.term().unwrap();
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor().unwrap();

        while self.matching(vec![TokenEnum::Plus, TokenEnum::Minus]) {
            let operator = self.previous();
            let right = self.factor().unwrap();
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary().unwrap();

        while self.matching(vec![TokenEnum::Star, TokenEnum::Slash]) {
            let operator = self.previous();
            let right = self.unary().unwrap();
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if self.matching(vec![TokenEnum::Bang, TokenEnum::Minus]) {
            let operator = self.previous();
            let right = self.unary().unwrap();
            return Ok(Expression::Unary {
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(self.primary().unwrap())
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.matching(vec![TokenEnum::False]) {
            return Ok(Expression::Literal { value: crate::token::Literal::Bool(false) });
        }

        if self.matching(vec![TokenEnum::True]) {
            return Ok(Expression::Literal { value: crate::token::Literal::Bool(true) });
        }

        if self.matching(vec![TokenEnum::Null]) {
            return Ok(Expression::Literal { value: crate::token::Literal::None });
        }

        if self.matching(vec![TokenEnum::Number, TokenEnum::String]) {
            return Ok(Expression::Literal { value: self.previous().literal });
        }

        return Ok(Expression::Literal { value: self.previous().literal });
        // if self.matching(vec![TokenEnum::LeftParen]) {
        //     let expr = self.expression().unwrap();
        //     consume(TokenEnum::RightParen, "Except ')' after expression.");
        //     return Ok(Expr::Grouping { expression: Box::new(expr) });
        // }

        // return Exception::error(120, "");
    }

    // This consumes the token and returns true. Otherwise, it returns false and leaves the current token alone.
    fn matching(&mut self, types: Vec<TokenEnum>) -> bool {
        for token_type in types.into_iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenEnum) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }

        self.previous() // This looks wrong. problably use lifetimes here.
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenEnum::EOF;
    }

    // returns the current token we have yet to consume
    fn peek(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap() // Add unwrap or error, better than that
    }

    // Returns the most recently consumed token.
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone() // Add unwrap or erro after
    }

    // fn error() -> ParserError {

    // }

    fn consume(&self, token_type: TokenEnum, message: &str) -> Result<Token> {
        if (self.check(token_type)) {
            // return Ok(self.advance());
        }

        panic!();
    }
}
