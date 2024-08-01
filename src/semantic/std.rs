//! Naively provides the standard library for THP
//! by directly inserting the definitions into the
//! Symbol Table

use super::{symbol_table::SymbolTable, types::Type};

/// Populates the symbol table with the stdlib
pub fn populate(table: &mut SymbolTable) {
    // print: (String) -> (Void)
    let print_fn = Type::Function(vec!["String".into()], "Void".into());
    table.insert("print".into(), print_fn);
}
