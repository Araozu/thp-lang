pub struct ModuleAST<'a> {
    pub bindings: Vec<Binding<'a>>,
}

pub enum Binding<'a> {
    Val(ValBinding<'a>),
    Var(VarBinding<'a>),
}

pub struct ValBinding<'a> {
    pub datatype: Option<String>,
    pub identifier: &'a String,
    pub expression: Expression<'a>,
}

pub struct VarBinding<'a> {
    pub datatype: Option<String>,
    pub identifier: &'a String,
    pub expression: Expression<'a>,
}

pub enum Expression<'a> {
    Number(&'a String),
    String(&'a String),
    Boolean(bool),
    Identifier(&'a String),
}
