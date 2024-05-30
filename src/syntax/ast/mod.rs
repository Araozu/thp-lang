use crate::lexic::token::Token;

use self::functions::FunctionCall;

pub mod functions;
pub mod statement;
pub mod var_binding;

pub struct ModuleAST<'a> {
    pub declarations: Vec<TopLevelDeclaration<'a>>,
}

// TODO: this and Statement should merge
#[derive(Debug)]
pub enum TopLevelDeclaration<'a> {
    Binding(var_binding::Binding<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    pub identifier: &'a Token,
    pub return_type: Option<&'a Token>,
    pub params_list: Box<ParamsList<'a>>,
    pub block: Box<Block<'a>>,
}

#[derive(Debug)]
pub struct Block<'a> {
    // TODO: this should be a Vec of Statement|Expression
    pub statements: Vec<statement::Statement<'a>>,
}

#[derive(Debug)]
pub struct ParamsList<'a> {
    pub parameters: Vec<Parameter<'a>>,
}

#[derive(Debug)]
pub struct Parameter<'a> {
    pub identifier: &'a String,
    pub datatype: &'a String,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Int(&'a String),
    Float(&'a String),
    String(&'a String),
    Boolean(bool),
    Identifier(&'a String),
    FunctionCall(FunctionCall<'a>),
    UnaryOperator(&'a String, Box<Expression<'a>>),
    BinaryOperator(Box<Expression<'a>>, Box<Expression<'a>>, &'a String),
}
