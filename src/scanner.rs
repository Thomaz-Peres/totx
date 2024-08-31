use crate::{exception, token::Literal};

use super::token::{Token, TokenEnum};

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    // The start field points to the first character in the lexeme being scanned,
    // and current points at the character currently being considered.
    start: usize,
    current: usize,
    line: u32
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self)  {
        while !self.is_end()  {
            self.start = self.current;
            self.start += 1;

            self.scan_token()
        }

        self.tokens.push(Token::new(TokenEnum::EOF, String::new(), Default::default(), self.line));
    }

    fn scan_token(&mut self) {
        let c: Option<char> = self.advance();

        if c.is_none() {
            return exception::Exception::error(self.line, "");
        }

        let c = c.unwrap();

        match c {
            '(' => self.add_token(TokenEnum::LeftParen),
            ')' => self.add_token(TokenEnum::RightParen),
            '{' => self.add_token(TokenEnum::LeftBrace),
            '}' => self.add_token(TokenEnum::RightBrace),
            ',' => self.add_token(TokenEnum::Comma),
            '.' => self.add_token(TokenEnum::Dot),
            '-' => self.add_token(TokenEnum::Minus),
            '+' => self.add_token(TokenEnum::Plus),
            ';' => self.add_token(TokenEnum::SemiColon),
            '*' => self.add_token(TokenEnum::Star),
            '!' => self.add_token(if self.match_char('+'){ TokenEnum::BangEqual    } else { TokenEnum::Bang    }),
            '=' => self.add_token(if self.match_char('='){ TokenEnum::EqualEqual   } else { TokenEnum::Equal   }),
            '<' => self.add_token(if self.match_char('='){ TokenEnum::LessEqual    } else { TokenEnum::Less    }),
            '>' => self.add_token(if self.match_char('='){ TokenEnum::GreaterEqual } else { TokenEnum::Greater }),
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line
                    while self.peek().unwrap() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenEnum::Slash);
                }
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => self.current += 1,
            '"' => self.string(),
            _   => exception::Exception::error(self.line, "Unexpected character."),
        }
    }

    // pub fn next_token(&mut self) -> Result<Token, &'static str> {
    //     if self.is_end() {
    //         return Err("Token not found");
    //     }

    //     // let token;
    //     let mut term = String::from("");

    //     loop {
    //         let current_char: char = self.next_char();
    //         self.pos += 1;
    //         match self.estado {
    //             0 => {
    //                 if self.is_char(current_char) {
    //                     term.push(current_char);
    //                     self.estado = 1;
    //                 } else if self.is_digit(current_char) {
    //                     term.push(current_char);
    //                     self.estado = 3;
    //                 } else if self.is_space(current_char) {
    //                     self.estado = 0;
    //                 } else if self.is_operator(current_char) {
    //                     self.estado = 5;
    //                 } else {
    //                     continue;
    //                     // return Err("Token state not found");
    //                 }
    //             }
    //             1 => {
    //                 if self.is_char(current_char) || self.is_digit(current_char) || self.is_char_end(current_char) {
    //                     term.push(current_char);
    //                     self.estado = 1;
    //                 } else if self.is_space(current_char) || self.is_operator(current_char) {
    //                     self.estado = 2;
    //                 } else {
    //                     continue;
    //                     // return Err("Token state not found");
    //                 }
    //             }
    //             2 => {
    //                 self.back();
    //                 let token = Token::new_token(TokenEnum::Identifier, term);
    //                 return Ok(token);
    //             }
    //             3 => {
    //                 if self.is_digit(current_char) {
    //                     term.push(current_char);
    //                     self.estado = 3;
    //                 } else if !self.is_char(current_char) {
    //                     self.estado = 4;
    //                 } else {
    //                     continue;
    //                     // return Err("Token state not found");
    //                 }
    //             }
    //             4 => {
    //                 self.back();
    //                 let token = Token::new_token(TokenEnum::Number, term);
    //                 return Ok(token);
    //             }
    //             5 => {
    //                 term.push(current_char);
    //                 let token = Token::new_token(TokenEnum::Operator, term);
    //                 return Ok(token);
    //             }
    //             _ => {
    //                 continue;
    //                 // return Err("Token state not found");
    //             }
    //         }
    //     }
    // }

    fn advance(&mut self) -> Option<char> {
        self.get_char(self.current + 1)
    }

    fn add_token(&mut self, token_type: TokenEnum) {
        self.add_token_base(token_type, Literal::None)
    }

    fn add_token_base(&mut self, token_type: TokenEnum, literal: Literal) {
        let text = String::from(&self.source[self.start..self.current]);

        self.tokens.push(Token::new(token_type, text, literal, self.line))
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        if self.get_char(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn get_char(&self, index: usize) -> Option<char> {
        self.source.chars().nth(index)
    }

    fn peek(&self) -> Option<char> {
        if self.is_end() {
            return Some('\0')
        }

        self.get_char(self.current)
    }

    fn string(&mut self) {
        while self.peek().unwrap() != '"' && !self.is_end() {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return exception::Exception::error(self.line, "Unterminated string.");
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_base(TokenEnum::String, Literal::String(value));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_char(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    fn is_operator(&self, c: char) -> bool {
        c == '>' || c == '<' || c == '=' || c == '!'
    }

    fn is_white_space(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\r'
    }

    // Pretty much the same as I was doing before, happy
    fn is_end(&self) -> bool {
        &self.current >= &self.source.len()
    }

    fn is_char_end(&self, c: char) -> bool {
        c == '\0'
    }

    pub fn back(&mut self) {
        self.estado = 0;
        self.pos -= 1;
    }
}
