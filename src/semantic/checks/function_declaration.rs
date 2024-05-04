use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{impls::SemanticCheck, symbol_table::SymbolEntry},
    syntax::ast::FunctionDeclaration,
};

impl SemanticCheck for FunctionDeclaration<'_> {
    fn check_semantics(
        &self,
        scope: &crate::semantic::symbol_table::SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        let function_name = self.identifier.value.clone();

        // Check that the function is not already defined
        if scope.test(&function_name) {
            let error = SemanticError {
                error_start: self.identifier.position,
                error_end: self.identifier.get_end_position(),
                reason: format!(
                    "Duplicated: A symbol with name {} was already defined",
                    function_name
                ),
            };

            return Err(MistiError::Semantic(error));
        }

        // TODO: Check the return type of the function
        // TODO: Check the return type of the function body

        scope.insert(
            function_name,
            SymbolEntry::new_function(vec![], "Unit".into()),
        );

        Ok(())
    }
}
