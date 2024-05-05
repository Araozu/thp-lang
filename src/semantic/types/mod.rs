// This crate provides an interface and implementations
// for determining the datatypes of the language constructs.

use crate::error_handling::MistiError;

use super::symbol_table::SymbolTable;

mod expression;

pub trait Typed {
    fn get_type(&self, scope: &SymbolTable) -> Result<String, MistiError>;
}
