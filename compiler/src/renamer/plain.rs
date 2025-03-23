use functor_derive::Functor;

use crate::parser::syntax;
use crate::util::ax::Ax;
use crate::util::bimap::Bimap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocalIdent {
    pub id: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnresolvedIdent {
    pub id: u32,
}

#[derive(Debug, Copy, Clone, Functor, PartialEq, Eq)]
#[functor(LocTy as loc)]
pub enum Ident<LocTy> {
    Local(Ax<LocTy, LocalIdent>),
    Global(Ax<LocTy, UnresolvedIdent>),
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct FunDef<LocTy, IdentTy> {
    pub name: Ax<LocTy, syntax::Ident>,
    pub implementation: FunImpl<LocTy, IdentTy>,
    pub refs: Bimap<UnresolvedIdent, syntax::Ident>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct FunImpl<LocTy, IdentTy> {
    pub params: Ax<LocTy, Vec<Ax<LocTy, LocalIdent>>>,
    pub preds: Ax<LocTy, Vec<Expr<LocTy, IdentTy>>>,
    pub body: Ax<LocTy, Vec<FunStmt<LocTy, IdentTy>>>,
}

pub type LitNum = syntax::LitNum;
pub type LitStr = syntax::LitStr;

#[derive(Debug, Clone, Functor, PartialEq)]
#[functor(LocTy as loc, IdentTy as ident)]
pub enum Expr<LocTy, IdentTy> {
    LitNum(Ax<LocTy, syntax::LitNum>),
    LitStr(Ax<LocTy, syntax::LitStr>),
    Ident(IdentTy),
    FunCall(FunCall<LocTy, IdentTy>),
}

#[derive(Debug, Clone, Functor, PartialEq)]
#[functor(LocTy as loc, IdentTy as ident)]
pub struct FunCall<LocTy, IdentTy> {
    pub name: Ax<LocTy, UnresolvedIdent>,
    pub args: Ax<LocTy, Vec<Expr<LocTy, IdentTy>>>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
#[functor(LocTy as loc, IdentTy as ident)]
pub struct Return<LocTy, IdentTy>(pub Expr<LocTy, IdentTy>);

#[derive(Debug, Clone, Functor, PartialEq)]
#[functor(LocTy as loc, IdentTy as ident)]
pub enum FunStmt<LocTy, IdentTy> {
    Return(Ax<LocTy, Return<LocTy, IdentTy>>),
    Inv(Ax<LocTy, Expr<LocTy, IdentTy>>),
    LetDecl(Ax<LocTy, LetDecl<LocTy, IdentTy>>),
    While(Ax<LocTy, While<LocTy, IdentTy>>),
    Assignment(Ax<LocTy, Assignment<LocTy, IdentTy>>),
    If(Ax<LocTy, If<LocTy, IdentTy>>),
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct LetDecl<LocTy, IdentTy> {
    pub name: Ax<LocTy, LocalIdent>,
    pub value: Expr<LocTy, IdentTy>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct While<LocTy, IdentTy> {
    pub condition: Expr<LocTy, IdentTy>,
    pub body: Ax<LocTy, Vec<FunStmt<LocTy, IdentTy>>>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct Assignment<LocTy, IdentTy> {
    pub id: Ax<LocTy, LocalIdent>,
    pub value: Expr<LocTy, IdentTy>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub struct If<LocTy, IdentTy> {
    pub condition: Expr<LocTy, IdentTy>,
    pub then: Ax<LocTy, Vec<FunStmt<LocTy, IdentTy>>>,
    pub else_: Option<Ax<LocTy, Vec<FunStmt<LocTy, IdentTy>>>>,
}

#[derive(Debug, Clone, Functor, PartialEq)]
pub enum TopLevelStmt<LocTy, IdentTy> {
    FunDef(Ax<LocTy, FunDef<LocTy, IdentTy>>),
}

#[derive(Debug, Clone, Functor, PartialEq)]
#[functor(LocTy as loc, IdentTy as ident)]
pub struct Module<LocTy, IdentTy> {
    pub statements: Vec<TopLevelStmt<LocTy, IdentTy>>,
}
