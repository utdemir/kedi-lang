use crate::renamer;

use super::simple;

pub fn simplify(module: renamer::Module) -> simple::Module {
    let mut functions = vec![];
    for stmt in module.statements {}
    simple::Module { functions }
}
