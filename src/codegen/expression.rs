use crate::ast_types::Expression;
use super::Transpilable;

impl Transpilable for Expression<'_> {
    fn transpile(&self) -> String {
        match self {
            Expression::Number(value) => {
                String::from(*value)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_types::Expression;

    #[test]
    fn number_should_transpile() {
        let str = String::from("42");
        let exp = Expression::Number(&str);
        let result = exp.transpile();

        assert_eq!("42", result);
    }
}
