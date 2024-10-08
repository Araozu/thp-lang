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

                // TODO: Should this check that the type signature is correct?
                // or is this done elsewhere?

                match &*f.function {
                    Expression::Identifier(id) => {
                        match scope.get_type(&id.value) {
                            Some(Type::Function(_, return_type)) => {
                                // Return the return type of the function,
                                // not the function itself
                                Ok(Type::Value(return_type))
                            }
                            Some(_) => Err(MistiError::Semantic(SemanticError {
                                error_start: id.position,
                                error_end: id.get_end_position(),
                                reason: format!("Expected `{}` to be a function", &id.value),
                            })),
                            None => Err(MistiError::Semantic(SemanticError {
                                error_start: id.position,
                                error_end: id.get_end_position(),
                                reason: format!("Cannot find `{}` in this scope.", id.value),
                            })),
                        }
                    }
                    _ => unimplemented!(
                        "Get datatype of an expression that resolves into a function call"
                    ),
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
                if op.value == "-" {
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
                } else if op.value == "!" {
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

                unreachable!("Illegal state: Found an unexpected unary operator during semantic analysis: {}", op.value);
            }
            Expression::BinaryOperator(exp1, exp2, operator) => {
                let t1 = exp1.get_type(scope)?;
                let t2 = exp2.get_type(scope)?;

                // TODO: There's definitely a better way to do this
                if operator.value == "+" && t1.is_value("Int") && t2.is_value("Int") {
                    return Ok(Type::Value("Int".into()));
                } else if operator.value == "-" && t1.is_value("Int") && t2.is_value("Int") {
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
    use crate::{
        error_handling::MistiError,
        lexic::token::Token,
        semantic::{
            std::populate,
            symbol_table::SymbolTable,
            types::{Type, Typed},
        },
        syntax::ast::{
            functions::{ArgumentsList, FunctionCall},
            Expression,
        },
    };

    #[test]
    fn should_get_global_print_type() {
        let mut scope = SymbolTable::new();
        populate(&mut scope);

        let identifier_token = Token::new_identifier("print".into(), 0);
        let expr = Expression::Identifier(&identifier_token);

        match expr.get_type(&scope) {
            Ok(Type::Function(params, return_type)) => {
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], "String");
                assert_eq!(return_type, "Void");
            }
            Ok(t) => panic!("Expected a Function, got {:?}", t),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }

    #[test]
    fn should_error_on_invalid_identifier() {
        let identifier_token = Token::new_identifier("print".into(), 0);
        let expr = Expression::Identifier(&identifier_token);
        let scope = SymbolTable::new();

        let expr_type = expr.get_type(&scope);
        match expr_type {
            Ok(_) => panic!("Expected an error"),
            Err(MistiError::Semantic(err)) => {
                assert_eq!(err.error_start, 0);
                assert_eq!(err.error_end, 5);
                assert_eq!(err.reason, "Cannot find `print` in this scope.");
            }
            Err(e) => panic!("Expected a semantic error, got {:?}", e),
        }
    }

    #[test]
    fn should_get_type_from_function_call() {
        let mut scope = SymbolTable::new();
        populate(&mut scope);

        let id_token = Token::new_identifier("print".into(), 0);
        let fn_expr = Expression::Identifier(&id_token);

        let args = ArgumentsList {
            arguments: vec![],
            paren_open_pos: 5,
            paren_close_pos: 7,
        };

        let fn_call = Expression::FunctionCall(FunctionCall {
            function: Box::new(fn_expr),
            arguments: Box::new(args),
        });

        match fn_call.get_type(&scope) {
            Ok(Type::Value(v)) => assert_eq!(v, "Void"),
            Ok(v) => panic!("Expected a value, got {:?}", v),
            Err(e) => panic!("Expected a value, got Err {:?}", e),
        }
    }

    #[test]
    fn should_fail_if_a_function_is_expected() {
        let scope = SymbolTable::new();
        // Add `print` as a Int
        scope.insert("print".into(), Type::Value("Int".into()));

        let id_token = Token::new_identifier("print".into(), 0);
        let fn_expr = Expression::Identifier(&id_token);

        let args = ArgumentsList {
            arguments: vec![],
            paren_open_pos: 5,
            paren_close_pos: 7,
        };

        let fn_call = Expression::FunctionCall(FunctionCall {
            function: Box::new(fn_expr),
            arguments: Box::new(args),
        });

        match fn_call.get_type(&scope) {
            Ok(v) => panic!("Expected an error, got {:?}", v),
            Err(MistiError::Semantic(e)) => {
                assert_eq!(e.error_start, 0);
                assert_eq!(e.error_end, 5);
                assert_eq!(e.reason, "Expected `print` to be a function");
            }
            Err(e) => panic!("Expected a semantic error, got {:?}", e),
        }
    }

    #[test]
    fn should_fail_if_a_function_is_not_defined() {
        let scope = SymbolTable::new();

        let id_token = Token::new_identifier("print".into(), 0);
        let fn_expr = Expression::Identifier(&id_token);

        let args = ArgumentsList {
            arguments: vec![],
            paren_open_pos: 5,
            paren_close_pos: 7,
        };

        let fn_call = Expression::FunctionCall(FunctionCall {
            function: Box::new(fn_expr),
            arguments: Box::new(args),
        });

        match fn_call.get_type(&scope) {
            Ok(v) => panic!("Expected an error, got {:?}", v),
            Err(MistiError::Semantic(e)) => {
                assert_eq!(e.error_start, 0);
                assert_eq!(e.error_end, 5);
                assert_eq!(e.reason, "Cannot find `print` in this scope.");
            }
            Err(e) => panic!("Expected a semantic error, got {:?}", e),
        }
    }
}
