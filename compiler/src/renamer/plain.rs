use std::collections::HashMap;

use crate::parser::syntax;
use crate::pp::{SExpr, SExprTerm};

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub enum Identifier {
    Local(LocalIdentifier),
    Global(GlobalIdentifier),
}

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub struct LocalIdentifier {
    pub id: u32,
}

impl LocalIdentifier {
    pub fn widen(self) -> Identifier {
        Identifier::Local(self)
    }
}

impl SExpr for LocalIdentifier {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Atom(format!("$${}", self.id))
    }
}

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
pub struct GlobalIdentifier {
    pub id: u32,
}

impl GlobalIdentifier {
    pub fn widen(self) -> Identifier {
        Identifier::Global(self)
    }
}

impl SExpr for GlobalIdentifier {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Atom(format!("$$${}", self.id))
    }
}

impl SExpr for Identifier {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Identifier::Local(x) => x.to_sexpr(),
            Identifier::Global(x) => x.to_sexpr(),
        }
    }
}

#[derive(Debug)]
pub struct FunDecl {
    pub name: syntax::Identifier,
    pub implementation: FunImpl,
    pub refs: HashMap<GlobalIdentifier, syntax::Identifier>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("fun".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(
                vec![SExprTerm::Atom("refs".to_string())]
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

#[derive(Debug)]
pub struct FunImpl {
    pub parameters: Vec<FunParam>,
    pub return_predicate: Option<Expr>,
    pub body: Vec<FunStatement>,
}

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::List(self.parameters.iter().map(|x| x.to_sexpr()).collect()),
            self.return_predicate.to_sexpr(),
            SExprTerm::List(self.body.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug)]
pub struct FunParam {
    pub name: LocalIdentifier,
    pub predicate: Option<Expr>,
}

impl SExpr for FunParam {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("param".to_owned()),
            self.name.to_sexpr(),
            self.predicate.to_sexpr(),
        ])
    }
}

#[derive(Debug)]
pub enum Expr {
    LitNumber(i64),
    LitString(String),
    ValueIdentifier(Identifier),
    FunCall(FunCall),
    Op(Box<Expr>, Identifier, Box<Expr>),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Expr::LitNumber(x) => SExprTerm::Atom(x.to_string()),
            Expr::LitString(x) => SExprTerm::Atom(x.clone()),
            Expr::ValueIdentifier(x) => x.to_sexpr(),
            Expr::FunCall(x) => x.to_sexpr(),
            Expr::Op(lhs, op, rhs) => SExprTerm::List(vec![
                SExprTerm::Atom("op".to_string()),
                lhs.to_sexpr(),
                op.to_sexpr(),
                rhs.to_sexpr(),
            ]),
        }
    }
}

#[derive(Debug)]
pub struct FunCall {
    pub name: GlobalIdentifier,
    pub arguments: Vec<Expr>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("fun_call".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.arguments.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug)]
pub enum FunStatement {
    Return(Expr),
    Inv(Expr),
    LetDecl(LocalIdentifier, Expr),
    While(Expr, Vec<FunStatement>),
    Assignment(LocalIdentifier, Expr),
}

impl SExpr for FunStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            FunStatement::Return(expr) => {
                SExprTerm::List(vec![SExprTerm::Atom("return".to_string()), expr.to_sexpr()])
            }
            FunStatement::Inv(expr) => {
                SExprTerm::List(vec![SExprTerm::Atom("inv".to_string()), expr.to_sexpr()])
            }
            FunStatement::LetDecl(id, expr) => SExprTerm::List(vec![
                SExprTerm::Atom("let".to_string()),
                id.to_sexpr(),
                expr.to_sexpr(),
            ]),
            FunStatement::While(expr, body) => SExprTerm::List(vec![
                SExprTerm::Atom("while".to_string()),
                expr.to_sexpr(),
                SExprTerm::List(body.iter().map(|x| x.to_sexpr()).collect()),
            ]),
            FunStatement::Assignment(id, expr) => SExprTerm::List(vec![
                SExprTerm::Atom("assignment".to_string()),
                id.to_sexpr(),
                expr.to_sexpr(),
            ]),
        }
    }
}

#[derive(Debug)]
pub enum TopLevelStatement {
    FunDecl(FunDecl),
}

impl SExpr for TopLevelStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            TopLevelStatement::FunDecl(fun_decl) => fun_decl.to_sexpr(),
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub statements: Vec<TopLevelStatement>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(self.statements.iter().map(|x| x.to_sexpr()).collect())
    }
}
