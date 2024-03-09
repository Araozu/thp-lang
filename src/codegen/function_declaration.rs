use crate::syntax::ast::FunctionDeclaration;

use super::Transpilable;

impl Transpilable for FunctionDeclaration<'_> {
    fn transpile(&self) -> String {
        format!(
            "function {}() {{\n{}\n}}",
            self.identifier.value,
            self.block.transpile()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexic::get_tokens,
        syntax::{ast::TopLevelDeclaration, construct_ast},
    };

    #[test]
    fn should_transpile() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();
        let result = construct_ast(&tokens).unwrap();

        let fun_dec = result.declarations.get(0).unwrap();

        match fun_dec {
            TopLevelDeclaration::Binding(_) => panic!("Expected function declaration"),
            TopLevelDeclaration::FunctionDeclaration(fun_decl) => {
                let transpiled = fun_decl.transpile();

                assert_eq!("function id() {\n\n}", transpiled);
            }
        }
    }
}
