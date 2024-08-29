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
    /// The concrete type, the type parameters.
    ///
    /// E.g.: Array[Int] -> ("Array", vec!["Int"])
    ///
    /// E.g.: Map[String, Float] -> ("Map", vec!["String", "Float"])
    Generic(String, Vec<Type>),
    // TODO: tuple, union types
}

impl Type {
    /// Checks if this type is a value and has the specified type
    pub fn is_value(&self, datatype: impl Into<String>) -> bool {
        match self {
            Type::Value(v) if *v == datatype.into() => true,
            _ => false,
        }
    }

    /// Compares this type to another
    pub fn equals(&self, other: &Self) -> bool {
        use Type::*;

        match (self, other) {
            (Value(v1), Value(v2)) => v1 == v2,
            (Function(_, _), Function(_, _)) => unimplemented!("Comparison of 2 function types"),
            (Generic(_, _), Generic(_, _)) => unimplemented!("Comparison of 2 generic types"),
            _ => false,
        }
    }
}

pub trait Typed {
    fn get_type(&self, scope: &SymbolTable) -> Result<Type, MistiError>;
}
