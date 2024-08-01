use crate::{codegen::Transpilable, php_ast::PhpAssignmentExpression};

impl Transpilable for PhpAssignmentExpression<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpAssignmentExpression::Primary(p) => p.transpile(),
            PhpAssignmentExpression::SimpleAssignment(assignment) => {
                let variable_name = &assignment.variable;
                let expression_str = assignment.assignment.transpile();

                format!("${} = {}", variable_name, expression_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::Transpilable,
        php_ast::{PhpAssignmentExpression, PhpPrimaryExpression, PhpSimpleAssignment},
    };

    #[test]
    fn should_gen_assignment() {
        let variable = String::from("name");
        let value = String::from("John");
        let assignment = PhpPrimaryExpression::StringLiteral(&value);

        let ast = PhpAssignmentExpression::SimpleAssignment(PhpSimpleAssignment {
            variable,
            assignment,
        });
        let output = ast.transpile();

        assert_eq!("$name = \"John\"", output)
    }
}
