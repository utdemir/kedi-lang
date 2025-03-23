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
pub enum Expr<LocTy> {
    LitNum(Ax<LocTy, LitNum>),
    LitStr(Ax<LocTy, LitStr>),
    Ident(Ax<LocTy, Ident>),
    FunCall(FunCall<LocTy>),
}

#[derive(Debug, Clone)]
pub struct FunDef<LocTy> {
    pub name: Ax<LocTy, Ident>,
    pub params: Ax<LocTy, Vec<Ax<LocTy, Ident>>>,
    pub preds: Ax<LocTy, Vec<Expr<LocTy>>>,
    pub body: Ax<LocTy, Vec<FunStmt<LocTy>>>,
}

#[derive(Debug, Clone)]
pub struct FunCall<LocTy> {
    pub name: Ax<LocTy, Ident>,
    pub args: Ax<LocTy, Vec<Expr<LocTy>>>,
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt<LocTy> {
    FunDef(Ax<LocTy, FunDef<LocTy>>),
}

#[derive(Debug, Clone)]
pub enum FunStmt<LocTy> {
    Return(Ax<LocTy, Return<LocTy>>),
    Inv(Ax<LocTy, Inv<LocTy>>),
    LetDecl(Ax<LocTy, LetDecl<LocTy>>),
    While(Ax<LocTy, While<LocTy>>),
    Assignment(Ax<LocTy, Assignment<LocTy>>),
    If(Ax<LocTy, If<LocTy>>),
}

#[derive(Debug, Clone)]
pub struct Return<LocTy>(pub Expr<LocTy>);

#[derive(Debug, Clone)]
pub struct Inv<LocTy> {
    pub value: Ax<LocTy, Expr<LocTy>>,
}

#[derive(Debug, Clone)]
pub struct LetDecl<LocTy> {
    pub name: Ax<LocTy, Ident>,
    pub value: Expr<LocTy>,
}

#[derive(Debug, Clone)]
pub struct While<LocTy> {
    pub condition: Expr<LocTy>,
    pub body: Ax<LocTy, Vec<FunStmt<LocTy>>>,
}

#[derive(Debug, Clone)]
pub struct Assignment<LocTy> {
    pub name: Ax<LocTy, Ident>,
    pub value: Expr<LocTy>,
}

#[derive(Debug, Clone)]
pub struct If<LocTy> {
    pub condition: Expr<LocTy>,
    pub then: Ax<LocTy, Vec<FunStmt<LocTy>>>,
    pub else_: Option<Ax<LocTy, Vec<FunStmt<LocTy>>>>,
}

#[derive(Debug, Clone)]
pub struct Module<LocTy> {
    pub statements: Ax<LocTy, Vec<TopLevelStmt<LocTy>>>,
}
