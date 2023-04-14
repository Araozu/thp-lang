use crate::{ast_types::Expression, symbol_table::SymbolTable};

use super::datatype::Datatype;

trait Typed<'a> {
    fn t(&self, symbol_table: &'a mut SymbolTable) -> Datatype;
}

impl<'a> Typed<'a> for Expression<'a> {
    fn t(&self, symbol_table: &'a mut SymbolTable) -> Datatype {
        match self {
            Expression::Number(_) => Datatype::num(),
            Expression::String(_) => Datatype::str(),
            Expression::Boolean(_) => Datatype::bool(),
            Expression::Identifier(id) => {
                let res = symbol_table.get_type(id).unwrap();
                res.clone()
            }
        }
    }
}



#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn should_get_type_of_primitives() {
        let mut t = SymbolTable::new();

        let s = String::from("322");
        let exp = Expression::Number(&s);
        let datatype = exp.t(&mut t);

        assert!(datatype == Datatype::num());

        let s = String::from("hello");
        let exp = Expression::String(&s);
        let datatype = exp.t(&mut t);

        assert!(datatype == Datatype::str());

        let exp = Expression::Boolean(true);
        let datatype = exp.t(&mut t);

        assert!(datatype == Datatype::bool());
    }

    #[test]
    fn shold_get_type_of_existing_id() {
        let mut table = SymbolTable::new();
        table.insert("my_number", Datatype::num());

        let id = String::from("my_number");
        let exp = Expression::Identifier(&id);

        assert!(exp.t(&mut table) == Datatype::num());
    }
}

