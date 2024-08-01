use crate::{codegen::Transpilable, php_ast::PhpStatement};

mod echo_statement;

impl Transpilable for PhpStatement<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpStatement::PhpEchoStatement(expr_list) => {
                let expressions_vec = expr_list
                    .expressions
                    .iter()
                    .map(|e| e.transpile())
                    .collect::<Vec<_>>();

                let expressions_str = if expressions_vec.is_empty() {
                    "\"\"".into()
                } else {
                    expressions_vec.join(", ")
                };

                format!("echo {};", expressions_str)
            }
            PhpStatement::PhpExpressionStatement(expr) => {
                let expr_str = expr.transpile();
                format!("{};", expr_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::Transpilable,
        php_ast::{
            PhpAssignmentExpression, PhpExpression, PhpExpressionList, PhpPrimaryExpression,
            PhpStatement,
        },
    };

    #[test]
    fn should_gen_empty_echo_statement() {
        let expressions = PhpExpressionList {
            expressions: vec![],
        };
        let ast = PhpStatement::PhpEchoStatement(expressions);
        let output = ast.transpile();

        assert_eq!("echo \"\";", output)
    }

    #[test]
    fn should_gen_echo_with_expr() {
        let input = String::from("322");
        let exp_1 = PhpPrimaryExpression::FloatingLiteral(&input);
        let expressions = PhpExpressionList {
            expressions: vec![PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                exp_1,
            ))],
        };
        let ast = PhpStatement::PhpEchoStatement(expressions);
        let output = ast.transpile();

        assert_eq!("echo 322;", output)
    }

    #[test]
    fn should_gen_echo_with_multiple_expr() {
        let input = String::from("322");
        let exp_1 = PhpPrimaryExpression::FloatingLiteral(&input);

        let input = String::from("Hai world");
        let exp_2 = PhpPrimaryExpression::StringLiteral(&input);

        let expressions = PhpExpressionList {
            expressions: vec![
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(exp_1)),
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(exp_2)),
            ],
        };
        let ast = PhpStatement::PhpEchoStatement(expressions);
        let output = ast.transpile();

        assert_eq!("echo 322, \"Hai world\";", output)
    }

    #[test]
    fn should_gen_expression_stmt() {
        let input = String::from("Hi!");
        let exp_1 = PhpPrimaryExpression::StringLiteral(&input);
        let ast = PhpStatement::PhpExpressionStatement(PhpExpression::Assignment(
            PhpAssignmentExpression::Primary(exp_1),
        ));
        let output = ast.transpile();

        assert_eq!("\"Hi!\";", output)
    }
}
