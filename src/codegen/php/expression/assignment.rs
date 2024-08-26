use crate::{codegen::Transpilable, php_ast::PSimpleAssignment};

impl Transpilable for PSimpleAssignment<'_> {
    fn transpile(&self) -> String {
        let variable_name = self.variable;
        let expression_str = self.assignment.transpile();

        format!("${} = {}", variable_name, expression_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::Transpilable,
        php_ast::{PExpresssion, PPrimary, PSimpleAssignment},
    };

    #[test]
    fn should_gen_assignment() {
        let variable = &String::from("name");
        let value = String::from("John");
        let assignment = PExpresssion::Primary(PPrimary::StringLiteral(&value));

        let ast = PSimpleAssignment {
            variable,
            assignment: Box::new(assignment),
        };
        let output = ast.transpile();

        assert_eq!("$name = \"John\"", output)
    }
}
