// TODO: These are for the THP AST. Eventually replace this
// with the PHP AST

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
