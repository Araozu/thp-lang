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
            Expression::Int(_) => Ok("Int".into()),
            Expression::Float(_) => Ok("Float".into()),
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
            Expression::FunctionCall(_f) => {
                // TODO: Must implement functions as first class citizens
                // for this to work

                // TODO: check the parameter types
                panic!("Not implemented: Get datatype of function call")
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
                    if expr_type != "Int" && expr_type != "Float" {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!(
                                "Expected a Int or Float after unary `-`, got {}",
                                expr_type
                            ),
                        }));
                    } else {
                        return Ok("Int".into());
                    }
                } else if *op == "!" {
                    if expr_type != "Bool" {
                        return Err(MistiError::Semantic(SemanticError {
                            error_start: 0,
                            error_end: 1,
                            reason: format!("Expected a Bool after unary `!`, got {}", expr_type),
                        }));
                    } else {
                        return Ok("Bool".into());
                    }
                }

                panic!("Illegal state: Found an unexpected unary operator during semantic analysis: {}", *op);
            }
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
