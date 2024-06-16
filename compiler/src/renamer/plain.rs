use std::collections::HashMap;

use crate::parser::syntax;
use crate::util::loc::Located;
use crate::util::pp::{SExpr, SExprTerm};

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
    pub name: Located<syntax::Identifier>,
    pub implementation: Located<FunImpl>,
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
    pub parameters: Located<Vec<Located<FunParam>>>,
    pub return_predicate: Option<Located<Expr>>,
    pub body: Located<Vec<Located<FunStatement>>>,
}

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::List(self.parameters.value.iter().map(|x| x.to_sexpr()).collect()),
            self.return_predicate.to_sexpr(),
            SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
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
    LitNumber(Located<i32>),
    LitString(Located<String>),
    ValueIdentifier(Located<Identifier>),
    FunCall(Located<FunCall>),
    Op(Box<Located<Expr>>, Located<Identifier>, Box<Located<Expr>>),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Expr::LitNumber(x) => SExprTerm::Atom(x.value.to_string()),
            Expr::LitString(x) => SExprTerm::Atom(x.value.clone()),
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
    pub name: Located<GlobalIdentifier>,
    pub arguments: Located<Vec<Located<Expr>>>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("fun_call".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.arguments.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug)]
pub enum FunStatement {
    Return(Located<Expr>),
    Inv(Located<Expr>),
    LetDecl(Located<LetDecl>),
    While(Located<While>),
    Assignment(Located<Assignment>),
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
            FunStatement::LetDecl(decl) => decl.to_sexpr(),
            FunStatement::While(while_) => while_.to_sexpr(),
            FunStatement::Assignment(assignment) => assignment.to_sexpr(),
        }
    }
}

#[derive(Debug)]
pub struct LetDecl {
    pub name: Located<LocalIdentifier>,
    pub value: Located<Expr>,
}

impl SExpr for LetDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("let".to_string()),
            self.name.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug)]
pub struct While {
    pub condition: Located<Expr>,
    pub body: Located<Vec<Located<FunStatement>>>,
}

impl SExpr for While {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("while".to_string()),
            self.condition.to_sexpr(),
            SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug)]
pub struct Assignment {
    id: Located<LocalIdentifier>,
    value: Located<Expr>,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("assignment".to_string()),
            self.id.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug)]
pub enum TopLevelStatement {
    FunDecl(Located<FunDecl>),
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
    pub statements: Vec<Located<TopLevelStatement>>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(self.statements.iter().map(|x| x.to_sexpr()).collect())
    }
}
