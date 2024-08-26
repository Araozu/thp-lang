use crate::{codegen::Transpilable, php_ast::php_ast_2::PFunctionCall};

impl Transpilable for PFunctionCall<'_> {
    fn transpile(&self) -> String {
        let args: Vec<_> = self.arguments.iter().map(|a| a.transpile()).collect();

        format!("{}({})", self.function_name, args.join(", "))
    }
}
