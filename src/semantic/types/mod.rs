// This crate provides an interface and implementations
// for determining the datatypes of the language constructs.

mod expression;

pub trait Typed {
    fn get_type(&self) -> String; 
}
