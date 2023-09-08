use super::Transpilable;
use crate::syntax::ast::ModuleAST;

impl Transpilable for ModuleAST {
    /// Transpiles the whole AST into JS, using this same trait on the
    /// nodes and leaves of the AST
    fn transpile(&self) -> String {
        let bindings_str: Vec<String> = self
            .bindings
            .iter()
            .map(|binding| binding.transpile())
            .collect();

        bindings_str.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::{Binding, Expression, ValBinding};

    #[test]
    fn module_ast_should_transpile() {
        let id = String::from("identifier");
        let value = String::from("322");
        let binding = Binding::Val(ValBinding {
            datatype: None,
            identifier: Box::new(id),
            expression: Expression::Number(Box::new(value)),
        });

        let module = ModuleAST {
            bindings: vec![binding],
        };

        let result = module.transpile();

        assert_eq!("const identifier = 322;", result);
    }
}
