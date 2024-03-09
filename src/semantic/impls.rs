use crate::{
    error_handling::semantic_error::SemanticError,
    error_handling::MistiError,
    syntax::ast::{ModuleAST, TopLevelDeclaration},
};

use super::symbol_table::SymbolTable;

pub trait SemanticCheck {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError>;
}

impl SemanticCheck for ModuleAST {
    /// Checks that this AST is semantically correct, given a symbol table
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        for declaration in &self.declarations {
            declaration.check_semantics(scope)?;
        }

        Ok(())
    }
}

impl SemanticCheck for TopLevelDeclaration {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        match self {
            TopLevelDeclaration::Binding(_) => {
                let error = SemanticError {
                    error_start: 0,
                    error_end: 0,
                    reason: "Binding typechecking: Not implemented".into(),
                };

                Err(MistiError::Semantic(error))
            }
            TopLevelDeclaration::FunctionDeclaration(function) => {
                let function_name = function.identifier.as_ref().clone();

                if scope.test(&function_name) {
                    let error = SemanticError {
                        // TODO: Get the position of the function name. For this, these structs
                        // should store the token instead of just the string
                        error_start: 0,
                        error_end: 0,
                        reason: format!("Function {} already defined", function_name),
                    };

                    return Err(MistiError::Semantic(error));
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
