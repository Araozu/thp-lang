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
            Expression::BinaryOperator(exp1, exp2, operator) => {
                let t1 = exp1.get_type(scope)?;
                let t2 = exp2.get_type(scope)?;

                // TODO: There's definitely a better way to do this
                if *operator == "+" && t1 == "Int" && t2 == "Int" {
                    return Ok("Int".into());
                } else if *operator == "-" && t1 == "Int" && t2 == "Int" {
                    return Ok("Int".into());
                }

                return Err(MistiError::Semantic(SemanticError {
                    error_start: 0,
                    error_end: 1,
                    reason: format!(
                        "Unsupported binary operator or invalid arguments to the operator."
                    ),
                }));
            }
        }
    }
}
