use crate::syntax::ast::{ModuleMembers, Statement};

use super::Transpilable;

impl Transpilable for ModuleMembers<'_> {
    fn transpile(&self) -> String {
        match self {
            ModuleMembers::Stmt(Statement::Binding(b)) => b.transpile(),
            ModuleMembers::Stmt(Statement::FnDecl(f)) => f.transpile(),
            _ => todo!("Not implemented: Transpilable for Expression"),
        }
    }
}
