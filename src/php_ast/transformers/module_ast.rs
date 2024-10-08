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
                }
            }
        }

        PFile {
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
