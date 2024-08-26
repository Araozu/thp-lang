pub mod expression;
pub mod functions;
pub mod module_ast;
pub mod statement;

/// Implemented by AST nodes that can be transformed to PHP
pub trait PHPTransformable<'a> {
    type Item;

    fn into_php_ast(&'a self) -> Self::Item;
}
