use crate::{
    error_handling::{
        error_messages::SEMANTIC_MISMATCHED_TYPES, ErrorContainer, ErrorLabel, MistiError,
    },
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{loops::WhileLoop, Positionable},
};

impl SemanticCheck for WhileLoop<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        // Check condition is a bool
        let condition = &self.condition;
        let condition_type = condition.get_type(scope)?;

        if !condition_type.equals(&Type::Value("Bool".into())) {
            let (error_start, error_end) = condition.get_position();

            let label = ErrorLabel {
                message: format!(
                    "Expected a condition of type Bool, found {:?}",
                    condition_type
                ),
                start: error_start,
                end: error_end,
            };
            let econtainer = ErrorContainer {
                error_code: SEMANTIC_MISMATCHED_TYPES,
                error_offset: error_start,
                labels: vec![label],
                note: Some(String::from("THP does not have truthy/falsey values.")),
                help: None,
            };
            return Err(econtainer);
        }

        // TODO: Define scoping rules for while loops

        // Check inner block
        self.body.check_semantics(scope)?;

        Ok(())
    }
}
