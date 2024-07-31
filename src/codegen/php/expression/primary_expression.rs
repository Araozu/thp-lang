use crate::{codegen::Transpilable, php_ast::PhpPrimaryExpression};

impl Transpilable for PhpPrimaryExpression<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpPrimaryExpression::IntegerLiteral(value) => value.to_string(),
            PhpPrimaryExpression::FloatingLiteral(value) => value.to_string(),
            PhpPrimaryExpression::StringLiteral(value) => format!("\"{}\"", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{codegen::Transpilable, php_ast::PhpPrimaryExpression};

    #[test]
    fn should_transpile_empty_string() {
        let input = String::from("");
        let ast = PhpPrimaryExpression::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"\"", output)
    }

    #[test]
    fn should_transpile_string() {
        let input = String::from("abc");
        let ast = PhpPrimaryExpression::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"abc\"", output)
    }

    #[test]
    fn should_transpile_string_with_quotes() {
        let input = String::from("a\\\"b\\\"c");
        let ast = PhpPrimaryExpression::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"a\\\"b\\\"c\"", output)
    }

    #[test]
    fn should_transpile_int() {
        let input = String::from("322");
        let ast = PhpPrimaryExpression::IntegerLiteral(&input);
        let output = ast.transpile();

        assert_eq!("322", output)
    }

    #[test]
    fn should_transpile_floating() {
        let input = String::from("322.644");
        let ast = PhpPrimaryExpression::FloatingLiteral(&input);
        let output = ast.transpile();

        assert_eq!("322.644", output)
    }
}
