use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
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
            return Err(MistiError::Semantic(SemanticError {
                error_start,
                error_end,
                reason: format!(
                    "Expected a condition of type Bool, found {:?}",
                    condition_type
                ),
            }));
        }

        // TODO: Define scoping rules for while loops

        // Check inner block
        self.body.check_semantics(scope)?;

        Ok(())
    }
}
