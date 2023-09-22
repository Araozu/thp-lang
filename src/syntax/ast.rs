pub struct ModuleAST {
    pub declarations: Vec<TopLevelDeclaration>,
}

#[derive(Debug)]
pub enum TopLevelDeclaration {
    Binding(Binding),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub identifier: Box<String>,
    pub params_list: Box<ParamsList>,
}

#[derive(Debug)]
pub struct Block {}

#[derive(Debug)]
pub struct ParamsList {}

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
