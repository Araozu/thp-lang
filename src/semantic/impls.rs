use crate::{
    error_handling::semantic_error::SemanticError,
    error_handling::MistiError,
    syntax::ast::{ModuleAST, TopLevelDeclaration},
};

use super::symbol_table::{SymbolEntry, SymbolTable};

pub trait SemanticCheck {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError>;
}

impl SemanticCheck for ModuleAST<'_> {
    /// Checks that this AST is semantically correct, given a symbol table
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        for declaration in &self.declarations {
            declaration.check_semantics(scope)?;
        }

        Ok(())
    }
}

impl SemanticCheck for TopLevelDeclaration<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        match self {
            TopLevelDeclaration::Binding(binding) => {
                let binding_name = &binding.identifier.value;

                if scope.test(binding_name) {
                    let error = SemanticError {
                        error_start: binding.identifier.position,
                        error_end: binding.identifier.get_end_position(),
                        reason: format!(
                            "Duplicated function: A function with name {} was already defined",
                            binding_name
                        ),
                    };

                    return Err(MistiError::Semantic(error));
                }

                let datatype = match binding.datatype {
                    Some(t) => t,
                    None => {
                        let error = SemanticError {
                            error_start: binding.identifier.position,
                            error_end: binding.identifier.get_end_position(),
                            reason: format!(
                                "The variable `{}` didn't define a datatype. Datatype inference is not implemented.",
                                binding_name
                            ),
                        };

                        return Err(MistiError::Semantic(error));
                    }
                };

                scope.insert(
                    binding_name.clone(),
                    SymbolEntry::new_variable(datatype.value.clone()),
                );

                Ok(())
            }
            TopLevelDeclaration::FunctionDeclaration(function) => {
                let function_name = function.identifier.value.clone();

                // Check that the function is not already defined
                if scope.test(&function_name) {
                    let error = SemanticError {
                        error_start: function.identifier.position,
                        error_end: function.identifier.get_end_position(),
                        reason: format!(
                            "Duplicated function: A function with name {} was already defined",
                            function_name
                        ),
                    };

                    return Err(MistiError::Semantic(error));
                }

                scope.insert(
                    function_name,
                    SymbolEntry::new_function(vec![], "Unit".into()),
                );

                Ok(())
            }
            _ => panic!("Not implemented"),
        }
    }
}
