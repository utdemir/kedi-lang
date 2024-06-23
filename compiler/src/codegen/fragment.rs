use std::collections::HashMap;

use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::loc::Located;
use crate::util::pp;

#[derive(Debug)]
pub struct Module<'t> {
    pub statements: Vec<Located<TopLevelStmt<'t>>>,
}

impl pp::SExpr for Module<'_> {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(self.statements.iter().map(|stmt| stmt.to_sexpr()).collect())
    }
}

#[derive(Debug)]
pub enum TopLevelStmt<'t> {
    FunDecl(FunDecl<'t>),
}

impl pp::SExpr for TopLevelStmt<'_> {
    fn to_sexpr(&self) -> pp::SExprTerm {
        match self {
            TopLevelStmt::FunDecl(fun) => fun.to_sexpr(),
        }
    }
}

#[derive(Debug)]
pub struct FunDecl<'t> {
    pub name: Located<syntax::Identifier>,
    pub implementation: Located<FunImpl<'t>>,
    pub refs: HashMap<plain::GlobalIdentifier, syntax::Identifier>,
}

impl pp::SExpr for FunDecl<'_> {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(vec![
            pp::SExprTerm::Atom("fun".to_string()),
            self.name.to_sexpr(),
            self.implementation.to_sexpr(),
        ])
    }
}

#[derive(Debug)]
pub struct FunImpl<'t> {
    pub parameters: Vec<plain::LocalIdentifier>,
    pub body: wast::core::Func<'t>,
}

impl pp::SExpr for FunImpl<'_> {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(vec![
            pp::SExprTerm::List(self.parameters.iter().map(|x| x.to_sexpr()).collect()),
            pp::SExprTerm::call(
                "wasm",
                vec![pp::SExprTerm::atom(&format!("{:#?}", self.body))],
            ),
        ])
    }
}
