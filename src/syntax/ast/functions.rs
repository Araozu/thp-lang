use super::Expression;

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Box<Expression>,
}

#[derive(Debug)]
pub struct ArgumentsList {
    pub arguments: Vec<Expression>,
}

