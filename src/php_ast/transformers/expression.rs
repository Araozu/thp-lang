use super::super::PhpExpression;
use crate::{php_ast::PhpPrimaryExpression, syntax::ast::Expression};

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Expression<'_> {
    type Item = PhpExpression<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        match self {
            Expression::String(value) => {
                let expr = PhpPrimaryExpression::StringLiteral(value);
                PhpExpression::PrimaryExpression(expr)
            },
            _ => todo!("transformation for expression: {:?}", self),
        }
    }
}
