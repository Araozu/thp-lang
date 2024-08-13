use crate::lexic::token::Token;

use self::functions::FunctionCall;
use var_binding::VariableBinding;

pub mod functions;
pub mod var_binding;

/// The AST for a whole THP file
#[derive(Debug)]
pub struct ModuleAST<'a> {
    /// All the productions in the file
    pub productions: Vec<ModuleMembers<'a>>,
}

/// Enum for productions available at the top level of a file
#[derive(Debug)]
pub enum ModuleMembers<'a> {
    // TODO: In the future implement module import
    Stmt(Statement<'a>),
    Expr(Expression<'a>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Binding(VariableBinding<'a>),
    FnDecl(FunctionDeclaration<'a>),
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
    pub members: Vec<BlockMember<'a>>,
}

/// Enum for productions available at the block level
#[derive(Debug)]
pub enum BlockMember<'a> {
    Stmt(Statement<'a>),
    Expr(Expression<'a>),
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
    Int(&'a Token),
    Float(&'a String),
    String(&'a String),
    Boolean(bool),
    Identifier(&'a Token),
    FunctionCall(FunctionCall<'a>),
    UnaryOperator(&'a String, Box<Expression<'a>>),
    BinaryOperator(Box<Expression<'a>>, Box<Expression<'a>>, &'a String),
}

impl Expression<'_> {
    /// Returns the absolute start and end position
    /// of this expression
    pub fn get_position(&self) -> (usize, usize) {
        match self {
            Expression::Identifier(id) => (id.position, id.get_end_position()),
            Expression::Int(id) => (id.position, id.get_end_position()),
            Expression::Float(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Boolean(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
            Expression::UnaryOperator(_, _) => todo!(),
            Expression::BinaryOperator(_, _, _) => todo!(),
        }
    }
}
