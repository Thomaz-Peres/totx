// Name	        Operators	Associates
// Equality	     == !=	    Left
// Comparison	> >= < <=	Left
// Term	          - +	    Left
// Factor	        / *	    Left
// Unary	      ! -	    Right

// Adding Comma operator precedence

// Name	           Operators	Associates
// Comma Op.	      ,    	    Left
// Equality	        == !=	    Left
// Comparison	  > >= < <=	    Left
// Term	             - +	    Left
// Factor	         / *	    Left
// Unary	         ! -	    Right

use crate::{
    ast::Expression, exception, token::{Token, TokenEnum}
};

// #[derive(Debug, Clone)]
// pub struct ParserError {
//     token: Token,
//     message: String,
// }

// pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 1
        }
    }

    pub fn parser(&mut self) -> exception::Result<Expression> {
        self.expression()
    }

    fn expression(&mut self) -> exception::Result<Expression> {
        self.equality()
    }

    fn comma(&mut self) -> exception::Result<Expression> {
        let mut expr = self.comparison()?;

        while self.matching(vec![TokenEnum::Comma]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary { operator: operator, left: Box::new(expr), right: Box::new(right) }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> exception::Result<Expression> {
        let mut expr = self.comparison()?;

        while self.matching(vec![TokenEnum::BangEqual, TokenEnum::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> exception::Result<Expression> {
        let mut expr = self.term()?;

        while self.matching(vec![TokenEnum::Greater, TokenEnum::GreaterEqual, TokenEnum::Less, TokenEnum::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> exception::Result<Expression> {
        let mut expr = self.factor()?;

        while self.matching(vec![TokenEnum::Plus, TokenEnum::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> exception::Result<Expression> {
        let mut expr = self.unary()?;

        while self.matching(vec![TokenEnum::Star, TokenEnum::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expression::Binary {
                operator: operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> exception::Result<Expression> {
        if self.matching(vec![TokenEnum::Bang, TokenEnum::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator: operator,
                right: Box::new(right),
            });
        }

        let x = self.primary()?;
        Ok(x)
    }

    fn primary(&mut self) -> exception::Result<Expression> {
        match self.peek().token_type {
            TokenEnum::True => {
                self.advance();
                return Ok(Expression::Literal { value: crate::token::Literal::Bool(true) });
            },
            TokenEnum::False => {
                self.advance();
                return Ok(Expression::Literal { value: crate::token::Literal::Bool(false) });
            },
            TokenEnum::Null => {
                self.advance();
                return Ok(Expression::Literal { value: crate::token::Literal::None });
            },
            TokenEnum::Number | TokenEnum::String => {
                self.advance();
                return Ok(Expression::Literal { value: self.previous().literal });
            }
            TokenEnum::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenEnum::RightParen, "Except ')' after expression.")?;
                return Ok(Expression::Grouping { expression: Box::new(expr) });
            }
            _ => {
                return Self::error(self.peek().clone(), "Expect expression.");
            }
        }

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
        &self.tokens[self.current] // Add unwrap or error, better than that
    }

    // Returns the most recently consumed token.
    fn previous(&self) -> Token {
        self.tokens[self.current].clone() // Add unwrap or erro after
    }

    fn error<T>(token: Token, message: &str) -> exception::Result<T> {
        if token.token_type == TokenEnum::EOF {
            exception::Exception::error(token.line, " at end", message);
        }
        else {
            let mut where_r = String::from(" at '");
            where_r.push_str(token.lexeme.as_str());
            exception::Exception::error(token.line, where_r.as_str(), message);
        }

        Err(exception::Exception::new(token.line, "", message))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenEnum::SemiColon {
                return ();
            }

            match self.peek().token_type {
                TokenEnum::Class
                | TokenEnum::Fun
                | TokenEnum::Var
                | TokenEnum::For
                | TokenEnum::If
                | TokenEnum::While
                | TokenEnum::Print
                | TokenEnum::Return => (),
                _ => ()
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenEnum, message: &str) -> exception::Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Self::error(self.peek().clone(), message)
    }
}
