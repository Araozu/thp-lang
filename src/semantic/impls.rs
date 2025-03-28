use crate::{error_handling::MistiError, syntax::ast::ModuleAST};

use super::symbol_table::SymbolTable;

/// Allows this type to have it's semantics checked.
pub trait SemanticCheck {
    /// Checks the semantics of this AST node and performs typechecking
    ///
    /// Types are provided by the Typed trait, because not every AST node
    /// will have a defined type
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError>;
}

impl SemanticCheck for ModuleAST<'_> {
    /// Checks that this AST is semantically correct, given a symbol table
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        for declaration in &self.productions {
            declaration.check_semantics(scope)?;
        }

        Ok(())
    }
}
