use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{Expression, Positionable},
};

impl SemanticCheck for Expression<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        match self {
            Expression::FunctionCall(f) => {
                let fun = &*f.function;
                let arguments = &*f.arguments.arguments;

                let function_datatype = fun.get_type(scope)?;
                match function_datatype {
                    Type::Function(parameters, _return_type) => {
                        // Check parameters length
                        if parameters.len() != arguments.len() {
                            let (error_start, error_end) = f.arguments.get_position();

                            return Err(MistiError::Semantic(SemanticError {
                                error_start,
                                error_end,
                                reason: format!(
                                    "Expected {} arguments, got {}",
                                    parameters.len(),
                                    arguments.len(),
                                ),
                            }));
                        }

                        // Check that each argument matches the required datatype
                        for i in 0..parameters.len() {
                            let parameter = &parameters[i];
                            let argument = &arguments[i];

                            let argument_datatype = argument.get_type(scope)?;
                            if !argument_datatype.is_value(parameter) {
                                // The argument and the parameter have diferent types
                                let (error_start, error_end) = argument.get_position();
                                return Err(MistiError::Semantic(SemanticError {
                                    error_start,
                                    error_end,
                                    reason: format!(
                                        "Expected a {}, got {:?}",
                                        parameter, argument_datatype
                                    ),
                                }));
                            }
                        }

                        Ok(())
                    }
                    _ => {
                        let (error_start, error_end) = fun.get_position();
                        return Err(MistiError::Semantic(SemanticError {
                            error_start,
                            error_end,
                            reason: format!(
                                "Expected a function type, got {:?}",
                                function_datatype
                            ),
                        }));
                    }
                }
            }
            // These are empty because they have nothing to check,
            // their existance alone is correct
            Expression::Int(_) => Ok(()),
            Expression::Float(_) => Ok(()),
            Expression::String(_) => Ok(()),
            Expression::Boolean(_) => Ok(()),
            Expression::Identifier(_) => Ok(()),
            Expression::UnaryOperator(operator, expression) => {
                // There are a limited amount of unary operators,
                // so their checking is not generalized
                let expr_type = expression.get_type(scope)?;
                match (operator.value.as_str(), expr_type) {
                    ("!", Type::Value(t)) => {
                        if t == "Bool" {
                            // Ok, empty
                            return Ok(());
                        } else {
                            // Error: unary negation can only be applied to a Bool
                            let (error_start, error_end) = expression.get_position();
                            return Err(MistiError::Semantic(SemanticError {
                                error_start,
                                error_end,
                                reason: format!("Expected a Bool, got a {}", t),
                            }));
                        }
                    }
                    ("!", Type::Function(_, _)) => {
                        // Error: unary negation can only be applied to a Bool
                        let (error_start, error_end) = expression.get_position();
                        return Err(MistiError::Semantic(SemanticError {
                            error_start,
                            error_end,
                            reason: format!("Expected a Bool, got a function",),
                        }));
                    }
                    ("-", Type::Value(t)) => {
                        if t == "Int" || t == "Float" {
                            // Ok, empty
                            return Ok(());
                        } else {
                            // Error: unary negation can only be applied to a Number
                            let (error_start, error_end) = expression.get_position();
                            return Err(MistiError::Semantic(SemanticError {
                                error_start,
                                error_end,
                                reason: format!("Expected a Float or Int, got a {}", t),
                            }));
                        }
                    }
                    ("-", Type::Function(_, _)) => {
                        // Error: unary negation can only be applied to a Bool
                        let (error_start, error_end) = expression.get_position();
                        return Err(MistiError::Semantic(SemanticError {
                            error_start,
                            error_end,
                            reason: format!("Expected a Float or Int, got a function",),
                        }));
                    }
                    (op, _) => {
                        // Compiler error: something that shouldn't be
                        // parsed as a unary operator was found.
                        unreachable!("Found a unary operator that shouldn't be: {}", op)
                    }
                }
            }
            Expression::BinaryOperator(left_expr, right_expr, op) => {
                // Operators are treated as functions
                let (op_params, _) = match scope.get_type(&op.value) {
                    Some(Type::Function(params, return_t)) => (params, return_t),
                    Some(Type::Value(v)) => {
                        // If a operator is stored as a value,
                        // it's a bug in the compiler
                        unreachable!("Compiler bug: a binary operator was registered in the symbol table as a value of type {}", v)
                    }
                    None => {
                        // If the operator is not found its a user error,
                        // because we allow arbitrary operators
                        let (error_start, error_end) = (op.position, op.get_end_position());
                        return Err(MistiError::Semantic(SemanticError {
                            error_start,
                            error_end,
                            reason: format!("The binary operator {} does not exist", op.value),
                        }));
                    }
                };

                if op_params.len() != 2 {
                    // If an operator has any other number
                    // of parameters, it's a bug in the compiler
                    unreachable!(
                        "Compiler bug: a binary operator didn't have 2 parameters: {:?}",
                        op_params
                    )
                }

                let left_expr_type = left_expr.get_type(scope)?;
                let right_expr_type = right_expr.get_type(scope)?;

                if !left_expr_type.is_value(&op_params[0]) {
                    let (error_start, error_end) = left_expr.get_position();
                    return Err(MistiError::Semantic(SemanticError {
                        error_start,
                        error_end,
                        reason: format!(
                            "Expected a {}, got a {:?} on the left side of the {} operator",
                            op_params[0], left_expr_type, op.value
                        ),
                    }));
                }

                if !right_expr_type.is_value(&op_params[1]) {
                    let (error_start, error_end) = right_expr.get_position();
                    return Err(MistiError::Semantic(SemanticError {
                        error_start,
                        error_end,
                        reason: format!(
                            "Expected a {}, got a {:?} on the right side of the {} operator",
                            op_params[1], left_expr_type, op.value
                        ),
                    }));
                }

                // After all these checks, we are ok
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error_handling::MistiError,
        lexic::token::Token,
        semantic::{impls::SemanticCheck, std::populate, symbol_table::SymbolTable},
        syntax::ast::{
            functions::{ArgumentsList, FunctionCall},
            Expression,
        },
    };

