use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::symbol_table::SymbolTable,
    syntax::ast::Expression,
};

use super::{Type, Typed};

impl Typed for Expression<'_> {
    /// Attempts to get the datatype for an expression.
    fn get_type(&self, scope: &SymbolTable) -> Result<Type, MistiError> {
        match self {
            Expression::Int(_) => Ok(Type::Value("Int".into())),
            Expression::Float(_) => Ok(Type::Value("Float".into())),
            Expression::String(_) => Ok(Type::Value("String".into())),
            Expression::Boolean(_) => Ok(Type::Value("Bool".into())),
            Expression::Identifier(identifier) => {
                // Attempt to get the datatype of the identifier in the current scope
                let datatype = match scope.get_type(&identifier.value) {
                    Some(x) => x,
                    None => {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: identifier.position,
                            error_end: identifier.get_end_position(),
                            reason: format!("Cannot find `{}` in this scope.", identifier.value),
                        }))
                    }
                };

                // TODO: use lifetimes
                Ok(datatype)
            }
            Expression::FunctionCall(f) => {
                // TODO: Must implement functions as first class citizens
                // for this to work with any arbitrary expression.
                // for now it justs expects an identifier

                match &*f.function {
                    Expression::Identifier(id) => {
                        match scope.get_type(&id.value) {
                            Some(t) => Ok(t),
                            None => Err(MistiError::Semantic(SemanticError {
                                // TODO: Actually find the start and end position
                                // this requires the token to be stored, rather than
                                // just the string value
                                error_start: 0,
                                error_end: 1,
                                reason: format!("Type not found for symbol {}", id.value),
                            })),
                        }
                    }
                    _ => todo!("Get datatype of an expression that resolves into a function call"),
                }
            }
            Expression::UnaryOperator(op, exp) => {
                let expr_type = match exp.get_type(scope) {
                    Ok(t) => t,
                    Err(_reason) => {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!("Error getting type of expression"),
                        }))
                    }
                };

                // Only supported unary operator: - & !
                if *op == "-" {
                    if !expr_type.is_value("Int") && !expr_type.is_value("Float") {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!(
                                "Expected a Int or Float after unary `-`, got {:?}",
                                expr_type
                            ),
                        }));
                    } else {
                        return Ok(Type::Value("Int".into()));
                    }
                } else if *op == "!" {
                    if !expr_type.is_value("Bool") {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!("Expected a Bool after unary `!`, got {:?}", expr_type),
                        }));
                    } else {
                        return Ok(Type::Value("Bool".into()));
                    }
                }

                unreachable!("Illegal state: Found an unexpected unary operator during semantic analysis: {}", *op);
            }
            Expression::BinaryOperator(exp1, exp2, operator) => {
                let t1 = exp1.get_type(scope)?;
                let t2 = exp2.get_type(scope)?;

                // TODO: There's definitely a better way to do this
                if *operator == "+" && t1.is_value("Int") && t2.is_value("Int") {
                    return Ok(Type::Value("Int".into()));
                } else if *operator == "-" && t1.is_value("Int") && t2.is_value("Int") {
                    return Ok(Type::Value("Int".into()));
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


#[cfg(test)]
mod tests {
    #[test]
    fn should_error() {

    }
}
