
pub struct ModuleAST<'a> {
    pub bindings: Vec<Binding<'a>>,
}

pub enum Binding<'a> {
    Val(ValBinding<'a>),
}

pub struct ValBinding<'a> {
    pub identifier: &'a String,
    pub expression: Expression<'a>,
}

pub enum Expression<'a> {
    Number(&'a String),
}
