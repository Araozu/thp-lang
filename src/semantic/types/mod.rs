//! This crate provides an interface and implementations
//! for determining the datatypes of the language constructs.

use crate::error_handling::MistiError;

use super::symbol_table::SymbolTable;

mod expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Value(String),
    // TODO: Use Type instead of String to allow
    // arbitrary types
    Function(Vec<String>, String),
    // TODO: tuple, union types
    // TODO: generics
}

impl Type {
    /// Checks if this type is a value and has the specified type
    pub fn is_value(&self, datatype: impl Into<String>) -> bool {
        match self {
            Type::Value(v) if *v == datatype.into() => true,
            _ => false,
        }
    }
}

pub trait Typed {
    fn get_type(&self, scope: &SymbolTable) -> Result<Type, MistiError>;
}
