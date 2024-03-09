use super::Expression;

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub function: Box<Expression<'a>>,
    pub arguments: Box<ArgumentsList<'a>>,
}

#[derive(Debug)]
pub struct ArgumentsList<'a> {
    pub arguments: Vec<Expression<'a>>,
}
