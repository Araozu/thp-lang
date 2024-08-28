use crate::{
    error_handling::MistiError,
    semantic::{impls::SemanticCheck, symbol_table::SymbolTable},
    syntax::ast::{ModuleMembers, Statement},
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
            Statement::Conditional(_) => unimplemented!("check conditional"),
            Statement::ForLoop(_) => unimplemented!("check for loop"),
        }
    }
}
