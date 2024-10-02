use crate::{
    error_handling::{
        error_messages::{
            COMPILER_TODO, SEMANTIC_INVALID_REFERENCE, SEMANTIC_MISMATCHED_ARGUMENT_COUNT,
            SEMANTIC_MISMATCHED_TYPES,
        },
        ErrorContainer, ErrorLabel, MistiError,
    },
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

                            let label = ErrorLabel {
                                message: format!(
                                    "Expected {} arguments, got {}",
                                    parameters.len(),
                                    arguments.len(),
                                ),
                                start: error_start,
                                end: error_end,
                            };
                            let econtainer = ErrorContainer {
                                error_code: SEMANTIC_MISMATCHED_ARGUMENT_COUNT,
                                error_offset: error_start,
                                labels: vec![label],
                                note: None,
                                help: None,
                            };
                            return Err(econtainer);
                        }

                        // Check that each argument matches the required datatype
                        for i in 0..parameters.len() {
                            let parameter = &parameters[i];
                            let argument = &arguments[i];

                            let argument_datatype = argument.get_type(scope)?;
                            if !argument_datatype.is_value(parameter) {
                                // The argument and the parameter have diferent types
                                let (error_start, error_end) = argument.get_position();
                                let label = ErrorLabel {
                                    message: format!(
                                        "Expected a {}, got {:?}",
                                        parameter, argument_datatype
                                    ),
                                    start: error_start,
                                    end: error_end,
                                };
                                let econtainer = ErrorContainer {
                                    error_code: SEMANTIC_MISMATCHED_TYPES,
                                    error_offset: error_start,
                                    labels: vec![label],
                                    note: None,
                                    help: None,
                                };
                                return Err(econtainer);
                            }
                        }

                        Ok(())
                    }
                    _ => {
                        let (error_start, error_end) = fun.get_position();
                        let label = ErrorLabel {
                            message: format!(
                                "Expected a function type, got {:?}",
                                function_datatype
                            ),
                            start: error_start,
                            end: error_end,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISMATCHED_TYPES,
                            error_offset: error_start,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(econtainer);
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
                            let label = ErrorLabel {
                                message: format!("Expected a Bool, got {}", t),
                                start: error_start,
                                end: error_end,
                            };
                            let econtainer = ErrorContainer {
                                error_code: SEMANTIC_MISMATCHED_TYPES,
                                error_offset: error_start,
                                labels: vec![label],
                                note: None,
                                help: None,
                            };
                            return Err(econtainer);
                        }
                    }
                    ("!", Type::Function(_, _)) => {
                        // Error: unary negation can only be applied to a Bool
                        let (error_start, error_end) = expression.get_position();
                        let label = ErrorLabel {
                            message: format!("Expected a Bool, got a function"),
                            start: error_start,
                            end: error_end,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISMATCHED_TYPES,
                            error_offset: error_start,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(econtainer);
                    }
                    ("-", Type::Value(t)) => {
                        if t == "Int" || t == "Float" {
                            // Ok, empty
                            return Ok(());
                        } else {
                            // Error: unary negation can only be applied to a Number
                            let (error_start, error_end) = expression.get_position();
                            let label = ErrorLabel {
                                message: format!("Expected a Float or Int, got a {}", t),
                                start: error_start,
                                end: error_end,
                            };
                            let econtainer = ErrorContainer {
                                error_code: SEMANTIC_MISMATCHED_TYPES,
                                error_offset: error_start,
                                labels: vec![label],
                                note: None,
                                help: None,
                            };
                            return Err(econtainer);
                        }
                    }
                    ("-", Type::Function(_, _)) => {
                        // Error: unary negation can only be applied to a Bool
                        let (error_start, error_end) = expression.get_position();
                        let label = ErrorLabel {
                            message: format!("Expected a Float or Int, got a function"),
                            start: error_start,
                            end: error_end,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_MISMATCHED_TYPES,
                            error_offset: error_start,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(econtainer);
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
                    Some(t) => {
                        // If a operator is stored as anything else
                        // it's a bug in the compiler
                        unreachable!("Compiler bug: a binary operator was registered in the symbol table as a value of type {:?}", t)
                    }
                    None => {
                        // If the operator is not found its a user error,
                        // because we allow arbitrary operators
                        let (error_start, error_end) = (op.position, op.get_end_position());
                        let label = ErrorLabel {
                            message: format!("The binary operator {} does not exist", op.value),
                            start: error_start,
                            end: error_end,
                        };
                        let econtainer = ErrorContainer {
                            error_code: SEMANTIC_INVALID_REFERENCE,
                            error_offset: error_start,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(econtainer);
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
                    let label = ErrorLabel {
                        message: format!(
                            "Expected a {}, got a {:?} on the left side of the {} operator",
                            op_params[0], left_expr_type, op.value
                        ),
                        start: error_start,
                        end: error_end,
                    };
                    let econtainer = ErrorContainer {
                        error_code: SEMANTIC_MISMATCHED_TYPES,
                        error_offset: error_start,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(econtainer);
                }

                if !right_expr_type.is_value(&op_params[1]) {
                    let (error_start, error_end) = right_expr.get_position();
                    let label = ErrorLabel {
                        message: format!(
                            "Expected a {}, got a {:?} on the right side of the {} operator",
                            op_params[1], right_expr_type, op.value
                        ),
                        start: error_start,
                        end: error_end,
                    };
                    let econtainer = ErrorContainer {
                        error_code: SEMANTIC_MISMATCHED_TYPES,
                        error_offset: error_start,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(econtainer);
                }

                // After all these checks, we are ok
                Ok(())
            }
            Expression::Array(arr) => {
                // There is some duplicated logic here with
                // the typechecking of an array in the impl
                // of the trait Typed

                // The first expression found determines the
                // type of the array

                // TODO: for now an array must have at least 1 element,
                // if the array is empty there is no way to know its type.
                // TODO: if the array is empty then its
                // datatype should be determined by its usage.
                if arr.exps.is_empty() {
                    let label = ErrorLabel {
                        message: format!(
                            "An array must have at least 1 element to determine its type. This will be fixed later."
                        ),
                        start: arr.start,
                        end: arr.end,
                    };
                    let econtainer = ErrorContainer {
                        error_code: COMPILER_TODO,
                        error_offset: arr.start,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(econtainer);
                }

                let mut expressions = arr.exps.iter();
                let first_expr = expressions.next().unwrap();
                let first_type = first_expr.get_type(scope)?;

                // then check that every expression has the same type
                for exp in expressions {
                    let exp_type = exp.get_type(scope)?;
                    if !exp_type.equals(&first_type) {
                        // TODO: subtyping

                        // error, found an item with a diferent datatype
                        let (error_start, error_end) = exp.get_position();
                        let label = ErrorLabel {
                        message: format!(
                                "All elements of an array must have the same datatype. Expected {:?}, got {:?}",
                                first_type,
                                exp_type,
                        ),
                        start: error_start,
                        end: error_end,
                    };
                        let econtainer = ErrorContainer {
                            error_code: COMPILER_TODO,
                            error_offset: error_start,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(econtainer);
                    }
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
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
            Err(err) => {
                //assert_eq!(err.reason, "Cannot find `print` in this scope.");
                assert_eq!(err.error_offset, 0);
                //assert_eq!(err.error_end, 5);
            }
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
            Err(e) => {
                //assert_eq!(e.reason, "Expected a String, got Value(\"Int\")");
                assert_eq!(e.error_offset, 6);
                //assert_eq!(e.error_end, 9);
            }
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
            Err(e) => {
                //assert_eq!(e.reason, "Expected 1 arguments, got 0");
                assert_eq!(e.error_offset, 5);
                //assert_eq!(e.error_end, 7);
            }
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
            Err(e) => {
                //assert_eq!(e.reason, "Expected 1 arguments, got 2");
                assert_eq!(e.error_offset, 5);
                //assert_eq!(e.error_end, 15);
            }
        }
    }
}
