use crate::syntax::ast::FunctionDeclaration;

use super::Transpilable;

impl Transpilable for FunctionDeclaration {
    fn transpile(&self) -> String {
        format!("function {}() {{}}", self.identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexic::get_tokens,
        syntax::{ast::TopLevelConstruct, construct_ast},
    };

    #[test]
    fn should_transpile() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();
        let result = construct_ast(&tokens).unwrap();

        let fun_dec = result.bindings.get(0).unwrap();

        match fun_dec {
            TopLevelConstruct::Binding(_) => panic!("Expected function declaration"),
            TopLevelConstruct::FunctionDeclaration(fun_decl) => {
                let transpiled = fun_decl.transpile();

                assert_eq!("function id() {}", transpiled);
            }
        }
    }
}
