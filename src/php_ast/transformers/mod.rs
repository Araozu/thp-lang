use crate::codegen::Transpilable;

pub mod expression;
pub mod module_ast;
pub mod statement;

/// Implemented by AST nodes that can be transformed to PHP
pub trait PHPTransformable<'a> {
    fn into_php_ast(&'a self) -> Box<(dyn Transpilable + 'a)>;
}
