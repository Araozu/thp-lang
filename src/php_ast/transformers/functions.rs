use crate::{
    php_ast::php_ast_2::PFunctionCall,
    syntax::ast::{functions::FunctionCall, Expression},
};

use super::PHPTransformable;

impl<'a> PHPTransformable<'a> for FunctionCall<'a> {
    type Item = PFunctionCall<'a>;

    fn into_php_ast(&'a self) -> Self::Item {
        let function_expr = match *self.function {
            Expression::Identifier(i) => &i.value,
            _ => panic!("Cannot use an arbitrary expression as a function, only identifiers (for now)"),
        };

        let expressions: Vec<_> = self
            .arguments
            .arguments
            .iter()
            .map(|a| a.into_php_ast())
            .collect();

        PFunctionCall {
            function_name: function_expr,
            arguments: expressions,
        }
    }
}
