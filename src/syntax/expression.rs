use crate::token::Token;

use super::types::Expression;


pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> Option<Expression> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_a_number() {
        let tokens = get_tokens(&String::from("40")).unwrap();
        let expression = try_parse(&tokens, 0).unwrap();

        match expression {
            Expression::Number(value) => assert_eq!("40", value),
        }
    }
}
