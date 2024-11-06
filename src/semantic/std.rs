//! Naively provides the standard library for THP
//! by directly inserting the definitions into the
//! Symbol Table

use super::{
    symbol_table::SymbolTable,
    types::{
        global::{INT, STRING, VOID},
        Type,
    },
};

/// Populates the symbol table with the stdlib
pub fn populate(table: &mut SymbolTable) {
    // print: (String) -> (Void)
    let print_fn = Type::Function(vec![STRING.into()], VOID.into());
    table.insert("print".into(), print_fn);

    // + operator (Int, Int) -> Int
    let plus_op = Type::Function(vec![INT.into(), INT.into()], INT.into());
    table.insert("+".into(), plus_op);

    // - operator (Int, Int) -> Int
    let plus_op = Type::Function(vec![INT.into(), INT.into()], INT.into());
    table.insert("-".into(), plus_op);
}
