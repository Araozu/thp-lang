// TODO: These are for the THP AST. Eventually replace this
// with the PHP AST
mod binding;
mod block;
mod expression;
mod function_call;
mod function_declaration;
mod module_ast;
mod statement;
mod top_level_construct;

mod php;

/// Trait that the AST and its nodes implement to support transformation to PHP
pub trait Transpilable {
    /// Transforms this struct into PHP
    fn transpile(&self) -> String;
}

/// Transforms an AST to its representation in PHP
pub fn codegen<'a>(ast: &'a impl Transpilable) -> String {
    ast.transpile()
}
