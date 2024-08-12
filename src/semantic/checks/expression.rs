use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{impls::SemanticCheck, symbol_table::SymbolTable, types::{Type, Typed}},
    syntax::ast::Expression,
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
                            return Err(MistiError::Semantic(SemanticError {
                                // TODO: fix
                                error_start: 0,
                                error_end: 1,
                                reason: format!(
                                    "Expected {} arguments, found {}",
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
                                return Err(MistiError::Semantic(SemanticError {
                                    // TODO: fix
                                    error_start: 0,
                                    error_end: 1,
                                    reason: format!(
                                        "Expected datatype {}, got {:?}",
                                        parameter, argument
                                    ),
                                }));
                            }
                        }
                    }
                    _ => {
                        return Err(MistiError::Semantic(SemanticError {
                            // TODO: fix
                            error_start: 0,
                            error_end: 1,
                            reason: format!(
                                "Expected a function type, got {:?}",
                                function_datatype
                            ),
                        }));
                    }
                }
            }
            Expression::Int(_) => {}
            Expression::Float(_) => {}
            Expression::String(_) => {}
            Expression::Boolean(_) => {}
            _ => todo!("Check semantics for expression other than function call and primitive"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error_handling::MistiError, lexic::token::Token, semantic::{impls::SemanticCheck, symbol_table::SymbolTable}, syntax::ast::{functions::{ArgumentsList, FunctionCall}, Expression}};

    #[test]
    fn should_error() {
        // source code: `print()`
        let expr_token = Token::new_identifier("print".into(), 0);
        let expr_function = Expression::Identifier(&expr_token);
        let arguments = ArgumentsList {
            arguments: vec![]
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
            },
            Err(e) => panic!("Expected a Semantic error, got {:?}", e)
        }
    }
}
