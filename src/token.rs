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
    Number,
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
    pub token_type: TokenEnum,
    pub lexeme: String,
    pub literal: Literal,
    line: u32
}

impl FromStr for TokenEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "String" | "string" => Ok(TokenEnum::String),
            "And"    | "and"    => Ok(TokenEnum::And),
            "Class"  | "class"  => Ok(TokenEnum::Class),
            "Else"   | "else"   => Ok(TokenEnum::Else),
            "False"  | "false"  => Ok(TokenEnum::False),
            "Fun"    | "fun"    => Ok(TokenEnum::Fun),
            "For"    | "for"    => Ok(TokenEnum::For),
            "If"     | "if"     => Ok(TokenEnum::If),
            "Null"   | "null"   => Ok(TokenEnum::Null),
            "Or"     | "or"     => Ok(TokenEnum::Or),
            "Print"  | "print"  => Ok(TokenEnum::Print),
            "Return" | "return" => Ok(TokenEnum::Return),
            "Super"  | "super"  => Ok(TokenEnum::Super),
            "This"   | "this"   => Ok(TokenEnum::This),
            "True"   | "true"   => Ok(TokenEnum::True),
            "Var"    | "var"    => Ok(TokenEnum::Var),
            "While"  | "while"  => Ok(TokenEnum::While),
            "EOF"               => Ok(TokenEnum::EOF),
            _                   => Err(())
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
    Number(i64),
    Bool(bool),
    None
}

impl Default for Literal {
    fn default() -> Self {
        Literal::None
    }
}

impl ToString for Literal {

    fn to_string(&self) -> String {
        match self {
            Literal::None => String::new(),
            Literal::Bool(value) => String::from(value.to_string()),
            Literal::Number(value) => String::from(value.to_string()),
            Literal::String(value) => String::from(value),
        }
    }
}

impl Token {
    pub fn new(token_type: TokenEnum, lexeme: &str, literal: Literal, line: u32) -> Self {
        let lexeme = lexeme.to_string();
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
