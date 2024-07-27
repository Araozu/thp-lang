use super::super::PhpStatement;
use crate::syntax::ast::Statement;

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Statement<'_> {
    type Item = PhpStatement<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        match self {
            _ => todo!("transformation for statement: {:?}", self),
        }
    }
}
