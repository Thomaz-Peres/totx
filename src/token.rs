// use core::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenEnum {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
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

    // Literal
    String,
    Integer,
    Identifier,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenEnum,
    lexeme: String,
    literal: Literal,
    line: u32
}

impl FromStr for TokenEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "String" => Ok(TokenEnum::String),
            "And"    => Ok(TokenEnum::And),
            "Class"  => Ok(TokenEnum::Class),
            "Else"   => Ok(TokenEnum::Else),
            "False"  => Ok(TokenEnum::False),
            "Fun"    => Ok(TokenEnum::Fun),
            "For"    => Ok(TokenEnum::For),
            "If"     => Ok(TokenEnum::If),
            "Null"   => Ok(TokenEnum::Null),
            "Or"     => Ok(TokenEnum::Or),
            "Print"  => Ok(TokenEnum::Print),
            "Return" => Ok(TokenEnum::Return),
            "Super"  => Ok(TokenEnum::Super),
            "This"   => Ok(TokenEnum::This),
            "True"   => Ok(TokenEnum::True),
            "Var"    => Ok(TokenEnum::Var),
            "While"  => Ok(TokenEnum::While),
            "EOF"    => Ok(TokenEnum::EOF),
            _        => Err(())
        }
    }
}

// impl fmt::Display for TokenEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Bool(bool),
    None
}

impl Default for Literal {
    fn default() -> Self {
        Literal::None
    }
}


// impl fmt::Display for Literal {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

impl Token {
    pub fn new(token_type: TokenEnum, lexeme: String, literal: Literal, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} - {} - {:?}", self.token_type, self.lexeme, self.literal)
    }
}
