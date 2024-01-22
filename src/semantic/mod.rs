mod symbol_table;

use symbol_table::{SymbolEntry, SymbolTable};

// What to do?
// 1. Create a mutable symbol table
// 2. Walk the AST
// 3. Add the symbols declared to the symbol table, annotating them with their type
// 4. Check if the symbols used are declared

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut global_scope = SymbolTable::new();
        let main_function = SymbolEntry::new_function(vec![], String::from("Unit"));

        global_scope.insert("main".into(), main_function);

        assert!(global_scope.test(&"main".into()));
    }

    #[test]
    fn test_2() {
        let mut global_scope = SymbolTable::new();
        let main_function = SymbolEntry::new_function(vec![], String::from("Unit"));
        global_scope.insert("main".into(), main_function);
        global_scope.insert("db_url".into(), SymbolEntry::Variable("String".into()));

        let add_function =
            SymbolEntry::new_function(vec!["Int".into(), "Int".into()], "Int".into());
        global_scope.insert("add".into(), add_function);

        let mut main_function_scope = SymbolTable::new_from_parent(&global_scope);

        main_function_scope.insert("message".into(), SymbolEntry::Variable("String".into()));

        assert!(main_function_scope.test(&"message".into()));
        assert!(main_function_scope.test(&"db_url".into()));
        assert_eq!(main_function_scope.test(&"non_existant".into()), false);

        let mut add_function_scope = SymbolTable::new_from_parent(&global_scope);
        add_function_scope.insert("a".into(), SymbolEntry::Variable("Int".into()));
        add_function_scope.insert("b".into(), SymbolEntry::Variable("Int".into()));

        assert!(add_function_scope.test(&"a".into()));

        global_scope.insert("test".into(), SymbolEntry::Variable("Int".into()));
    }
}
