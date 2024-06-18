use std::collections::HashMap;

use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::loc::Located;

#[derive(Debug)]
pub struct Module<'t> {
    pub statements: Vec<Located<TopLevelStmt<'t>>>,
}

#[derive(Debug)]
pub enum TopLevelStmt<'t> {
    FunDecl(FunDecl<'t>),
}

#[derive(Debug)]
pub struct FunDecl<'t> {
    pub name: Located<syntax::Identifier>,
    pub implementation: Located<FunImpl<'t>>,
    pub refs: HashMap<plain::GlobalIdentifier, syntax::Identifier>,
}

#[derive(Debug)]
pub struct FunImpl<'t> {
    pub parameters: Vec<plain::LocalIdentifier>,
    pub body: wast::core::Func<'t>,
}
