use crate::syntax::ast::ModuleAST;

mod binding;
mod block;
mod expression;
mod function_call;
mod function_declaration;
mod module_ast;
mod statement;
mod top_level_construct;

/// Trait that the AST and its nodes implement to support transformation to PHP
trait Transpilable {
    /// Transforms this struct into PHP
    fn transpile(&self) -> String;
}

/// Transforms an AST to its representation in PHP
pub fn codegen<'a>(ast: &'a ModuleAST) -> String {
    ast.transpile()
}
