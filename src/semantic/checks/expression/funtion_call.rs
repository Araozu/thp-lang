use crate::{
    error_handling::{
        error_messages::{SEMANTIC_MISMATCHED_ARGUMENT_COUNT, SEMANTIC_MISMATCHED_TYPES},
        ErrorContainer, ErrorLabel,
    },
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{functions::FunctionCall, Positionable},
};

impl SemanticCheck for FunctionCall<'_> {
    fn check_semantics(
        &self,
        scope: &SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        let fun = &*self.function;
        let arguments = &*self.arguments.arguments;

        let function_datatype = fun.get_type(scope)?;
        let Type::Function(parameters, _) = function_datatype else {
            let (error_start, error_end) = fun.get_position();
            let label = ErrorLabel {
                message: format!(
                    "Expected this expression to be a function, found a {:?}",
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
        };

        // Check parameters length
        if parameters.len() != arguments.len() {
            let (error_start, error_end) = self.arguments.get_position();

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
                    message: format!("Expected a {}, got {:?}", parameter, argument_datatype),
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
}

#[cfg(test)]
mod tests {
    use crate::{
        error_handling::error_messages::{
            SEMANTIC_INVALID_REFERENCE, SEMANTIC_MISMATCHED_TYPES, SEMANTIC_MISSING_REFERENCE,
        },
        lexic::{get_tokens, token::Token},
        semantic::{
            impls::SemanticCheck,
            symbol_table::SymbolTable,
            types::{global::INT, Type},
        },
        syntax::{
            ast::{functions::FunctionCall, Expression},
            parseable::Parseable,
        },
    };

    fn t(i: &str) -> Vec<Token> {
        get_tokens(&i.into()).unwrap()
    }
    fn exp<'a>(t: &'a Vec<Token>) -> FunctionCall<'a> {
        let e = Expression::try_parse(t, 0).unwrap().0;
        match e {
            Expression::FunctionCall(f) => f,
            _ => panic!("Expected to parse a function call"),
        }
    }

    #[test]
    fn should_fail_on_ref_not_exist() {
        let b = t("my_fun()");
        let expr = exp(&b);

        let scope = SymbolTable::new();

        let output = expr.check_semantics(&scope);
        match output {
            Ok(_) => panic!("Expected an error"),
            Err(err) => {
                assert_eq!(err.error_code, SEMANTIC_MISSING_REFERENCE);
                assert_eq!(err.error_offset, 0);

                let label = &err.labels[0];
                assert_eq!(label.message, "Cannot find this identifier in this scope");
                assert_eq!(label.start, 0);
                assert_eq!(label.end, 6);
            }
        }
    }

    #[test]
    fn should_fail_on_ref_not_a_function() {
        let b = t("my_fun()");
        let expr = exp(&b);

        let scope = SymbolTable::new();
        scope.insert(String::from("my_fun"), Type::Value(INT.into()));

        let output = expr.check_semantics(&scope);
        match output {
            Ok(_) => panic!("Expected an error"),
            Err(err) => {
                assert_eq!(err.error_code, SEMANTIC_MISMATCHED_TYPES);
                assert_eq!(err.error_offset, 0);

                let label = &err.labels[0];
                assert_eq!(
                    label.message,
                    "Expected this expression to be a function, found a `Int`"
                );
                assert_eq!(label.start, 0);
                assert_eq!(label.end, 6);
            }
        }
    }
}
