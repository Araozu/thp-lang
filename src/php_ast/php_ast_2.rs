/// A single PHP source code file
pub struct PFile<'a> {
    pub statements: Vec<PStatement<'a>>,
}

/// A PHP statement
pub enum PStatement<'a> {
    ExpressionStatement(PExpressionStatement<'a>),
}

/// A statement composed of a single expression,
/// whose value is discarded
///
/// ## Examples
///
/// ```php
/// 10;
/// "hello";
/// ```
pub type PExpressionStatement<'a> = PExpresssion<'a>;

/// A generic PHP expression
pub enum PExpresssion<'a> {
    FunctionCall(PFunctionCall<'a>),
    Primary(PPrimary<'a>),
    /// This comes from a THP binding
    Assignment(PSimpleAssignment<'a>),
}

pub struct PSimpleAssignment<'a> {
    pub variable: &'a String,
    pub assignment: Box<PExpresssion<'a>>,
}

/// A function call as an expression
pub struct PFunctionCall<'a> {
    /// Arbitrary expressions that compute into
    /// a function not supported
    pub function_name: &'a String,
    pub arguments: Vec<PExpresssion<'a>>,
}

/// A Primary expression: literals and variables
pub enum PPrimary<'a> {
    IntegerLiteral(&'a String),
    FloatingLiteral(&'a String),
    StringLiteral(&'a String),
    /// https://phplang.org/spec/19-grammar.html#grammar-variable
    ///
    /// Supports only variable -> callable-variable -> simple-variable -> variable-name
    ///
    /// This is a $variable
    Variable(&'a String),
    /// This is a symbol, e.g. a function name
    Symbol(&'a String),
}