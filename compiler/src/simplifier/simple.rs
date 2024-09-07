use crate::util::loc::{Located, Tagged, WithLoc, WithTag};
use std::collections::HashMap;

use crate::{
    parser::syntax,
    renamer::plain,
    util::loc,
    util::pp::{SExpr, SExprTerm},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SingleUseIdent {
    pub id: u32,
}

impl SExpr for SingleUseIdent {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Symbol(format!("${}", self.id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ident {
    Local(WithTag<plain::LocalIdent>),
    SingleUse(WithTag<SingleUseIdent>),
}

impl SExpr for Ident {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Ident::Local(l) => l.to_sexpr(),
            Ident::SingleUse(u) => u.to_sexpr(),
        }
    }
}

impl Tagged for Ident {
    fn tag(&self) -> loc::Tag {
        match self {
            Ident::Local(l) => l.tag(),
            Ident::SingleUse(u) => u.tag(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<TopLevelStmt>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("module", &self.statements)
    }
}

#[derive(Clone, Debug)]
pub enum TopLevelStmt {
    FunDecl(WithLoc<FunDecl>),
}

impl SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            TopLevelStmt::FunDecl(f) => f.to_sexpr(),
        }
    }
}

impl Located for TopLevelStmt {
    fn location(&self) -> loc::SrcLoc {
        match self {
            TopLevelStmt::FunDecl(f) => f.location(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunDecl {
    pub name: syntax::LIdent,
    pub implementation: WithLoc<FunImpl>,
    pub tag_map: loc::TagMap,
    pub refs: HashMap<plain::GlobalIdent, syntax::Ident>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun_decl",
            [
                &self.name.to_sexpr(),
                &SExprTerm::call(
                    "refs",
                    &self
                        .refs
                        .iter()
                        .map(|(k, v)| SExprTerm::call("ref", [&k.to_sexpr(), &v.to_sexpr()]))
                        .collect::<Vec<_>>(),
                ),
                &SExprTerm::call("implementation", [&self.implementation.to_sexpr()]),
            ],
        )
    }
}

#[derive(Clone, Debug)]
pub struct FunImpl {
    pub parameters: WithTag<Vec<WithTag<plain::LocalIdent>>>,
    pub body: WithTag<Vec<FunStmt>>,
}

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun_impl",
            [
                &SExprTerm::call("parameters", &self.parameters.value),
                &SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
            ],
        )
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub target: Ident,
    pub value: AssignmentValue,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "assignment",
            &[self.target.to_sexpr(), self.value.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub fun_name: WithTag<plain::GlobalIdent>,
    pub arguments: WithTag<Vec<Ident>>,
}

impl SExpr for Call {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "call",
            &[self.fun_name.to_sexpr(), self.arguments.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub enum AssignmentValue {
    Call(WithTag<Call>),
    Ident(Ident),
    LitNum(WithTag<plain::LitNum>),
}

impl SExpr for AssignmentValue {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            AssignmentValue::Call(c) => c.to_sexpr(),
            AssignmentValue::Ident(v) => v.to_sexpr(),
            AssignmentValue::LitNum(n) => n.to_sexpr(),
        }
    }
}

impl Tagged for AssignmentValue {
    fn tag(&self) -> loc::Tag {
        match self {
            AssignmentValue::Call(c) => c.tag(),
            AssignmentValue::Ident(i) => i.tag(),
            AssignmentValue::LitNum(n) => n.tag(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    pub id: u32,
}

impl SExpr for Label {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("label", &[SExprTerm::number(self.id)])
    }
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Ident,
    pub then: WithTag<Vec<FunStmt>>,
    pub else_: Option<WithTag<Vec<FunStmt>>>,
}

impl SExpr for If {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "if",
            &[
                self.condition.to_sexpr(),
                SExprTerm::call("then", &self.then.value),
                SExprTerm::call("else", &self.else_),
            ],
        )
    }
}

#[derive(Clone, Debug)]
pub struct Loop {
    pub body: WithTag<Vec<FunStmt>>,
}

impl SExpr for Loop {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("loop", &[SExprTerm::call("body", &self.body.value)])
    }
}

#[derive(Clone, Debug)]
pub enum FunStmt {
    // Loop(Label),
    Loop(WithTag<Loop>),
    Assignment(WithTag<Assignment>),
    Break(),
    Return(Ident),
    If(If),
    // InlineWasm(InlineWasm),
    Nop,
}

impl SExpr for FunStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            FunStmt::Loop(l) => l.to_sexpr(),
            FunStmt::Assignment(a) => a.to_sexpr(),
            FunStmt::Break() => SExprTerm::symbol("break"),
            FunStmt::Return(i) => SExprTerm::call("return", &[i.to_sexpr()]),
            FunStmt::If(i) => i.to_sexpr(),
            FunStmt::Nop => SExprTerm::symbol("nop"),
        }
    }
}
