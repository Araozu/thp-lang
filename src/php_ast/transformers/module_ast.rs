use super::super::PhpAst;
use crate::php_ast::{PhpExpressionList, PhpStatement};
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
                    // TODO: a print() function call is technically an
                    // expression in the AST, but PHP expects it to be an statement.
                    // transform beforehand?

                    match expr {
                        Expression::FunctionCall(fc) => {
                            let function_expr: &Expression = &*fc.function;
                            match function_expr {
                                Expression::Identifier(id) if *id == "print" => {
                                        // transform to print() expression
                                        // no parameters supported
                                        php_statements.push(PhpStatement::PhpEchoStatement(PhpExpressionList {
                                            expressions: vec![]
                                        }));
                                },
                                _ => todo!("Not implemented: AST transformation for function call that is not an identifier")
                            }
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
