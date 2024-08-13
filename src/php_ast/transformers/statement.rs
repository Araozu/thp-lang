use super::super::PhpStatement;
use crate::{
    php_ast::{PhpAssignmentExpression, PhpExpression, PhpSimpleAssignment},
    syntax::ast::Statement,
};

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Statement<'_> {
    type Item = PhpStatement<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        match self {
            Statement::Binding(b) => {
                // This is a PhpExpression, but a PhpPrimaryExpression is needed
                let binding_expr = b.expression.into_php_ast();

                // TODO: Somehow fix this...
                // the function above `into_php_ast` should somehow
                // return what I need? Or should return something general and
                // then i decide how to transform it here?
                // if it reaches this point in the pipeline, is it
                // safe to assume that any AST is correct, since
                // semantic analysis (supposedly) did its job?
                let binding_primary_expr = match binding_expr {
                    PhpExpression::Assignment(PhpAssignmentExpression::Primary(p)) => p,
                    _ => unreachable!("Expected a PrimaryExpression during AST transformation"),
                };

                PhpStatement::PhpExpressionStatement(PhpExpression::Assignment(
                    PhpAssignmentExpression::SimpleAssignment(PhpSimpleAssignment {
                        variable: b.identifier.value.clone(),
                        assignment: binding_primary_expr,
                    }),
                ))
            }
            _ => todo!("transformation for statement: {:?}", self),
        }
    }
}

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
