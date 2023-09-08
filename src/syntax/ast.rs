pub struct ModuleAST {
    pub bindings: Vec<Binding>,
}

pub enum Binding {
    Val(ValBinding),
    Var(VarBinding),
}

pub struct ValBinding {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression,
}

pub struct VarBinding {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression,
}

pub enum Expression {
    Number(Box<String>),
    String(Box<String>),
    Boolean(bool),
    Identifier(Box<String>),
}
