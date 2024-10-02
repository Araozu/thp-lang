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
                message: format!("Expected a function type, got {:?}", function_datatype),
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
