use super::super::PhpExpression;
use crate::{
    php_ast::{PhpAssignmentExpression, PhpPrimaryExpression},
    syntax::ast::Expression,
};

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Expression<'_> {
    type Item = PhpExpression<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        match self {
            Expression::String(value) => {
                let expr = PhpPrimaryExpression::StringLiteral(value);
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(expr))
            }
            Expression::Int(value) => {
                let expr = PhpPrimaryExpression::IntegerLiteral(value);
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(expr))
            }
            Expression::Float(value) => {
                let expr = PhpPrimaryExpression::FloatingLiteral(value);
                PhpExpression::Assignment(PhpAssignmentExpression::Primary(expr))
            }
            _ => todo!("transformation for expression: {:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        php_ast::{
            transformers::PHPTransformable, PhpAssignmentExpression, PhpExpression,
            PhpPrimaryExpression,
        },
        syntax::ast::Expression,
    };

    #[test]
    fn should_transform_string() {
        let value = String::from("Hello");
        let input = Expression::String(&value);
        let output = input.into_php_ast();

        match output {
            PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                PhpPrimaryExpression::StringLiteral(value),
            )) => {
                assert_eq!("Hello", value)
            }
            _ => panic!("Expected a String literal"),
        }
    }

    #[test]
    fn should_transform_int() {
        let value = String::from("322");
        let input = Expression::Int(&value);
        let output = input.into_php_ast();

        match output {
            PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                PhpPrimaryExpression::IntegerLiteral(value),
            )) => {
                assert_eq!("322", value)
            }
            _ => panic!("Expected a Int literal"),
        }
    }

    #[test]
    fn should_transform_float() {
        let value = String::from("322.644");
        let input = Expression::Float(&value);
        let output = input.into_php_ast();

        match output {
            PhpExpression::Assignment(PhpAssignmentExpression::Primary(
                PhpPrimaryExpression::FloatingLiteral(value),
            )) => {
                assert_eq!("322.644", value)
            }
            _ => panic!("Expected a Float literal"),
        }
    }
}
