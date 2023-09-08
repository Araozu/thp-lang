use super::Transpilable;
use crate::ast_types::Expression;

impl Transpilable for Expression<'_> {
    /// Transpiles an Expression to JS
    ///
    /// Right now the expressions in the grammar are:
    /// - Number
    /// - String
    /// - Boolean
    /// - Identifier
    fn transpile(&self) -> String {
        match self {
            Expression::Number(value) => String::from(*value),
            Expression::String(value) => {
                format!("\"{}\"", *value)
            }
            Expression::Boolean(value) => String::from(if *value { "true" } else { "false" }),
            Expression::Identifier(value) => String::from(*value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_types::Expression;

    #[test]
    fn should_transpile_number() {
        let str = String::from("42");
        let exp = Expression::Number(&str);
        let result = exp.transpile();

        assert_eq!("42", result);
    }

    #[test]
    fn should_transpile_string() {
        let str = String::from("Hello world");
        let exp = Expression::String(&str);
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
        let exp = Expression::Identifier(&s);
        let result = exp.transpile();

        assert_eq!("newValue", result);
    }
}
