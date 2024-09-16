use crate::{
    error_handling::{
        error_messages::{
            COMPILER_TODO, SEMANTIC_INVALID_REFERENCE, SEMANTIC_MISMATCHED_TYPES,
            SEMANTIC_MISSING_REFERENCE,
        },
        ErrorContainer, ErrorLabel, MistiError,
    },
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
                        let label = ErrorLabel {
                            message: String::from("Cannot find this identifier in this scope"),
                            start: identifier.position,
                            end: identifier.get_end_position(),
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISSING_REFERENCE,
                            error_offset: identifier.position,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(MistiError::Semantic(econtainer));
                    }
                };

                // TODO: use lifetimes
                Ok(datatype)
            }
            Expression::FunctionCall(f) => {
                // TODO: allow arbitrary expressions and
                // check that they resolve into a function
                // (e.g. object.member)

                match &*f.function {
                    Expression::Identifier(id) => {
                        match scope.get_type(&id.value) {
                            Some(Type::Function(_, return_type)) => {
                                // Return the return type of the function,
                                // not the function itself
                                Ok(Type::Value(return_type))
                            }
                            Some(_) => {
                                let label = ErrorLabel {
                                    message: String::from(
                                        "Expected this identifier to be a function",
                                    ),
                                    start: id.position,
                                    end: id.get_end_position(),
                                };
                                let econtainer = ErrorContainer {
                                    error_code: SEMANTIC_INVALID_REFERENCE,
                                    error_offset: id.position,
                                    labels: vec![label],
                                    note: None,
                                    help: None,
                                };
                                return Err(MistiError::Semantic(econtainer));
                            }
                            None => {
                                let label = ErrorLabel {
                                    message: String::from(
                                        "Cannot find this identifier in this scope",
                                    ),
                                    start: id.position,
                                    end: id.get_end_position(),
                                };
                                let econtainer = ErrorContainer {
                                    error_code: SEMANTIC_INVALID_REFERENCE,
                                    error_offset: id.position,
                                    labels: vec![label],
                                    note: None,
                                    help: None,
                                };
                                return Err(MistiError::Semantic(econtainer));
                            }
                        }
                    }
                    _ => unimplemented!(
                        "Get datatype of an expression that may resolve into a function call"
                    ),
                }
            }
            Expression::UnaryOperator(op, exp) => {
                let expr_type = match exp.get_type(scope) {
                    Ok(t) => t,
                    Err(_reason) => {
                        let label = ErrorLabel {
                            message: String::from("Error getting type of this expression"),
                            // TODO: Fix these positions
                            start: 0,
                            end: 1,
                        };
                        let econtainer = ErrorContainer {
                            error_code: COMPILER_TODO,
                            error_offset: 0,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(MistiError::Semantic(econtainer));
                    }
                };

                // Only supported unary operator: - & !
                if op.value == "-" {
                    if !expr_type.is_value("Int") && !expr_type.is_value("Float") {
                        let label = ErrorLabel {
                            message: format!("Expected an `Int` or `Float`, got {:?}", expr_type),
                            // TODO: Fix positioning
                            start: 0,
                            end: 1,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISMATCHED_TYPES,
                            error_offset: 0,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(MistiError::Semantic(econtainer));
                    } else {
                        return Ok(Type::Value("Int".into()));
                    }
                } else if op.value == "!" {
                    if !expr_type.is_value("Bool") {
                        let label = ErrorLabel {
                            message: format!("Expected a `Bool`, got {:?}", expr_type),
                            // TODO: Fix positioning
                            start: 0,
                            end: 1,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISMATCHED_TYPES,
                            error_offset: 0,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(MistiError::Semantic(econtainer));
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
                // maybe store operators as functions?
                if operator.value == "+" && t1.is_value("Int") && t2.is_value("Int") {
                    return Ok(Type::Value("Int".into()));
                } else if operator.value == "-" && t1.is_value("Int") && t2.is_value("Int") {
                    return Ok(Type::Value("Int".into()));
                }

                let label = ErrorLabel {
                    message: format!("Unsupported binary operator"),
                    // TODO: Fix positioning
                    start: 0,
                    end: 1,
                };
                let econtainer = ErrorContainer {
                    error_code: SEMANTIC_MISMATCHED_TYPES,
                    error_offset: 0,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(MistiError::Semantic(econtainer));
            }
            Expression::Array(arr) => {
                // The first expression found determines the
                // type of the array

                // TODO: for now an array must have at least 1 element,
                // if the array is empty there is no way to know its type.
                // TODO: if the array is empty then its
                // datatype should be determined by its usage.
                if arr.exps.is_empty() {
                    let label = ErrorLabel {
                    message: format!("Compiler limit: Arrays must have at least 1 element to determine their type"),
                    // TODO: Fix positioning
                    start: arr.start,
                    end: arr.end,
                };
                    let econtainer = ErrorContainer {
                        error_code: SEMANTIC_MISMATCHED_TYPES,
                        error_offset: 0,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(MistiError::Semantic(econtainer));
                }

                // Just get the first type and use it
                // Checking of the types of every element in the array
                // is done by SemanticCheck
                let first_type = arr.exps[0].get_type(scope)?;
                Ok(Type::Generic("Array".into(), vec![first_type]))
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
