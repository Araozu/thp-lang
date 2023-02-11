use crate::ast_types::Binding;
use super::Transpilable;

impl Transpilable for Binding<'_> {
    /// Transpiles val and var bindings into JS.
    fn transpile(&self) -> String {
        match self {
            Binding::Val(val_binding) => {
                let expression_str = val_binding.expression.transpile();

                format!("const {} = {};", val_binding.identifier, expression_str)
            }
            Binding::Var(var_binding) => {
                let expression_str = var_binding.expression.transpile();

                format!("let {} = {};", var_binding.identifier, expression_str)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_types::{Expression, Binding, ValBinding};

    #[test]
    fn binding_should_transpile() {
        let id = String::from("identifier");
        let value = String::from("322");
        let binding = Binding::Val(ValBinding {
            identifier: &id,
            expression: Expression::Number(&value),
        });

        let result = binding.transpile();
        
        assert_eq!("const identifier = 322;", result);
    }
}
