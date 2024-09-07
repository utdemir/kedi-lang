use std::collections::HashMap;

use crate::parser::syntax;
use crate::util::loc::{LVec, Located, SrcLoc, WithLoc};
use crate::util::pp::{SExpr, SExprTerm};

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub enum Ident {
    Local(LLocalIdent),
    Global(LGlobalIdent),
}

impl Located for Ident {
    fn location(&self) -> SrcLoc {
        match self {
            Ident::Local(x) => x.location(),
            Ident::Global(x) => x.location(),
        }
    }
}

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub struct LocalIdent {
    pub id: u32,
}

impl WithLoc<LocalIdent> {
    pub fn widen(self) -> WithLoc<Ident> {
        self.map(|i| Ident::Local(self.location.attach(*i)))
    }
}

pub type LLocalIdent = WithLoc<LocalIdent>;

impl SExpr for LocalIdent {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Symbol(format!("$${}", self.id))
    }
}

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub struct GlobalIdent {
    pub id: u32,
}

type LGlobalIdent = WithLoc<GlobalIdent>;

impl SExpr for GlobalIdent {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Symbol(format!("$$${}", self.id))
    }
}

impl SExpr for Ident {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Ident::Local(x) => x.to_sexpr(),
            Ident::Global(x) => x.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunDef {
    pub name: syntax::LIdent,
    pub implementation: LFunImpl,
    pub refs: HashMap<GlobalIdent, syntax::Ident>,
}

impl SExpr for FunDef {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("fun".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(
                vec![SExprTerm::Symbol("refs".to_string())]
                    .into_iter()
                    .chain(
                        self.refs
                            .iter()
                            .map(|(id, name)| SExprTerm::List(vec![id.to_sexpr(), name.to_sexpr()]))
                            .collect::<Vec<_>>()
                            .into_iter(),
                    )
                    .collect(),
            ),
            self.implementation.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct FunImpl {
    pub params: LVec<LLocalIdent>,
    pub preds: LVec<Expr>,
    pub body: LVec<FunStmt>,
}

pub type LFunImpl = WithLoc<FunImpl>;

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun_impl",
            &[
                SExprTerm::call("params", &self.params.value),
                SExprTerm::call("preds", &self.preds.value),
                SExprTerm::call("body", &self.body.value),
            ],
        )
    }
}

pub type LitNum = syntax::LitNum;
pub type LLitNum = syntax::LLitNum;

pub type LitStr = syntax::LitStr;
pub type LLitStr = syntax::LLitStr;

#[derive(Debug, Clone)]
pub enum Expr {
    LitNum(LLitNum),
    LitStr(LLitStr),
    Ident(Ident),
    FunCall(FunCall),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Expr::LitNum(x) => x.to_sexpr(),
            Expr::LitStr(x) => x.to_sexpr(),
            Expr::Ident(x) => x.to_sexpr(),
            Expr::FunCall(x) => x.to_sexpr(),
        }
    }
}

impl Located for Expr {
    fn location(&self) -> SrcLoc {
        match self {
            Expr::LitNum(x) => x.location(),
            Expr::LitStr(x) => x.location(),
            Expr::Ident(x) => x.location(),
            Expr::FunCall(x) => x.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunCall {
    pub name: LGlobalIdent,
    pub args: LVec<Expr>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("fun_call".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.args.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

impl Located for FunCall {
    fn location(&self) -> SrcLoc {
        SrcLoc::all_enclosing(&[self.name.location(), self.args.location()])
    }
}

#[derive(Debug, Clone)]
pub struct Return(pub Expr);
pub type LReturn = WithLoc<Return>;

impl SExpr for Return {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("return", [&self.0.to_sexpr()])
    }
}

#[derive(Debug, Clone)]
pub enum FunStmt {
    Return(LReturn),
    Inv(Expr),
    LetDecl(LLetDecl),
    While(LWhile),
    Assignment(LAssignment),
    If(LIf),
}

impl SExpr for FunStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            FunStmt::Return(expr) => SExprTerm::List(vec![
                SExprTerm::Symbol("return".to_string()),
                expr.to_sexpr(),
            ]),
            FunStmt::Inv(expr) => {
                SExprTerm::List(vec![SExprTerm::Symbol("inv".to_string()), expr.to_sexpr()])
            }
            FunStmt::LetDecl(decl) => decl.to_sexpr(),
            FunStmt::While(while_) => while_.to_sexpr(),
            FunStmt::Assignment(assignment) => assignment.to_sexpr(),
            FunStmt::If(if_) => if_.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: LLocalIdent,
    pub value: Expr,
}

pub type LLetDecl = WithLoc<LetDecl>;

impl SExpr for LetDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("let".to_string()),
            self.name.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: LVec<FunStmt>,
}

pub type LWhile = WithLoc<While>;

impl SExpr for While {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("while".to_string()),
            self.condition.to_sexpr(),
            SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: LLocalIdent,
    pub value: Expr,
}

pub type LAssignment = WithLoc<Assignment>;

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("assignment".to_string()),
            self.id.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then: LVec<FunStmt>,
    pub else_: Option<LVec<FunStmt>>,
}

pub type LIf = WithLoc<If>;

impl SExpr for If {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "if",
            &[
                self.condition.to_sexpr(),
                SExprTerm::call("then", &self.then.value),
                SExprTerm::call(
                    "else",
                    &self
                        .else_
                        .as_ref()
                        .map(|x| x.value.clone())
                        .unwrap_or_default(),
                ),
            ],
        )
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt {
    FunDef(WithLoc<FunDef>),
}

impl SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            TopLevelStmt::FunDef(fun_decl) => fun_decl.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub statements: Vec<TopLevelStmt>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(self.statements.iter().map(|x| x.to_sexpr()).collect())
    }
}
