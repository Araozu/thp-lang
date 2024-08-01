use crate::{error_handling::MistiError, syntax::ast::ModuleAST};

mod checks;
mod impls;
mod std;
mod symbol_table;
mod types;

use impls::SemanticCheck;

// What to do?
// 1. Create a mutable symbol table
// 2. Walk the AST
// 3. Add the symbols declared to the symbol table, annotating them with their type
// 4. Check if the symbols used are declared

/// Checks that the AST is semantically correct
pub fn check_semantics(ast: &ModuleAST) -> Result<(), MistiError> {
    // For now there's only support for a single file
    // TODO: Receive a symbol table as a reference and work on it.
    // this way we can implement a unique symbol table for REPL session
    let mut global_scope = symbol_table::SymbolTable::new();
    std::populate(&mut global_scope);

    ast.check_semantics(&global_scope)
}

#[cfg(test)]
mod tests {
    use crate::semantic::types::Type;

    use super::symbol_table::SymbolTable;

    #[test]
    fn test_1() {
        let global_scope = SymbolTable::new();
        let main_function = Type::Function(vec![], String::from("Unit"));

        global_scope.insert("main".into(), main_function);

        assert!(global_scope.test(&"main".into()));
    }

    #[test]
    fn test_2() {
        let global_scope = SymbolTable::new();

        let main_function = Type::Function(vec![], String::from("Unit"));
        global_scope.insert("main".into(), main_function);
        global_scope.insert("db_url".into(), Type::Value("String".into()));

        let add_function = Type::Function(vec!["Int".into(), "Int".into()], "Int".into());

        global_scope.insert("add".into(), add_function);

        let main_function_scope = SymbolTable::new_from_parent(&global_scope);
        main_function_scope.insert("message".into(), Type::Value("String".into()));

        assert!(main_function_scope.test(&"message".into()));
        assert!(main_function_scope.test(&"db_url".into()));
        assert_eq!(main_function_scope.test(&"non_existant".into()), false);

        let add_function_scope = SymbolTable::new_from_parent(&global_scope);

        add_function_scope.insert("a".into(), Type::Value("Int".into()));
        add_function_scope.insert("b".into(), Type::Value("Int".into()));

        assert!(add_function_scope.test(&"a".into()));
        global_scope.insert("test".into(), Type::Value("Int".into()));
    }
}
