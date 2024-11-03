use crate::{
    error_handling::{error_messages::SEMANTIC_DUPLICATED_REFERENCE, ErrorContainer, ErrorLabel},
    semantic::{
        impls::SemanticCheck,
        types::{Type, Typed},
    },
    syntax::ast::{var_binding::VariableBinding, Positionable},
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
            // This can only happen if the binding has an annotated type,
            // so its safe to unwrap here
            let datatype_token = self.datatype.unwrap();

            let label1 = ErrorLabel {
                message: format!("The variable is declared as {:?} here", datatype),
                start: datatype_token.position,
                end: datatype_token.get_end_position(),
            };
            let (expr_start, expr_end) = self.expression.get_position();
            let label2 = ErrorLabel {
                message: format!("But this expression has type {:?}", expression_datatype),
                start: expr_start,
                end: expr_end,
            };

            let econtainer = ErrorContainer {
                error_code: SEMANTIC_DUPLICATED_REFERENCE,
                error_offset: self.identifier.position,
                labels: vec![label1, label2],
                note: None,
                help: None,
            };
            return Err(econtainer);
        }

        scope.insert_custom(binding_name.clone(), datatype, self.is_mutable);

        Ok(())
    }
}
