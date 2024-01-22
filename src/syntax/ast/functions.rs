use super::Expression;

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub arguments: Box<ArgumentsList>,
}

#[derive(Debug)]
pub struct ArgumentsList {
    pub arguments: Vec<Expression>,
}
