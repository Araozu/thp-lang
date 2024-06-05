use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{
        impls::SemanticCheck,
        symbol_table::{SymbolEntry, SymbolTable},
    },
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
                BlockMember::Stmt(Statement::FnDecl(_)) => {
                    todo!("Function declaration: semantic check not implemented")
                }
                _ => todo!("Expression: semantic check not implemented"),
            }
        }

        // TODO: Check the return type of the function

        scope.insert(
            function_name,
            SymbolEntry::new_function(vec![], "Unit".into()),
        );

        Ok(())
    }
}
