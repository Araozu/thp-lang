use super::super::PhpStatement;
use crate::{
    codegen::Transpilable,
    php_ast::{
        php_ast_2::{PExpresssion, PSimpleAssignment, PStatement},
        PhpAssignmentExpression, PhpExpression, PhpSimpleAssignment,
    },
    syntax::ast::Statement,
};

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Statement<'_> {
    type Item = PStatement<'a>;

    fn into_php_ast(&'a self) -> PStatement<'a> {
        match self {
            Statement::Binding(b) => {
                let binding_expr = b.expression.into_php_ast();

                PStatement::ExpressionStatement(PExpresssion::Assignment(PSimpleAssignment {
                    variable: &b.identifier.value,
                    assignment: Box::new(binding_expr),
                }))
            }
            _ => todo!("transformation for statement: {:?}", self),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::{
        lexic::token::{Token, TokenType},
        php_ast::{
            transformers::PHPTransformable, PhpAssignmentExpression, PhpExpression,
            PhpPrimaryExpression, PhpStatement,
        },
        syntax::ast::{var_binding::VariableBinding, Expression, Statement},
    };

    #[test]
    fn should_transform_binding() {
        let identifier_token = Token {
            token_type: TokenType::Identifier,
            value: "name".into(),
            position: 0,
        };

        let t = Token::new_string("Hello".into(), 0);
        let expression = Expression::String(&t);
        let binding = Statement::Binding(VariableBinding {
            datatype: None,
            identifier: &identifier_token,
            expression,
            is_mutable: false,
        });
        let output = binding.into_php_ast();

        match output {
            PhpStatement::PhpExpressionStatement(PhpExpression::Assignment(
                PhpAssignmentExpression::SimpleAssignment(assignment),
            )) => {
                assert_eq!("name", assignment.variable);

                match assignment.assignment {
                    PhpPrimaryExpression::StringLiteral(value) => {
                        assert_eq!("Hello", value);
                    }
                    _ => panic!("Expected a String literal as the value of the assignment"),
                }
            }
            _ => panic!("Expected an ExpressionStatement"),
        }
    }
}
*/
