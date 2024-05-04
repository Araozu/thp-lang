use crate::{semantic::impls::SemanticCheck, syntax::ast::TopLevelDeclaration};

impl SemanticCheck for TopLevelDeclaration<'_> {
    fn check_semantics(&self, scope: &crate::semantic::symbol_table::SymbolTable) -> Result<(), crate::error_handling::MistiError> {
        match self {
            TopLevelDeclaration::Binding(binding) => binding.check_semantics(scope),
            TopLevelDeclaration::FunctionDeclaration(function) => function.check_semantics(scope),
            _ => panic!("Not implemented"),
        }
    }
}

