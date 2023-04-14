use super::ast_types::{Binding, Expression, ModuleAST};
use super::symbol_table::{SymbolTable, _BOOLEAN, _NUMBER, _STRING};

mod datatype;
mod type_check;

pub use datatype::Datatype;

/// Checks the AST. In the future should return a list of errors.
pub fn check_ast<'a>(ast: &'a mut ModuleAST, symbol_table: &'a mut SymbolTable) {
    
}



#[cfg(test)]
mod tests {
    use crate::lexic;
    use crate::symbol_table::_BOOLEAN;
    use crate::symbol_table::_STRING;
    use crate::syntax;

    use super::*;

}
