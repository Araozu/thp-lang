use crate::{codegen::Transpilable, php_ast::PPrimary};

impl Transpilable for PPrimary<'_> {
    fn transpile(&self) -> String {
        match self {
            PPrimary::IntegerLiteral(value) => value.to_string(),
            PPrimary::FloatingLiteral(value) => value.to_string(),
            PPrimary::StringLiteral(value) => format!("\"{}\"", value),
            PPrimary::Variable(name) => format!("${}", name),
            PPrimary::Symbol(name) => format!("{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{codegen::Transpilable, php_ast::PPrimary};

    #[test]
    fn should_transpile_empty_string() {
        let input = String::from("");
        let ast = PPrimary::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"\"", output)
    }

    #[test]
    fn should_transpile_string() {
        let input = String::from("abc");
        let ast = PPrimary::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"abc\"", output)
    }

    #[test]
    fn should_transpile_string_with_quotes() {
        let input = String::from("a\\\"b\\\"c");
        let ast = PPrimary::StringLiteral(&input);
        let output = ast.transpile();

        assert_eq!("\"a\\\"b\\\"c\"", output)
    }

    #[test]
    fn should_transpile_int() {
        let input = String::from("322");
        let ast = PPrimary::IntegerLiteral(&input);
        let output = ast.transpile();

        assert_eq!("322", output)
    }

    #[test]
    fn should_transpile_floating() {
        let input = String::from("322.644");
        let ast = PPrimary::FloatingLiteral(&input);
        let output = ast.transpile();

        assert_eq!("322.644", output)
    }

    #[test]
    fn should_transpile_variable() {
        let input = String::from("name");
        let ast = PPrimary::Variable(&input);
        let output = ast.transpile();

        assert_eq!("$name", output)
    }
}
