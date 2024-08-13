use super::{Expression, Positionable};

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub function: Box<Expression<'a>>,
    pub arguments: Box<ArgumentsList<'a>>,
}

impl Positionable for FunctionCall<'_> {
    fn get_position(&self) -> (usize, usize) {
        let (start, _) = self.function.get_position();
        let (_, end) = self.arguments.get_position();
        (start, end)
    }
}

#[derive(Debug)]
pub struct ArgumentsList<'a> {
    pub arguments: Vec<Expression<'a>>,
    pub paren_open_pos: usize,
    /// This is after the paren is closed
    pub paren_close_pos: usize,
}

impl Positionable for ArgumentsList<'_> {
    fn get_position(&self) -> (usize, usize) {
        (self.paren_open_pos, self.paren_close_pos)
    }
}
