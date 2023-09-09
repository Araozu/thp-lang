use super::Transpilable;
use crate::syntax::ast::Binding;

impl Transpilable for Binding {
    /// Transpiles val and var bindings into PHP.
    fn transpile(&self) -> String {
        match self {
            Binding::Val(val_binding) => {
                let expression_str = val_binding.expression.transpile();

                format!("${} = {};", val_binding.identifier, expression_str)
            }
            Binding::Var(var_binding) => {
                let expression_str = var_binding.expression.transpile();

                format!("${} = {};", var_binding.identifier, expression_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::{Binding, Expression, ValBinding};

    #[test]
    fn binding_should_transpile() {
        let id = String::from("identifier");
        let value = String::from("322");
        let binding = Binding::Val(ValBinding {
            datatype: None,
            identifier: Box::new(id),
            expression: Expression::Number(Box::new(value)),
        });

        let result = binding.transpile();

        assert_eq!("$identifier = 322;", result);
    }
}
