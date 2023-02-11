use super::symbol_table::{SymbolTable, _NUMBER, _STRING, _BOOLEAN};
use super::ast_types::{ModuleAST, Binding, Expression};

/// Checks the AST. In the future should return a list of errors.
pub fn check_ast<'a>(ast: &'a mut ModuleAST, symbol_table: &'a mut SymbolTable) {
    for binding in &ast.bindings {
        match binding {
            Binding::Val(binding) => {
                symbol_table.add(
                    binding.identifier,
                    get_expression_type(&binding.expression).as_str()
                );
            }
            Binding::Var(binding) => {
                symbol_table.add(
                    binding.identifier,
                    get_expression_type(&binding.expression).as_str(),
                );
            }
        }
    }
}

fn get_expression_type(exp: &Expression) -> String {
    match exp {
        Expression::Number(_) => String::from(_NUMBER),
        Expression::String(_) => String::from(_STRING),
        Expression::Boolean(_) => String::from(_BOOLEAN),
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::_BOOLEAN;
    use crate::symbol_table::_STRING;
    use crate::syntax;
    use crate::lexic;

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
}
