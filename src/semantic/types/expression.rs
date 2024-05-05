use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::symbol_table::SymbolTable,
    syntax::ast::Expression,
};

use super::Typed;

impl Typed for Expression<'_> {
    /// Attempts to get the datatype for an expression.
    fn get_type(&self, scope: &SymbolTable) -> Result<String, MistiError> {
        match self {
            // TODO: Distinguish between Int & Float
            Expression::Number(_) => Ok("Int".into()),
            Expression::String(_) => Ok("String".into()),
            Expression::Boolean(_) => Ok("Bool".into()),
            Expression::Identifier(identifier) => {
                // Attempt to get the datatype of the identifier in the current scope
                let datatype = match scope.get_type(identifier) {
                    Some(x) => x,
                    None => {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!("The identifier {} does not exist.", identifier),
                        }))
                    }
                };
                
                Ok(datatype)
            }
            Expression::FunctionCall(_) => todo!(),
            Expression::UnaryOperator(_, _) => todo!(),
            Expression::BinaryOperator(_, _, _) => todo!(),
        }
    }
}
