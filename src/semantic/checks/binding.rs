use crate::{
    error_handling::{error_messages::SEMANTIC_DUPLICATED_REFERENCE, ErrorContainer, ErrorLabel},
    semantic::{
        impls::SemanticCheck,
        types::{Type, Typed},
    },
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
            let label = ErrorLabel {
                message: String::from("A reference with this name was already defined"),
                start: self.identifier.position,
                end: self.identifier.get_end_position(),
            };
            let econtainer = ErrorContainer {
                error_code: SEMANTIC_DUPLICATED_REFERENCE,
                error_offset: self.identifier.position,
                labels: vec![label],
                note: None,
                help: None,
            };
            return Err(econtainer);
        }

        self.expression.check_semantics(scope)?;

        // This gets the datatype of the assigned expression,
        // to compare it later with the declared datatype.
        let expression_datatype = self.expression.get_type(scope)?;

        let datatype = match self.datatype {
            Some(t) => Type::Value(t.value.clone()),
            // If the datatype is not defined, we use the expression datatype
            None => expression_datatype.clone(),
        };

        // Both the declared & actual datatypes must be the same
        if datatype != expression_datatype {
            let label = ErrorLabel {
                message: format!(
                    "The variable `{}` was declared as `{:?}` but its expression has type `{:?}`",
                    binding_name, datatype, expression_datatype
                ),
                start: self.identifier.position,
                end: self.identifier.get_end_position(),
            };
            let econtainer = ErrorContainer {
                error_code: SEMANTIC_DUPLICATED_REFERENCE,
                error_offset: self.identifier.position,
                labels: vec![label],
                note: None,
                help: None,
            };
            return Err(econtainer);
        }

        scope.insert_custom(binding_name.clone(), datatype, self.is_mutable);

        Ok(())
    }
}
