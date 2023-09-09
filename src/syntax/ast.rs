pub struct ModuleAST {
    pub bindings: Vec<TopLevelConstruct>,
}

#[derive(Debug)]
pub enum TopLevelConstruct {
    Binding(Binding),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub identifier: Box<String>,
}

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

#[derive(Debug)]
pub enum Expression {
    Number(Box<String>),
    String(Box<String>),
    Boolean(bool),
    Identifier(Box<String>),
}
