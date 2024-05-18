use std::collections::HashMap;

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub struct RenamedIdentifier {
    pub uid: u32,
}

#[derive(Debug)]
pub struct FunDecl {
    pub name: RenamedIdentifier,
    pub parameters: Vec<FunParam>,
    pub return_predicate: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunParam {
    pub name: RenamedIdentifier,
    pub predicate: Expr,
}

#[derive(Debug)]
pub enum Expr {
    LitNumber(u64),
    LitString(String),
    ValueIdentifier(RenamedIdentifier),
    FunCall(FunCall),
    Op(Box<Expr>, RenamedIdentifier, Box<Expr>),
}

#[derive(Debug)]
pub struct FunCall {
    pub name: RenamedIdentifier,
    pub arguments: Vec<Expr>,
}

#[derive(Debug)]
pub enum Statement {
    FunDecl(FunDecl),
    Return(Expr),
    Inv(Expr),
    LetDecl(RenamedIdentifier, Expr),
    While(Expr, Vec<Statement>),
    Assignment(RenamedIdentifier, Expr),
}

#[derive(Debug)]
pub enum TopLevelStatement {
    FunDecl(FunDecl),
}

#[derive(Debug)]
pub struct Module {
    pub statements: Vec<TopLevelStatement>,
}
