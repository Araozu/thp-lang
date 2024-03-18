use crate::lexic::token::Token;

use self::functions::FunctionCall;

pub mod functions;
pub mod statement;
pub mod var_binding;

pub struct ModuleAST<'a> {
    pub declarations: Vec<TopLevelDeclaration<'a>>,
}

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
    pub params_list: Box<ParamsList>,
    pub block: Box<Block<'a>>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub statements: Vec<statement::Statement<'a>>,
}

#[derive(Debug)]
pub struct ParamsList {}

pub struct Parameter<'a> {
    pub identifier: &'a String,
    pub datatype: &'a String,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Number(&'a String),
    String(&'a String),
    Boolean(bool),
    Identifier(&'a String),
    FunctionCall(FunctionCall<'a>),
    UnaryOperator(&'a String, Box<Expression<'a>>),
    BinaryOperator(Box<Expression<'a>>, Box<Expression<'a>>, &'a String),
}
