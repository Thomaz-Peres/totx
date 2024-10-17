use crate::{ast::Expr, exception, token::{Token, TokenEnum}};

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

    fn expression(&mut self) -> exception::Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> exception::Result<Expr> {
        let expr = self.comparison();

        while self.matching(vec![TokenEnum::BangEqual, TokenEnum::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary { operator: operator.to_owned(), left: expr, right: right }
        }

        Ok(expr)
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

    fn advance(&mut self) -> &Token {
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
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap() // Add unwrap or erro  after
    }
}
