mod prune_single_use;
mod remove_nops;

use super::simple;

pub fn run(fun: &mut simple::Module) {
    for stmt in &mut fun.statements {
        match stmt {
            simple::TopLevelStmt::FunDecl(fun) => {
                let fun_impl = &mut fun.value.implementation.value;
                prune_single_use::run(fun_impl);
                remove_nops::run(fun_impl);
            }
        }
    }
}
