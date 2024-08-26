use crate::{
    php_ast::{PExpresssion, PPrimary},
    syntax::ast::Expression,
};

// TODO: next rewrite the test to use the output of Transpilable?

use super::PHPTransformable;

/// Transforms a THP expression into a PHP expression
impl<'a> PHPTransformable<'a> for Expression<'_> {
    type Item = PExpresssion<'a>;

    fn into_php_ast(&'a self) -> PExpresssion<'a> {
        match self {
            Expression::String(value) => {
                let expr = PPrimary::StringLiteral(&value.value);

                PExpresssion::Primary(expr)
            }
            Expression::Int(value) => {
                let expr = PPrimary::IntegerLiteral(&value.value);
                PExpresssion::Primary(expr)
            }
            Expression::Float(value) => {
                let expr = PPrimary::FloatingLiteral(&value.value);
                PExpresssion::Primary(expr)
            }
            Expression::FunctionCall(f) => {
                let fn_call_expr = f.into_php_ast();

                PExpresssion::FunctionCall(fn_call_expr)
            }
            Expression::Identifier(i) => PExpresssion::Primary(PPrimary::Variable(&i.value)),
            _ => todo!("transformation for expression: {:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexic::token::Token,
        php_ast::{transformers::PHPTransformable, PExpresssion, PPrimary},
        syntax::ast::Expression,
    };

    #[test]
    fn should_transform_string() {
        let t = Token::new_string("Hello".into(), 0);
        let input = Expression::String(&t);
        let output = input.into_php_ast();

        match output {
            PExpresssion::Primary(PPrimary::StringLiteral(value)) => {
                assert_eq!("Hello", value)
            }
            _ => panic!("Expected a String literal"),
        }
    }

    #[test]
    fn should_transform_int() {
        let binding = Token::new_int(String::from("322"), 0);
        let input = Expression::Int(&binding);
        let output = input.into_php_ast();

        match output {
            PExpresssion::Primary(PPrimary::IntegerLiteral(value)) => {
                assert_eq!("322", value)
            }
            _ => panic!("Expected a Int literal"),
        }
    }

    #[test]
    fn should_transform_float() {
        let t = Token::new_float("322.644".into(), 0);
        let input = Expression::Float(&t);
        let output = input.into_php_ast();

        match output {
            PExpresssion::Primary(PPrimary::FloatingLiteral(value)) => {
                assert_eq!("322.644", value)
            }
            _ => panic!("Expected a Float literal"),
        }
    }
}
