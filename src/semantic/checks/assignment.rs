use crate::{
    error_handling::{
        self,
        error_messages::{SEMANTIC_IMMUTABLE_VARIABLE, SEMANTIC_INVALID_REFERENCE},
        ErrorContainer, ErrorLabel,
    },
    semantic::{self, impls::SemanticCheck, types::Typed},
    syntax::ast::{Assignment, Positionable},
};

impl SemanticCheck for Assignment<'_> {
    fn check_semantics(
        &self,
        scope: &semantic::symbol_table::SymbolTable,
    ) -> Result<(), error_handling::MistiError> {
        // for now the assignment can only be to a variable

        // get the datatype and mutability status
        let datatype = match scope.get_type_and_mut(&self.identifier.value) {
            Some((datatype, true)) => datatype,
            Some((_, false)) => {
                // throw error: variable is immutable
                let label = ErrorLabel {
                    message: String::from(
                        "This variable is immutable, therefore it cannot be assigned a new value",
                    ),
                    start: self.identifier.position,
                    end: self.identifier.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SEMANTIC_IMMUTABLE_VARIABLE,
                    error_offset: self.identifier.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(econtainer);
            }
            None => {
                // throw error: variable does not exist
                let label = ErrorLabel {
                    message: String::from("This variable does not exist in this scope"),
                    start: self.identifier.position,
                    end: self.identifier.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SEMANTIC_INVALID_REFERENCE,
                    error_offset: self.identifier.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(econtainer);
            }
        };

        // assert the datatype is the same
        let expression_type = self.expression.get_type(scope)?;

        if !datatype.equals(&expression_type) {
            // throw error: variable and expression have different types
            let label = ErrorLabel {
                message: format!("This variable has type {:?}", datatype),
                start: self.identifier.position,
                end: self.identifier.get_end_position(),
            };
            let (expr_start, expr_end) = self.expression.get_position();
            let label2 = ErrorLabel {
                message: format!("But this expression has type {:?}", expression_type),
                start: expr_start,
                end: expr_end,
            };
            let econtainer = ErrorContainer {
                error_code: SEMANTIC_INVALID_REFERENCE,
                error_offset: self.identifier.position,
                labels: vec![label, label2],
                note: None,
                help: None,
            };
            return Err(econtainer);
        }

        // ok
        Ok(())
    }
}
