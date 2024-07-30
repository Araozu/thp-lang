use std::os::linux::raw::stat;

use crate::php_ast::{PhpAst, PhpExpression, PhpStatement};

use super::Transpilable;

impl Transpilable for PhpAst<'_> {
    fn transpile(&self) -> String {
        let mut fragments = vec![String::from("<?php\n")];

        for statement in self.statements.iter() {
            fragments.push(statement.transpile());
        }

        fragments.join("")
    }
}

impl Transpilable for PhpStatement<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpStatement::PhpEchoStatement(expr_list) => {
                let expressions_vec = expr_list
                    .expressions
                    .iter()
                    .map(|e| e.transpile())
                    .collect::<Vec<_>>();

                let expressions_str = if expressions_vec.is_empty() {
                    "\"\"".into()
                } else {
                    expressions_vec.join(", ")
                };

                format!("echo {};", expressions_str)
            }
        }
    }
}

impl Transpilable for PhpExpression<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpExpression::String(value) => {
                format!("{}", value)
            }
        }
    }
}
