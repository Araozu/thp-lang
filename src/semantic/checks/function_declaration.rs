use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{impls::SemanticCheck, symbol_table::SymbolTable, types::Type},
    syntax::ast::{BlockMember, FunctionDeclaration, Statement},
};

impl SemanticCheck for FunctionDeclaration<'_> {
    fn check_semantics(
        &self,
        scope: &crate::semantic::symbol_table::SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        let function_name = self.identifier.value.clone();

        // Check that the function is not already defined
        if scope.test(&function_name) {
            let error = SemanticError {
                error_start: self.identifier.position,
                error_end: self.identifier.get_end_position(),
                reason: format!(
                    "Duplicated: A symbol with name {} was already defined",
                    function_name
                ),
            };

            return Err(MistiError::Semantic(error));
        }

        // Create a new scope and use it in the function block
        let function_scope = SymbolTable::new_from_parent(scope);

        // TODO: Check the return type of the function body
        // This should be the last expression in the block
        for stmt in self.block.members.iter() {
            match stmt {
                BlockMember::Stmt(Statement::Binding(b)) => {
                    if let Err(err) = b.check_semantics(&function_scope) {
                        return Err(err);
                    }
                }
                BlockMember::Stmt(Statement::FnDecl(f)) => {
                    // TODO: (for now) a function cannot be declared inside another function.
                    let error = SemanticError {
                        error_start: f.identifier.position,
                        error_end: f.identifier.get_end_position(),
                        reason: format!(
                            "A function cannot be defined inside another function."
                        ),
                    };

                    return Err(MistiError::Semantic(error));
                }
                BlockMember::Expr(e) => e.check_semantics(scope)?,
            }
        }

        // TODO: Check that the return type of the function
        // matches the return type of the last expression

        scope.insert(function_name, Type::Function(vec![], "Unit".into()));

        Ok(())
    }
}
