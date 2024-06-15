use std::collections::HashMap;

use crate::{
    parser::syntax,
    pp::{SExpr, SExprTerm},
    renamer::plain,
};

#[derive(Clone, Copy, Debug)]
pub struct SingleUseIdentifier {
    pub id: u32,
}

impl SExpr for SingleUseIdentifier {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Atom(format!("${}", self.id))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Identifier {
    Plain(plain::Identifier),
    SingleUse(SingleUseIdentifier),
}

impl SExpr for Identifier {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Identifier::Plain(p) => p.to_sexpr(),
            Identifier::SingleUse(u) => u.to_sexpr(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<TopLevelStmt>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "module",
            self.statements.iter().map(|x| x.to_sexpr()).collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub enum TopLevelStmt {
    FunDecl(FunDecl),
}

impl SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            TopLevelStmt::FunDecl(f) => f.to_sexpr(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunDecl {
    pub name: syntax::Identifier,
    pub implementation: FunImpl,
    pub refs: HashMap<plain::GlobalIdentifier, syntax::Identifier>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun_decl",
            vec![self.name.to_sexpr(), self.implementation.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub struct FunImpl {
    pub parameters: Vec<plain::LocalIdentifier>,
    pub body: Vec<Statement>,
}

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::atom("fun_impl"),
            SExprTerm::call(
                "parameters",
                self.parameters.iter().map(|x| x.to_sexpr()).collect(),
            ),
            SExprTerm::List(self.body.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub target: Identifier,
    pub value: AssignmentValue,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "assignment",
            vec![self.target.to_sexpr(), self.value.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub enum AssignmentValue {
    Call(plain::GlobalIdentifier, Vec<Identifier>),
    Identifier(Identifier),
    LiteralNumber(i64),
}

impl SExpr for AssignmentValue {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            AssignmentValue::Call(f, a) => SExprTerm::List(vec![
                SExprTerm::Atom("call".to_string()),
                f.to_sexpr(),
                a.to_sexpr(),
            ]),
            AssignmentValue::Identifier(v) => v.to_sexpr(),
            AssignmentValue::LiteralNumber(n) => {
                SExprTerm::call("lit_number", vec![SExprTerm::number(*n)])
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    id: u32,
}

impl SExpr for Label {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("label", vec![SExprTerm::number(self.id)])
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Loop(Label),
    Assignment(Assignment),
    Branch(Label),
    Return(Identifier),
}

impl SExpr for Statement {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Statement::Loop(l) => SExprTerm::call("loop", vec![l.to_sexpr()]),
            Statement::Assignment(a) => a.to_sexpr(),
            Statement::Branch(l) => SExprTerm::call("branch", vec![l.to_sexpr()]),
            Statement::Return(i) => SExprTerm::call("return", vec![i.to_sexpr()]),
        }
    }
}
