use super::Transpilable;
use crate::syntax::ast::var_binding::VariableBinding;

impl Transpilable for VariableBinding<'_> {
    /// Transpiles val and var bindings into PHP.
    fn transpile(&self) -> String {
        let expression_str = self.expression.transpile();

        format!("${} = {}", self.identifier.value, expression_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexic::token::{Token, TokenType},
        syntax::ast::{var_binding::VariableBinding, Expression},
    };

    #[test]
    fn binding_should_transpile() {
        let id = String::from("identifier");
        let id_token = Token {
            token_type: TokenType::Identifier,
            value: id,
            position: 0,
        };
        let value = String::from("322");
        let binding = VariableBinding {
            datatype: None,
            identifier: &id_token,
            expression: Expression::Int(&value),
            is_mutable: false,
        };

        let result = binding.transpile();

        assert_eq!("$identifier = 322", result);
    }
}
