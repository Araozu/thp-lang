use super::Transpilable;
use crate::syntax::ast::var_binding::Binding;

impl Transpilable for Binding {
    /// Transpiles val and var bindings into PHP.
    fn transpile(&self) -> String {
        let expression_str = self.expression.transpile();

        format!("${} = {};", self.identifier, expression_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::{var_binding::Binding, Expression};

    #[test]
    fn binding_should_transpile() {
        let id = String::from("identifier");
        let value = String::from("322");
        let binding = Binding {
            datatype: None,
            identifier: Box::new(id),
            expression: Expression::Number(Box::new(value)),
            is_mutable: false,
        };

        let result = binding.transpile();

        assert_eq!("$identifier = 322;", result);
    }
}
