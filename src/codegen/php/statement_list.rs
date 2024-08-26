use crate::{codegen::Transpilable, php_ast::PhpAst};

impl Transpilable for PhpAst<'_> {
    fn transpile(&self) -> String {
        let mut fragments = vec![String::from("<?php\n")];

        for statement in self.statements.iter() {
            fragments.push(statement.transpile());
        }

        fragments.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::Transpilable,
        php_ast::{
            PhpAssignmentExpression, PhpAst, PhpExpression, PhpPrimaryExpression, PhpStatement,
        },
    };

    #[test]
    fn should_transpile_empty_file() {
        let ast = PhpAst { statements: vec![] };
        let output = ast.transpile();

        assert_eq!("<?php\n", output);
    }

    /*
    #[test]
    fn should_transpile_expr_statement() {
        let value = String::from("Hello world!");
        let ast = PhpAst {
            statements: vec![PhpStatement::PhpExpressionStatement(
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                    PhpPrimaryExpression::StringLiteral(&value),
                )),
            )],
        };
        let output = ast.transpile();

        assert_eq!("<?php\n\"Hello world!\";", output);
    }
    */
}
