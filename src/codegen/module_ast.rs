use super::Transpilable;
use crate::syntax::ast::ModuleAST;

impl Transpilable for ModuleAST<'_> {
    /// Transpiles the whole AST into PHP, using this same trait on the
    /// nodes and leaves of the AST
    fn transpile(&self) -> String {
        let bindings_str: Vec<String> = self
            .productions
            .iter()
            .map(|binding| binding.transpile())
            .collect();

        format!("<?php\n\n{}\n", bindings_str.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexic::token::{Token, TokenType},
        syntax::ast::{var_binding::VariableBinding, Expression, ModuleMembers, Statement},
    };

    #[test]
    fn module_ast_should_transpile() {
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

        let module = ModuleAST {
            productions: vec![ModuleMembers::Stmt(Statement::Binding(binding))],
        };

        let result = module.transpile();

        assert_eq!("<?php\n\n$identifier = 322\n", result);
    }
}
