use crate::ast_types::Expression;
use super::Transpilable;

impl Transpilable for Expression<'_> {
    fn transpile(&self) -> String {
        match self {
            Expression::Number(value) => {
                String::from(*value)
            }
            Expression::String(value) => {
                format!("\"{}\"", *value)
            }
            Expression::Boolean(value) => {
                String::from(if *value {"true"} else {"false"})
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_types::Expression;

    #[test]
    fn should_transpile_number() {
        let str = String::from("42");
        let exp = Expression::Number(&str);
        let result = exp.transpile();

        assert_eq!("42", result);
    }
    
    #[test]
    fn should_transpile_string() {
        let str = String::from("Hello world");
        let exp = Expression::String(&str);
        let result = exp.transpile();

        assert_eq!("\"Hello world\"", result);
    }
    
    #[test]
    fn should_transpile_boolean() {
        let exp = Expression::Boolean(true);
        let result = exp.transpile();
        
        assert_eq!("true", result);
    }
}
