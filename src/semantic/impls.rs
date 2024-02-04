use crate::syntax::ast::{ModuleAST, TopLevelDeclaration};

use super::symbol_table::SymbolTable;

pub trait SemanticCheck {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), String>;
}

impl SemanticCheck for ModuleAST {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), String> {
        for declaration in &self.declarations {
            declaration.check_semantics(scope)?;
        }

        Ok(())
    }
}

impl SemanticCheck for TopLevelDeclaration {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), String> {
        match self {
            TopLevelDeclaration::Binding(_) => Err("Binding not implemented".into()),
            TopLevelDeclaration::FunctionDeclaration(function) => {
                let function_name = function.identifier.as_ref().clone();

                if scope.test(&function_name) {
                    return Err(format!("Function {} already defined", function_name));
                }

                scope.insert(
                    function_name,
                    super::symbol_table::SymbolEntry::Function(vec![], "Unit".into()),
                );

                Ok(())
            }
        }
    }
}
