use super::symbol_table::{SymbolTable, _NUMBER};
use super::ast_types::{ModuleAST, Binding};

/// Checks the AST. In the future should return a list of errors.
pub fn check_ast<'a>(ast: &'a mut ModuleAST, symbol_table: &'a mut SymbolTable) {
    for binding in &ast.bindings {
        match binding {
            Binding::Val(val_binding) => {
                // TODO: create a function to get the datatype, instead of a hardcoded value
                symbol_table.add(val_binding.identifier, _NUMBER);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax;
    use crate::lexic;

    use super::*;

    /*
    val identifier = 20

    [Binding]
    | identifier
    | [Expression]
      | [Number]
        | 20

    - Check [Expression] is valid
    - Check type of [Expression]
    - Check if `identifier` already exists in the symbol table
    - Create entry in symbol table

    ->

    SymbolTable {
        identifier: Num
    }
     */

    #[test]
    fn should_update_symbol_table() {
        let tokens = lexic::get_tokens(&String::from("val identifier = 20")).unwrap();
        let mut table = SymbolTable::new();
        let mut ast = syntax::construct_ast(&tokens).unwrap();

        check_ast(&mut ast, &mut table);

        let result = table.test("identifier");
        assert_eq!(true, result);
    }
}
