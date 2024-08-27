use crate::{codegen::Transpilable, php_ast::PFile};

impl Transpilable for PFile<'_> {
    fn transpile(&self) -> String {
        let mut fragments = vec![String::from("<?php\n")];

        for statement in self.statements.iter() {
            fragments.push(statement.transpile());
        }

        fragments.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::Transpilable,
        php_ast::{PExpresssion, PFile, PPrimary, PStatement},
    };

    #[test]
    fn should_transpile_empty_file() {
        let ast = PFile { statements: vec![] };
        let output = ast.transpile();

        assert_eq!("<?php\n", output);
    }

    #[test]
    fn should_transpile_expr_statement() {
        let value = String::from("Hello world!");
        let ast = PFile {
            statements: vec![PStatement::ExpressionStatement(PExpresssion::Primary(
                PPrimary::StringLiteral(&value),
            ))],
        };
        let output = ast.transpile();

        assert_eq!("<?php\n\n\"Hello world!\";", output);
    }
}
