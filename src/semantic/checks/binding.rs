use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{impls::SemanticCheck, symbol_table::SymbolEntry, types::Typed},
    syntax::ast::var_binding::VariableBinding,
};

impl SemanticCheck for VariableBinding<'_> {
    fn check_semantics(
        &self,
        scope: &crate::semantic::symbol_table::SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        let binding_name = &self.identifier.value;

        // TODO: Define if variables can be redeclared.
        //       If so, it is irrelevant to check if the variable is already defined
        if scope.test(binding_name) {
            let error = SemanticError {
                error_start: self.identifier.position,
                error_end: self.identifier.get_end_position(),
                reason: format!(
                    "Duplicated: A symbol with name {} was already defined",
                    binding_name
                ),
            };

            return Err(MistiError::Semantic(error));
        }

        // This gets the datatype of the assigned expression,
        // to compare it later with the declared datatype.
        let expression_datatype = self.expression.get_type(scope)?;

        let datatype = match self.datatype {
            Some(t) => t.value.clone(),
            // If the datatype is not defined, we use the expression datatype
            None => expression_datatype.clone(),
        };

        // Both the declared & actual datatypes must be the same
        if datatype != expression_datatype {
            let error = SemanticError {
                error_start: self.identifier.position,
                error_end: self.identifier.get_end_position(),
                reason: format!(
                    "The variable `{}` was declared as `{}` but its expression has type `{}`",
                    binding_name, datatype, expression_datatype
                ),
            };

            return Err(MistiError::Semantic(error));
        }

        scope.insert(binding_name.clone(), SymbolEntry::new_variable(datatype));

        Ok(())
    }
}
