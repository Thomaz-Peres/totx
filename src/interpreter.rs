use crate::{ast::Expression, exception::{self, Exception}, token::{Literal, TokenEnum}};


#[derive(Debug, Clone)]
pub struct Interpreter;

type EvaluateResult<T> = exception::Result<T>;

impl Interpreter {
    // We eargely produced the runtime value way back during scanning and stuffed it in the token. The parser took that value and stuck it in the literal tree node, so to evaluate a literal, we simply pull it back out.
    pub fn evaluate(&self, expr: &Expression) -> EvaluateResult<Literal> {
        let result = match expr {
            Expression::Literal { value } => value.clone(),
            Expression::Grouping { expression } => self.evaluate(expression)?,
            Expression::Unary { operator, right } => match operator.token_type {
                TokenEnum::Minus => Literal::Number(check_number(self.evaluate(right)?)?),
                TokenEnum::Bang => Literal::Bool(!is_truthy(self.evaluate(right)?)),
                _ => return Exception::error(17, "Interpreter.rs", "message")
            },
            Expression::Binary {
                operator,
                left,
                right
            } => {
                let right = self.evaluate(right)?;
                let left = self.evaluate(left)?;
                match operator.token_type {
                    TokenEnum::Minus => Literal::Number(check_number(left)? - check_number(right)?),
                    TokenEnum::Slash => Literal::Number(check_number(left)? / check_number(right)?),
                    TokenEnum::Star => Literal::Number(check_number(left)? * check_number(right)?),
                    TokenEnum::Plus => {
                        match (left, right) {
                            (Literal::Number(left), Literal::Number(right)) => Literal::Number(left + right),
                            (Literal::String(left), Literal::String(right)) => Literal::String(left + &right),
                            _ => return Exception::error(1, "Interpreter.rs", "Must be all string or number for PLUS (+)")
                        }
                    },
                    TokenEnum::Greater => Literal::Bool(check_number(left)? > check_number(right)?),
                    TokenEnum::GreaterEqual => Literal::Bool(check_number(left)? >= check_number(right)?),
                    TokenEnum::Less => Literal::Bool(check_number(left)? < check_number(right)?),
                    TokenEnum::LessEqual => Literal::Bool(check_number(left)? <= check_number(right)?),
                    TokenEnum::BangEqual => Literal::Bool(!(left == right)),
                    TokenEnum::EqualEqual => Literal::Bool(left == right),
                    _ => return Exception::error(19, "Interpreter.rs", "message")
                }
            },
            _ => return Exception::error(19, "Interpreter.rs", "message")
        };

        Ok(result)
    }
}

fn check_number(value: Literal) -> EvaluateResult<i64> {
    match value {
        Literal::Number(value) => Ok(value),
        _ => Exception::error(1, "Interpreter.rs/to_number", "Must be a number")
    }
}

fn is_truthy(value: Literal) -> bool {
    match value {
        Literal::Bool(false) | Literal::None => false,
        _ => true
    }
}



#[cfg(test)]
mod tests {
    use core::assert_eq;

    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    #[test]
    fn sum_numbers() {
        let mut scan = Scanner::new("5 + 5");
        let tokens = scan.scan_tokens().unwrap();
        let mut binding = Parser::new(tokens);
        let parse = binding.parser();
        let interpreter = Interpreter.evaluate( &parse.unwrap()).unwrap();

        assert_eq!(interpreter, Literal::Number(10));
    }

    #[test]
    fn concat_string() {
        let mut scan = Scanner::new(r#""te" + "st""#);
        let tokens = scan.scan_tokens().unwrap();
        let mut binding = Parser::new(tokens);
        let parse = binding.parser();
        let interpreter = Interpreter.evaluate( &parse.unwrap()).unwrap();

        assert_eq!(interpreter, Literal::String("test".to_string()));
    }

    #[test]
    fn should_fail_string_number() {
        let mut scan = Scanner::new(r#""te" + 5"#);
        let tokens = scan.scan_tokens().unwrap();
        let mut binding = Parser::new(tokens);
        let parse = binding.parser();
        let interpreter = Interpreter.evaluate( &parse.unwrap());

        assert!(interpreter.is_err());
        // Will try using assert_eq
    }

    #[test]
    fn expressions() {
        let mut scan = Scanner::new("(10 + 2) / 2");
        let tokens = scan.scan_tokens().unwrap();
        let mut binding = Parser::new(tokens);
        let parse = binding.parser();
        let interpreter = Interpreter.evaluate( &parse.unwrap()).unwrap();

        assert_eq!(interpreter, Literal::Number(6));
    }
}
