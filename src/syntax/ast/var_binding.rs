use super::Expression;

#[derive(Debug)]
pub struct Binding<'a> {
    pub datatype: Option<String>,
    pub identifier: Box<String>,
    pub expression: Expression<'a>,
    pub is_mutable: bool,
}
