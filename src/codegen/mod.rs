use super::ast_types::ModuleAST;

mod expression;
mod binding;
mod module_ast;

/// Trait that the AST and its nodes implement to support transformation to JavaScript
trait Transpilable {
    /// Transforms this struct into JavaScript
    fn transpile(&self) -> String;
}

/// Transforms an AST to its representation in JavaScript
pub fn codegen<'a>(ast: &'a ModuleAST) -> String {
    ast.transpile()
}





#[cfg(test)]
mod tests {
    use crate::{lexic, syntax, semantic, symbol_table::SymbolTable};

    use super::*;

    #[test]
    fn should_codegen_1() {
        let input = String::from("val id = 322");
        let tokens = lexic::get_tokens(&input).unwrap();
        let mut ast = syntax::construct_ast(&tokens).unwrap();
        let mut table = SymbolTable::new();
        semantic::check_ast(&mut ast, &mut table);

        let out_str = codegen(&ast);

        assert_eq!("const id = 322;", out_str);
    }
}

