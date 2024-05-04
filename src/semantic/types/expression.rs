use crate::syntax::ast::Expression;

use super::Typed;


impl Typed for Expression<'_> {
    fn get_type(&self) -> String {
        todo!()
    }
}
