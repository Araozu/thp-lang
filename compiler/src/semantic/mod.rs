use super::ast_types::{Binding, Expression, ModuleAST};
use super::symbol_table::{SymbolTable, _BOOLEAN, _NUMBER, _STRING};

/// Checks the AST. In the future should return a list of errors.
pub fn check_ast<'a>(ast: &'a mut ModuleAST, symbol_table: &'a mut SymbolTable) {
    for binding in &ast.bindings {
        match binding {
            Binding::Val(binding) => {
                symbol_table.add(
                    binding.identifier,
                    get_expression_type(&binding.expression, symbol_table).as_str(),
                );
            }
            Binding::Var(binding) => {
                symbol_table.add(
                    binding.identifier,
                    get_expression_type(&binding.expression, symbol_table).as_str(),
                );
            }
        }
    }
}

fn get_expression_type(exp: &Expression, symbol_table: &SymbolTable) -> String {
    match exp {
        Expression::Number(_) => String::from(_NUMBER),
        Expression::String(_) => String::from(_STRING),
        Expression::Boolean(_) => String::from(_BOOLEAN),
        Expression::Identifier(id) => {
            match symbol_table.get_type(*id) {
                Some(datatype) => datatype,
                None => {
                    // Should add an error to the list instead of panicking
                    panic!("Semantic analysis: identifier {} not found", id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexic;
    use crate::symbol_table::_BOOLEAN;
    use crate::symbol_table::_STRING;
    use crate::syntax;

    use super::*;

    fn test_type(input: String, datatype: &str) -> bool {
        let tokens = lexic::get_tokens(&input).unwrap();
        let mut table = SymbolTable::new();
        let mut ast = syntax::construct_ast(&tokens).unwrap();

        check_ast(&mut ast, &mut table);

        table.check_type("a", datatype)
    }

    #[test]
    fn should_update_symbol_table() {
        let tokens = lexic::get_tokens(&String::from("val identifier = 20")).unwrap();
        let mut table = SymbolTable::new();
        let mut ast = syntax::construct_ast(&tokens).unwrap();

        check_ast(&mut ast, &mut table);

        let result = table.test("identifier");
        assert_eq!(true, result);
    }

    #[test]
    fn should_get_correct_type() {
        assert!(test_type(String::from("val a = 322"), _NUMBER));
        assert!(test_type(String::from("var a = 322"), _NUMBER));

        assert!(test_type(String::from("val a = \"str\" "), _STRING));
        assert!(test_type(String::from("var a = \"str\" "), _STRING));

        assert!(test_type(String::from("val a = false"), _BOOLEAN));
        assert!(test_type(String::from("var a = true"), _BOOLEAN));
    }

    #[test]
    fn should_get_type_from_identifier() {
        let mut table = SymbolTable::new();
        let tokens = lexic::get_tokens(&String::from("val identifier = 20")).unwrap();
        let mut ast = syntax::construct_ast(&tokens).unwrap();

        // Add an identifier
        check_ast(&mut ast, &mut table);

        let tokens = lexic::get_tokens(&String::from("val newValue = identifier")).unwrap();
        let mut ast = syntax::construct_ast(&tokens).unwrap();

        // Add a new value that references an identifier
        check_ast(&mut ast, &mut table);

        // The type should be Num
        let current_type = table.get_type("newValue").unwrap();
        assert_eq!(_NUMBER, current_type);
    }
}
