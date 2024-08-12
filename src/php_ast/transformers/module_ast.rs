use super::super::PhpAst;
use crate::php_ast::{
    PhpAssignmentExpression, PhpExpression, PhpExpressionList, PhpPrimaryExpression, PhpStatement,
};
use crate::syntax::ast::{Expression, ModuleAST, ModuleMembers};

use super::PHPTransformable;

/// Transforms a THP AST into a PHP AST
impl<'a> PHPTransformable<'a> for ModuleAST<'_> {
    type Item = PhpAst<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        let mut php_statements = Vec::<PhpStatement>::new();

        for production in self.productions.iter() {
            match production {
                ModuleMembers::Stmt(stmt) => {
                    php_statements.push(stmt.into_php_ast());
                }
                ModuleMembers::Expr(expr) => {
                    // TODO: This should be done by the Expression transformer
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
                                                    PhpExpression::Assignment(PhpAssignmentExpression::Primary(PhpPrimaryExpression::StringLiteral(v)))
                                                )
                                            },
                                            _ => todo!("Non string expressions not supported")
                                        }
                                    }

                                    php_statements.push(PhpStatement::PhpEchoStatement(PhpExpressionList {
                                        expressions
                                    }));
                                },
                                _ => todo!("Not implemented: AST transformation for function call that is not an identifier")
                            }
                        }
                        Expression::Int(value) => {
                            php_statements.push(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::IntegerLiteral(value),
                                )),
                            ));
                        }
                        Expression::Float(value) => {
                            php_statements.push(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::FloatingLiteral(value),
                                )),
                            ));
                        }
                        Expression::String(value) => {
                            php_statements.push(PhpStatement::PhpExpressionStatement(
                                PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                                    PhpPrimaryExpression::StringLiteral(value),
                                )),
                            ));
                        }
                        _ => {
                            todo!("not implemented: AST transform for expression {:?}", expr)
                        }
                    }
                }
            }
        }

        PhpAst {
            statements: php_statements,
        }
    }
}

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
