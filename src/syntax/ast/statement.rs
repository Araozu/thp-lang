use super::{functions::FunctionCall, var_binding::Binding};

#[derive(Debug)]
pub enum Statement<'a> {
    FunctionCall(FunctionCall<'a>),
    Binding(Binding<'a>),
}
