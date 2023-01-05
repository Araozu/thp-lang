
pub struct ModuleAST {
    bindings: Vec<Binding>,
}

pub enum Binding {
    Val(ValBinding),
}

pub struct ValBinding {
    identifier: String,
    expression: Expression,
}

pub enum Expression {
    Number(String),
}
