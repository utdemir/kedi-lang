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

#[derive(Debug, Copy, Clone)]
pub enum Ident<L> {
    Local(Ax<L, LocalIdent>),
    Global(Ax<L, UnresolvedIdent>),
}

#[derive(Debug, Clone)]
pub struct FunDef<L> {
    pub name: Ax<L, syntax::Ident>,
    pub implementation: FunImpl<L>,
    pub refs: Bimap<UnresolvedIdent, syntax::Ident>,
}

#[derive(Debug, Clone)]
pub struct FunImpl<L> {
    pub params: Ax<L, Vec<Ax<L, LocalIdent>>>,
    pub preds: Ax<L, Vec<Expr<L>>>,
    pub body: Ax<L, Vec<FunStmt<L>>>,
}

pub type LitNum = syntax::LitNum;
pub type LitStr = syntax::LitStr;

#[derive(Debug, Clone)]
pub enum Expr<L> {
    LitNum(Ax<L, syntax::LitNum>),
    LitStr(Ax<L, syntax::LitStr>),
    Ident(Ident<L>),
    FunCall(FunCall<L>),
}

#[derive(Debug, Clone)]
pub struct FunCall<L> {
    pub name: Ax<L, UnresolvedIdent>,
    pub args: Ax<L, Vec<Expr<L>>>,
}

#[derive(Debug, Clone)]
pub struct Return<L>(pub Expr<L>);

#[derive(Debug, Clone)]
pub enum FunStmt<L> {
    Return(Ax<L, Return<L>>),
    Inv(Ax<L, Expr<L>>),
    LetDecl(Ax<L, LetDecl<L>>),
    While(Ax<L, While<L>>),
    Assignment(Ax<L, Assignment<L>>),
    If(Ax<L, If<L>>),
}

#[derive(Debug, Clone)]
pub struct LetDecl<L> {
    pub name: Ax<L, LocalIdent>,
    pub value: Expr<L>,
}

#[derive(Debug, Clone)]
pub struct While<L> {
    pub condition: Expr<L>,
    pub body: Ax<L, Vec<FunStmt<L>>>,
}

#[derive(Debug, Clone)]
pub struct Assignment<L> {
    pub id: Ax<L, LocalIdent>,
    pub value: Expr<L>,
}

#[derive(Debug, Clone)]
pub struct If<L> {
    pub condition: Expr<L>,
    pub then: Ax<L, Vec<FunStmt<L>>>,
    pub else_: Option<Ax<L, Vec<FunStmt<L>>>>,
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt<L> {
    FunDef(Ax<L, FunDef<L>>),
}

#[derive(Debug, Clone)]
pub struct Module<L> {
    pub statements: Vec<TopLevelStmt<L>>,
}
