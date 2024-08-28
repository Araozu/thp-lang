use crate::lexic::token::Token;

use self::functions::FunctionCall;
use loops::ForLoop;
use var_binding::VariableBinding;

pub mod functions;
pub mod loops;
pub mod var_binding;

/// Trait that allows nodes to inform
/// on where they start and end on the source code
pub trait Positionable {
    fn get_position(&self) -> (usize, usize);
}

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
    // TODO: Implement conditionals as expressions
    Conditional(Conditional<'a>),
    ForLoop(ForLoop<'a>),
}

#[derive(Debug)]
pub struct Conditional<'a> {
    pub if_member: Condition<'a>,
    pub else_if_members: Vec<Condition<'a>>,
    pub else_block: Option<Block<'a>>,
}

#[derive(Debug)]
pub struct Condition<'a> {
    pub condition: Expression<'a>,
    pub body: Block<'a>,
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
    pub start: usize,
    pub end: usize,
    pub members: Vec<BlockMember<'a>>,
}

impl Positionable for Block<'_> {
    fn get_position(&self) -> (usize, usize) {
        (self.start, self.end)
    }
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
    Float(&'a Token),
    String(&'a Token),
    Boolean(&'a Token),
    Identifier(&'a Token),
    FunctionCall(FunctionCall<'a>),
    /// operator, right expression
    UnaryOperator(&'a Token, Box<Expression<'a>>),
    /// left expression, right expression, operator
    BinaryOperator(Box<Expression<'a>>, Box<Expression<'a>>, &'a Token),
    Array(Array<'a>),
}

#[derive(Debug)]
pub struct Array<'a> {
    pub exps: Vec<Expression<'a>>,
    /// The position of the open bracket [
    pub start: usize,
    /// The position of the closed bracket ]
    pub end: usize,
}

impl Positionable for Expression<'_> {
    /// Returns the absolute start and end position
    /// of this expression
    fn get_position(&self) -> (usize, usize) {
        match self {
            Expression::Identifier(id) => (id.position, id.get_end_position()),
            Expression::Int(id) => (id.position, id.get_end_position()),
            Expression::Float(id) => (id.position, id.get_end_position()),
            Expression::String(id) => (id.position, id.get_end_position()),
            Expression::Boolean(id) => (id.position, id.get_end_position()),
            Expression::FunctionCall(f) => f.get_position(),
            Expression::UnaryOperator(operator, exp) => {
                let start = operator.position;
                let (_, end) = exp.get_position();
                (start, end)
            }
            Expression::BinaryOperator(left_expr, right_expr, _) => {
                let (start, _) = left_expr.get_position();
                let (_, end) = right_expr.get_position();
                (start, end)
            }
            Expression::Array(Array {
                start,
                end,
                exps: _,
            }) => (*start, *end),
        }
    }
}
