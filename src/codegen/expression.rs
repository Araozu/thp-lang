use super::Transpilable;
use crate::syntax::ast::Expression;

impl Transpilable for Expression {
    /// Transpiles an Expression to PHP
    ///
    /// Right now the expressions in the grammar are:
    /// - Number
    /// - String
    /// - Boolean
    /// - Identifier
    fn transpile(&self) -> String {
        match self {
            Expression::Number(value) => format!("{}", value),
            Expression::String(value) => {
                format!("{}", *value)
            }
            Expression::Boolean(value) => String::from(if *value { "true" } else { "false" }),
            Expression::Identifier(value) => format!("{}", *value),
            Expression::FunctionCall(f) => f.transpile(),
            Expression::BinaryOperator(left_expr, right_expr, operator) => {
                format!(
                    "{}{}{}",
                    left_expr.transpile(),
                    operator,
                    right_expr.transpile()
                )
            }
            Expression::UnaryOperator(operator, expression) => {
                format!("{}{}", operator, expression.transpile())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::Expression;

    #[test]
    fn should_transpile_number() {
        let str = String::from("42");
        let exp = Expression::Number(Box::new(str));
        let result = exp.transpile();

        assert_eq!("42", result);
    }

    #[test]
    fn should_transpile_string() {
        let str = String::from("\"Hello world\"");
        let exp = Expression::String(Box::new(str));
        let result = exp.transpile();

        assert_eq!("\"Hello world\"", result);
    }

    #[test]
    fn should_transpile_boolean() {
        let exp = Expression::Boolean(true);
        let result = exp.transpile();

        assert_eq!("true", result);
    }

    #[test]
    fn should_transpile_identifier() {
        let s = String::from("newValue");
        let exp = Expression::Identifier(Box::new(s));
        let result = exp.transpile();

        assert_eq!("newValue", result);
    }
}
