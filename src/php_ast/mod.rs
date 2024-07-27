// Follows https://phplang.org/spec/19-grammar.html#syntactic-grammar

pub mod transformers;

/// Represents `statement-list` on the grammar
pub struct PhpAst<'a> {
    pub statements: Vec<PhpStatement<'a>>,
}

pub enum PhpStatement<'a> {
    PhpEchoStatement(PhpExpressionList<'a>),
}

pub struct PhpExpressionList<'a> {
    pub expressions: Vec<PhpExpression<'a>>,
}

pub enum PhpExpression<'a> {
    String(&'a String),
}
