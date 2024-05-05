use crate::syntax::ast::Expression;

use super::Typed;

impl Typed for Expression<'_> {
    fn get_type(&self) -> String {
        match self {
            // TODO: Distinguish between Int & Float
            Expression::Number(_) => "Int".into(),
            Expression::String(_) => "String".into(),
            Expression::Boolean(_) => "Bool".into(),
            Expression::Identifier(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
            Expression::UnaryOperator(_, _) => todo!(),
            Expression::BinaryOperator(_, _, _) => todo!(),
        }
    }
}
