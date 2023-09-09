use crate::syntax::ast::ModuleAST;

mod binding;
mod expression;
mod module_ast;

/// Trait that the AST and its nodes implement to support transformation to PHP
trait Transpilable {
    /// Transforms this struct into PHP
    fn transpile(&self) -> String;
}

/// Transforms an AST to its representation in PHP
pub fn codegen<'a>(ast: &'a ModuleAST) -> String {
    ast.transpile()
}

#[cfg(test)]
mod tests {
    use crate::{lexic, syntax};

    use super::*;

    #[test]
    fn should_codegen_1() {
        let input = String::from("val id = 322");
        let tokens = lexic::get_tokens(&input).unwrap();
        let ast = syntax::construct_ast(&tokens).unwrap();

        let out_str = codegen(&ast);

        assert_eq!("$id = 322;", out_str);
    }
}
