use crate::util::ax::Ax;

// Identifier

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Ident(pub String);

// Literals

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LitNum(pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitStr(pub String);

// Expressions

#[derive(Debug, Clone)]
pub enum Expr<L> {
    LitNum(Ax<L, LitNum>),
    LitStr(Ax<L, LitStr>),
    Ident(Ax<L, Ident>),
    FunCall(FunCall<L>),
}

#[derive(Debug, Clone)]
pub struct FunDef<L> {
    pub name: Ax<L, Ident>,
    pub params: Ax<L, Vec<Ax<L, Ident>>>,
    pub preds: Ax<L, Vec<Expr<L>>>,
    pub body: Ax<L, Vec<FunStmt<L>>>,
}

#[derive(Debug, Clone)]
pub struct FunCall<L> {
    pub name: Ax<L, Ident>,
    pub args: Ax<L, Vec<Expr<L>>>,
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt<L> {
    FunDef(Ax<L, FunDef<L>>),
}

#[derive(Debug, Clone)]
pub enum FunStmt<L> {
    Return(Ax<L, Return<L>>),
    Inv(Ax<L, Inv<L>>),
    LetDecl(Ax<L, LetDecl<L>>),
    While(Ax<L, While<L>>),
    Assignment(Ax<L, Assignment<L>>),
    If(Ax<L, If<L>>),
}

#[derive(Debug, Clone)]
pub struct Return<L>(pub Expr<L>);

#[derive(Debug, Clone)]
pub struct Inv<L> {
    pub value: Ax<L, Expr<L>>,
}

#[derive(Debug, Clone)]
pub struct LetDecl<L> {
    pub name: Ax<L, Ident>,
    pub value: Expr<L>,
}

#[derive(Debug, Clone)]
pub struct While<L> {
    pub condition: Expr<L>,
    pub body: Ax<L, Vec<FunStmt<L>>>,
}

#[derive(Debug, Clone)]
pub struct Assignment<L> {
    pub name: Ax<L, Ident>,
    pub value: Expr<L>,
}

#[derive(Debug, Clone)]
pub struct If<L> {
    pub condition: Expr<L>,
    pub then: Ax<L, Vec<FunStmt<L>>>,
    pub else_: Option<Ax<L, Vec<FunStmt<L>>>>,
}

#[derive(Debug, Clone)]
pub struct Module<L> {
    pub statements: Ax<L, Vec<TopLevelStmt<L>>>,
}
