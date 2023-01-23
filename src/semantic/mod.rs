use crate::symbol_table;

use super::symbol_table::SymbolTable;
use super::ast_types::ModuleAST;

pub fn check_ast<'a>(ast: &'a mut ModuleAST, symbol_table: &'a mut SymbolTable) -> Option<ModuleAST<'a>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    val identifier = 20

    [Binding]
    | identifier
    | [Expression]
      | [Number]
        | 20

    - Check [Expression] is valid
    - Check type of [Expression]
    - Check if `identifier` already exists in the symbol table
    - Create entry in symbol table

    ->

    SymbolTable {
        identifier: Num
    }
     */
}
