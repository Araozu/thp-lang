use super::Transpilable;
use crate::syntax::ast::ModuleAST;

impl Transpilable for ModuleAST {
    /// Transpiles the whole AST into PHP, using this same trait on the
    /// nodes and leaves of the AST
    fn transpile(&self) -> String {
        let bindings_str: Vec<String> = self
            .declarations
            .iter()
            .map(|binding| binding.transpile())
            .collect();

        bindings_str.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::{Binding, Expression, TopLevelDeclaration, ValBinding};

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
            declarations: vec![TopLevelDeclaration::Binding(binding)],
        };

        let result = module.transpile();

        assert_eq!("$identifier = 322;", result);
    }
}
