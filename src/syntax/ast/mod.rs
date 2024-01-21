use self::functions::FunctionCall;

pub mod functions;
pub mod statement;
pub mod var_binding;

pub struct ModuleAST {
    pub declarations: Vec<TopLevelDeclaration>,
}

#[derive(Debug)]
pub enum TopLevelDeclaration {
    Binding(var_binding::Binding),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub identifier: Box<String>,
    pub params_list: Box<ParamsList>,
    pub block: Box<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<statement::Statement>,
}

#[derive(Debug)]
pub struct ParamsList {}

pub struct Parameter {
    pub identifier: Box<String>,
    pub datatype: Box<String>,
}

#[derive(Debug)]
pub enum Expression {
    Number(Box<String>),
    String(Box<String>),
    Boolean(bool),
    Identifier(Box<String>),
    FunctionCall(FunctionCall),
    UnaryOperator(Box<String>, Box<Expression>),
    BinaryOperator(Box<Expression>, Box<Expression>, Box<String>),
}
