use crate::{
    error_handling::{semantic_error::SemanticError, MistiError},
    semantic::{
        impls::SemanticCheck,
        symbol_table::SymbolTable,
        types::{Type, Typed},
    },
    syntax::ast::{loops::ForLoop, Positionable},
};

impl SemanticCheck for ForLoop<'_> {
    fn check_semantics(&self, scope: &SymbolTable) -> Result<(), MistiError> {
        // TODO: Implement a generic Collection interface?
        // Use Traversable from PHP?
        // for now this is restricted to arrays

        let collection_type = self.collection.get_type(scope)?;
        // This will be a single Type,
        let item_type = match collection_type {
            Type::Generic(t, type_params) if t == "Array" => {
                if type_params.len() != 1 {
                    unreachable!(
                        "Compiler error: found an Array[] with more than 1 type parameter: {:?}",
                        type_params
                    )
                }

                type_params
            }
            _ => {
                // error, types other than an Array are not supported
                let (error_start, error_end) = self.collection.get_position();
                return Err(MistiError::Semantic(SemanticError {
                    error_start,
                    error_end,
                    reason: format!("Only Array[T] are allowed as a for-loop collection."),
                }));
            }
        };
        let item_type = &item_type[0];

        let loop_scope = SymbolTable::new_from_parent(&scope);

        // Create a new scope, insert key,value
        if let Some(key) = self.key {
            // Since for now this only supports Array[T], key
            // can only be a Int
            loop_scope.insert(key.value.clone(), Type::Value("Int".into()));
        }
        // TODO: Add lifetimes to scoping instead of cloning
        loop_scope.insert(self.value.value.clone(), item_type.clone());

        // Check every statement inside the block
        self.body.check_semantics(&loop_scope)?;

        // Ok
        Ok(())
    }
}
