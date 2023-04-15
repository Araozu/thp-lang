use super::ast_types::{Binding, ModuleAST};
use super::symbol_table::SymbolTable;

mod datatype;
mod type_check;

use type_check::Typed;

pub use datatype::Datatype;

/// Checks the AST. In the future should return a list of errors.
pub fn check_ast<'a>(ast: &'a ModuleAST, symbol_table: &'a mut SymbolTable) {
    for binding in ast.bindings.iter() {
        match binding {
            Binding::Val(b) => {
                let datatype = b.expression.t(symbol_table);
                let identifier = b.identifier;

                // TODO: check datatype of a explicit datatype, e.g. `Str val x = 322`

                symbol_table.insert(identifier.as_str(), datatype);
            }
            Binding::Var(b) => {
                let datatype = b.expression.t(symbol_table);
                let identifier = b.identifier;

                // TODO: check datatype of a explicit datatype, e.g. `Str val x = 322`

                symbol_table.insert(identifier.as_str(), datatype);
            }
        }
    }
}

#[cfg(test)]
mod t {
    use crate::ast_types::Expression;

    use super::*;

    #[test]
    fn should_insert_into_symbol_table() {
        let s1 = String::from("id");
        let s2 = String::from("322");
        let binding = Binding::Val(crate::ast_types::ValBinding {
            datatype: None,
            identifier: &s1,
            expression: Expression::Number(&s2),
        });

        let mut table = SymbolTable::new();

        check_ast(
            &ModuleAST {
                bindings: vec![binding],
            },
            &mut table,
        );

        assert!(table.has_id(&String::from("id")));
        assert!(table.check_type(&String::from("id"), Datatype::num()));
    }

    #[test]
    fn should_insert_id_reference() {
        let s1 = String::from("id");
        let s2 = String::from("322");
        let binding = Binding::Val(crate::ast_types::ValBinding {
            datatype: None,
            identifier: &s1,
            expression: Expression::Number(&s2),
        });

        let mut table = SymbolTable::new();

        check_ast(
            &ModuleAST {
                bindings: vec![binding],
            },
            &mut table,
        );

        let s1 = String::from("id2");
        let s2 = String::from("id");
        let binding = Binding::Val(crate::ast_types::ValBinding {
            datatype: None,
            identifier: &s1,
            expression: Expression::Identifier(&s2),
        });

        check_ast(
            &ModuleAST {
                bindings: vec![binding],
            },
            &mut table,
        );

        assert!(table.has_id(&String::from("id2")));
        assert!(table.check_type(&String::from("id2"), Datatype::num()));
    }
}
