use crate::{codegen::Transpilable, php_ast::PhpAst};

impl Transpilable for PhpAst<'_> {
    fn transpile(&self) -> String {
        let mut fragments = vec![String::from("<?php\n")];

        for statement in self.statements.iter() {
            fragments.push(statement.transpile());
        }

        fragments.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::{codegen::Transpilable, php_ast::PhpAst};

    #[test]
    fn should_transpile_empty_file() {
        let ast = PhpAst {statements: vec![]};
        let output = ast.transpile();

        assert_eq!("<?php\n", output);
    }
}

