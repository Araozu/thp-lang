use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{Conditional, Positionable},
};

impl SemanticCheck for Conditional<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        let bool_type = Type::Value(String::from("Bool"));

        // Check if condition is a Bool
        let if_condition = &self.if_member.condition;
        let if_condition_type = if_condition.get_type(scope)?;
        if !if_condition_type.equals(&bool_type) {
            let (error_start, error_end) = if_condition.get_position();
            return Err(MistiError::Semantic(SemanticError {
                error_start,
                error_end,
                reason: format!(
                    "Expected a condition of type Bool, found {:?}",
                    if_condition_type
                ),
            }));
        }

        // Check if block
        let if_block = &self.if_member.body;
        if_block.check_semantics(scope)?;

        // Check all else if
        for else_if_member in self.else_if_members.iter() {
            let condition = &else_if_member.condition;
            let condition_type = condition.get_type(scope)?;
            if !condition_type.equals(&bool_type) {
                let (error_start, error_end) = condition.get_position();
                return Err(MistiError::Semantic(SemanticError {
                    error_start,
                    error_end,
                    reason: format!("Expected a condition of type Bool, found {:?}", condition_type),
                }));
            }

            else_if_member.body.check_semantics(scope)?;
        }

        // Check else
        if let Some(else_block) = &self.else_block {
            else_block.check_semantics(scope)?;
        }

        Ok(())
    }
}
