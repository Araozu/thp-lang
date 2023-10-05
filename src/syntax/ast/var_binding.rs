use super::Expression;

#[derive(Debug)]
pub enum Binding {
    Val(ValBinding),
    Var(VarBinding),
}

#[derive(Debug)]
pub struct ValBinding {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct VarBinding {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression,
}
