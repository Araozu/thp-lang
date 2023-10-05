use super::{functions::FunctionCall, var_binding::Binding};

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
    Binding(Binding),
}
