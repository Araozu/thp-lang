use crate::{
    error_handling::MistiError,
    semantic::{impls::SemanticCheck, symbol_table::SymbolTable},
    syntax::ast::{Block, BlockMember},
};

impl SemanticCheck for Block<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        for member in &self.members {
            member.check_semantics(scope)?;
        }

        Ok(())
    }
}

impl<'a> SemanticCheck for BlockMember<'a> {
    // TODO: A block may contain a function declaration statement,
    // but (afaik) those are not allowed inside conditionals/loops
    // somehow detect those?
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        match self {
            BlockMember::Stmt(s) => s.check_semantics(scope),
            BlockMember::Expr(e) => e.check_semantics(scope),
        }
    }
}
