use crate::{lexic::token::Token, syntax::{ParseResult, ast::functions::FunctionCall}};



pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<FunctionCall, ()> {



    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_1() {

    }
}
