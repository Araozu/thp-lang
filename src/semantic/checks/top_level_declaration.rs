use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{Expression, ModuleMembers, Statement},
};

impl SemanticCheck for ModuleMembers<'_> {
    fn check_semantics(
        &self,
        scope: &crate::semantic::symbol_table::SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        match self {
            ModuleMembers::Stmt(statement) => statement.check_semantics(scope),
            ModuleMembers::Expr(expression) => expression.check_semantics(scope),
        }
    }
}

// TODO: Move to its own file when it grows
impl SemanticCheck for Statement<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        match self {
            Statement::Binding(b) => b.check_semantics(scope),
            Statement::FnDecl(f) => f.check_semantics(scope),
        }
    }
}

// TODO: Move to its own file when it grows
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
            Expression::Int(_) => {},
            Expression::Float(_) => {},
            Expression::String(_) => {},
            Expression::Boolean(_) => {},
            _ => todo!("Check semantics for expression other than function call and primitive"),
        }

        Ok(())
    }
}
