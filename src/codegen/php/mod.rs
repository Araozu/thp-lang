use crate::php_ast::{PhpAst, PhpStatement};

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
                // TODO: Actually generate parameters from the expr_list
                "echo \"\";".into()
            }
        }
    }
}

