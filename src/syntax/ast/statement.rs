use super::functions::FunctionCall;

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
}