    #[test]
    fn should_error_on_undefined_symbol() {
        // source code: `print()`
        let expr_token = Token::new_identifier("print".into(), 0);
        let expr_function = Expression::Identifier(&expr_token);
        let arguments = ArgumentsList {
            arguments: vec![],
            paren_open_pos: 5,
            paren_close_pos: 7,
        };

        let expr = Expression::FunctionCall(FunctionCall {
            function: Box::new(expr_function),
            arguments: Box::new(arguments),
        });

        let scope = SymbolTable::new();

        let output = expr.check_semantics(&scope);
        match output {
            Ok(_) => panic!("Expected an error"),
            Err(MistiError::Semantic(err)) => {
                assert_eq!(err.reason, "Cannot find `print` in this scope.");
                assert_eq!(err.error_start, 0);
                assert_eq!(err.error_end, 5);
            }
            Err(e) => panic!("Expected a Semantic error, got {:?}", e),
        }
    }

    #[test]
    fn should_error_on_invalid_function_argument() {
        // source code: `print(322)`
        let mut scope = SymbolTable::new();
        populate(&mut scope);

        let expr_token = Token::new_identifier("print".into(), 0);
        let expr_function = Expression::Identifier(&expr_token);

        let arg_t = Token::new_int(String::from("322"), 6);
        let arg_1 = Expression::Int(&arg_t);
        let arguments = ArgumentsList {
            arguments: vec![arg_1],
            paren_open_pos: 5,
            paren_close_pos: 10,
        };

        let expr = Expression::FunctionCall(FunctionCall {
            function: Box::new(expr_function),
            arguments: Box::new(arguments),
        });

        match expr.check_semantics(&scope) {
            Ok(_) => panic!("Expected semantic error, got ok"),
            Err(MistiError::Semantic(e)) => {
                assert_eq!(e.reason, "Expected a String, got Value(\"Int\")");
                assert_eq!(e.error_start, 6);
                assert_eq!(e.error_end, 9);
            }
            Err(e) => panic!("Expected semantic error, got {:?}", e),
        }
    }

    #[test]
    fn should_error_on_invalid_function_argument_count() {
        // source code: `print()`
        let mut scope = SymbolTable::new();
        populate(&mut scope);

        let expr_token = Token::new_identifier("print".into(), 0);
        let expr_function = Expression::Identifier(&expr_token);

        let arguments = ArgumentsList {
            arguments: vec![],
            paren_open_pos: 5,
            paren_close_pos: 7,
        };

        let expr = Expression::FunctionCall(FunctionCall {
            function: Box::new(expr_function),
            arguments: Box::new(arguments),
        });

        match expr.check_semantics(&scope) {
            Ok(_) => panic!("Expected semantic error, got ok"),
            Err(MistiError::Semantic(e)) => {
                assert_eq!(e.reason, "Expected 1 arguments, got 0");
                assert_eq!(e.error_start, 5);
                assert_eq!(e.error_end, 7);
            }
            Err(e) => panic!("Expected semantic error, got {:?}", e),
        }
    }

    #[test]
    fn should_error_on_invalid_function_argument_2() {
        // source code: `print(322, 644)`
        let mut scope = SymbolTable::new();
        populate(&mut scope);

        let expr_token = Token::new_identifier("print".into(), 0);
        let expr_function = Expression::Identifier(&expr_token);

        let arg_t = Token::new_int(String::from("322"), 6);
        let arg_1 = Expression::Int(&arg_t);

        let arg_t_2 = Token::new_int(String::from("644"), 11);
        let arg_2 = Expression::Int(&arg_t_2);

        let arguments = ArgumentsList {
            arguments: vec![arg_1, arg_2],
            paren_open_pos: 5,
            paren_close_pos: 15,
        };

        let expr = Expression::FunctionCall(FunctionCall {
            function: Box::new(expr_function),
            arguments: Box::new(arguments),
        });

        match expr.check_semantics(&scope) {
            Ok(_) => panic!("Expected semantic error, got ok"),
            Err(MistiError::Semantic(e)) => {
                assert_eq!(e.reason, "Expected 1 arguments, got 2");
                assert_eq!(e.error_start, 5);
                assert_eq!(e.error_end, 15);
            }
            Err(e) => panic!("Expected semantic error, got {:?}", e),
        }
    }
}
