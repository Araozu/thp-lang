use crate::php_ast::{PFile, PStatement};
use crate::syntax::ast::{ModuleAST, ModuleMembers};

use super::PHPTransformable;

/// Transforms a THP AST into a PHP AST
impl<'a> PHPTransformable<'a> for ModuleAST<'_> {
    type Item = PFile<'a>;

    fn into_php_ast(&'a self) -> PFile<'a> {
        let mut php_statements = Vec::<PStatement>::new();

        for production in self.productions.iter() {
            match production {
                ModuleMembers::Stmt(stmt) => {
                    php_statements.push(stmt.into_php_ast());
                }
                ModuleMembers::Expr(expr) => {
                    let p_expression = expr.into_php_ast();

                    php_statements.push(PStatement::ExpressionStatement(p_expression));

                    /*
                    match expr {
                        Expression::FunctionCall(fc) => {



                            // TODO: This definitely needs refactoring
                            let function_expr: &Expression = &*fc.function;
                            match function_expr {
                                Expression::Identifier(id) if id.value == "print" => {
                                    // transform to print() expression
                                    // no parameters supported

                                    // transform parameters, expect them all to be strings

                                    let mut expressions = Vec::<PhpExpression>::new();

                                    for e in fc.arguments.arguments.iter() {
                                        match e {
                                            Expression::String(v) => {
                                                expressions.push(
                                                    PhpExpression::Assignment(PhpAssignmentExpression::Primary(PhpPrimaryExpression::StringLiteral(&v.value)))
                                                )
                                            },
                                            _ => todo!("Non string expressions not supported")
                                        }
                                    }

                                    php_statements.push(Box::new(PhpStatement::PhpEchoStatement(PhpExpressionList {
                                        expressions
                                    })));
                                },
                                _ => todo!("Not implemented: AST transformation for function call that is not an identifier")
                            }
                        }
                        Expression::Int(value) => {
                            php_statements.push(Box::new(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::IntegerLiteral(&value.value),
                                )),
                            )));
                        }
                        Expression::Float(value) => {
                            php_statements.push(Box::new(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::FloatingLiteral(&value.value),
                                )),
                            )));
                        }
                        Expression::String(value) => {
                            php_statements.push(Box::new(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::StringLiteral(&value.value),
                                )),
                            )));
                        }
                        _ => {
                            todo!("not implemented: AST transform for expression {:?}", expr)
                        }
                    }
                    */
                }
            }
        }

        PFile {
            statements: php_statements,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::{php_ast::transformers::PHPTransformable, syntax::ast::ModuleAST};

    #[test]
    fn should_transform_empty_ast() {
        let input = ModuleAST {
            productions: vec![],
        };
        let output = input.into_php_ast();

        assert!(output.statements.is_empty())
    }
}
*/
