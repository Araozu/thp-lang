use super::{functions::FunctionCall, var_binding::Binding};

// TODO: this and TopLevelDeclaration should merge
#[derive(Debug)]
pub enum Statement<'a> {
    FunctionCall(FunctionCall<'a>),
    Binding(Binding<'a>),
}
