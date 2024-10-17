use crate::{exception, token::{Literal, Token}};

// #[derive(Debug, Clone)]
// pub struct Parser {
//     operator: Token,
//     left:  Expr,
//     right: Expr,
// }

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept(&self, expr: &Expr) -> exception::Result<String> {
        match expr {
            Self::Binary { operator, left, right, } => {
                self.parenthesize(&operator.lexeme, &[*left.clone(), *right.clone()])
            },
            Self::Grouping { expression } => {
                self.parenthesize("group", &[*expression.clone()])
            },
            Self::Literal { value } => {
                if value.eq(&Literal::None) {
                    ()
                }

                Ok(value.to_string())
            }
            Self::Unary { operator, right } => {
                self.parenthesize(&operator.lexeme, &[*right.clone()])
            },
        }
    }

    pub fn print(&self) -> exception::Result<String> {
        self.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[Expr]) -> exception::Result<String> {
        let mut builder: String = String::new();

        builder.push('(');
        builder.push_str(name);

        for expr in exprs {
            builder.push(' ');
            builder.push_str(&self.accept(expr).unwrap());
        }

        builder.push(')');

        Ok(builder)
    }

    fn reverse_polish_notation(&self) -> exception::Result<String> {
        let mut builder: String = String::new();
        match self {
            // For a binary operator (e.g., +, -, *, /)
            Expr::Binary { operator, left, right } => {
                let left_rpn = left.reverse_polish_notation();
                let right_rpn = right.reverse_polish_notation();

                builder.push_str(&format!("{} {} {}", left_rpn.unwrap(), right_rpn.unwrap(), operator.lexeme));
                Ok(builder)
            }
            Expr::Grouping { expression } => {
                expression.reverse_polish_notation()
            }
            Expr::Literal { value } => {
                Ok(value.to_string())
            }
            Expr::Unary { operator, right } => {
                let right_rpn = right.reverse_polish_notation();
                builder.push_str(&format!("{} {}", right_rpn.unwrap(), operator.lexeme));
                Ok(builder)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenEnum;
    use super::*;

    #[test]
    fn test_literal() {
        let parser = Expr::print(&Expr::Literal { value: Literal::String("teste".to_string()) });

        assert!(parser.is_ok());
    }

    #[test]
    fn test_numbers() {
        let expression = Expr::Binary {
            operator: Token::new(TokenEnum::Star, "*", Literal::None, 1),
            left: Box::new(
                Expr::Unary {
                         operator: Token::new(TokenEnum::Minus, "-", Literal::None, 1),
                         right: Box::new(Expr::Literal { value: Literal::Number(123) })
                    }),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal { value: Literal::Number(123) })
            })
        };

        let parser = Expr::reverse_polish_notation(&expression);
        assert!(parser.is_ok());
        assert_eq!(parser.unwrap(), "(* (- 123) (group 123))")
    }

    #[test]
    fn test_reverse_polish_notation() {
        let expression: Expr = Expr::Binary {
            operator: Token::new(TokenEnum::Star, "*", Literal::None, 1),
            left: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    operator: Token::new(TokenEnum::Plus, "+", Literal::None, 1),
                    left: Box::new(Expr::Literal { value: Literal::Number(1) }),
                    right: Box::new(Expr::Literal { value: Literal::Number(2) })
                })
            }),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    operator: Token::new(TokenEnum::Minus, "-", Literal::None, 1),
                    left: Box::new(Expr::Literal { value: Literal::Number(4) }),
                    right: Box::new(Expr::Literal { value: Literal::Number(3) })
                })
            })
        };

        let parser = Expr::reverse_polish_notation(&expression);
        assert!(parser.is_ok());
        assert_eq!(parser.unwrap(), "1 2 + 4 3 - *")
    }
}

// use crate::{
//     token::{Token, TokenEnum},
//     Scanner,
// };

// pub struct Parser {
//     scanner: Scanner,
//     current_token: Token,
// }

// impl Parser {
//     // O parser recebe o scanner (analisador lexico) como parametro pois a cada procedimento,
//     // invoca-o sob demanda.
//     pub fn new(scanner: &Scanner, current_token: &Token) -> Parser {
//         Parser {
//             scanner: scanner.clone(),
//             current_token: current_token.clone()
//         }
//     }

//     pub fn e(&self) {
//         self.t();
//         self.el();
//     }

//     pub fn el(&self) {
//         if self.current_token.get_type() != &TokenEnum::Operator {
//             self.op();
//             self.t();
//             self.el();
//         }
//     }

//     pub fn t(&self) -> Result<(), &'static str> {
//         let x = self.current_token.get_type();
//         if (x == &TokenEnum::Identifier && x == &TokenEnum::Number)
//         {
//             Ok(())
//         } else {
//             Err("ID or Number expected")
//         }
//         // match  {
//         //     TokenEnum::Identifier | TokenEnum::Number => Ok(()),
//         //     _ => Err("ID or Number expected"),
//         // }
//     }

//     pub fn op(&self) -> Result<(), &'static str> {
//         match self.current_token.get_type() {
//             TokenEnum::Operator => Ok(()),
//             _ => Err("Operator expected"),
//         }
//     }
// }
