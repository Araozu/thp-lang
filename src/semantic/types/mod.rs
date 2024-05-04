// This crate provides an interface and implementations
// for determining the datatypes of the language constructs.

use crate::lexic::token::Token;

mod expression;


pub trait Typed {
    fn get_type(&self) -> String; 
}
