use super::Expression;

#[derive(Debug)]
pub struct Binding {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression,
    pub is_mutable: bool,
}
