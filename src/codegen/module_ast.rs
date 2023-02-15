use crate::ast_types::ModuleAST;
use super::Transpilable;

impl Transpilable for ModuleAST<'_> {
    /// Transpiles the whole AST into JS, using this same trait on the
    /// nodes and leaves of the AST
    fn transpile(&self) -> String {
        let bindings_str: Vec::<String> = self.bindings.iter().map(|binding| binding.transpile()).collect();

        bindings_str.join("\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_types::{Expression, ValBinding, Binding};

    #[test]
    fn module_ast_should_transpile() {
        let id = String::from("identifier");
        let value = String::from("322");
        let binding = Binding::Val(ValBinding {
            datatype: None,
            identifier: &id,
            expression: Expression::Number(&value),
        });

        let module = ModuleAST {
            bindings: vec![binding],
        };

        let result = module.transpile();

        assert_eq!("const identifier = 322;", result);
    }   
}
