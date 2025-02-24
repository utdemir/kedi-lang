
use bimap::BiHashMap;

use crate::parser::syntax;
use crate::renamer::plain;
use crate::util::loc::WithLoc;
use crate::util::pp;

#[derive(Debug, Clone)]
pub struct Module {
    pub statements: Vec<TopLevelStmt>,
}

impl Module {
    pub fn add(&self, other: &Module) -> Module {
        Module {
            statements: self
                .statements
                .iter()
                .chain(other.statements.iter())
                .cloned()
                .collect(),
        }
    }
}

impl pp::SExpr for Module {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(self.statements.iter().map(|stmt| stmt.to_sexpr()).collect())
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt {
    FunDecl(WithLoc<FunDecl>),
}

impl pp::SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> pp::SExprTerm {
        match self {
            TopLevelStmt::FunDecl(fun) => fun.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: WithLoc<syntax::Ident>,
    pub export: bool,
    pub implementation: WithLoc<FunImpl>,
    pub refs: BiHashMap<plain::GlobalIdent, syntax::Ident>,
}

impl pp::SExpr for FunDecl {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(vec![
            pp::SExprTerm::Symbol("fun".to_string()),
            self.name.to_sexpr(),
            pp::SExprTerm::call("export", &[self.export]),
            self.implementation.to_sexpr(),
            pp::SExprTerm::call(
                "refs",
                &self
                    .refs
                    .iter()
                    .map(|(k, v)| pp::SExprTerm::call("ref", &[k.to_sexpr(), v.to_sexpr()]))
                    .collect::<Vec<_>>(),
            ),
        ])
    }
}

pub type FunImpl = FunImpl_<Instr>;

#[derive(Debug, Clone)]
pub struct FunImpl_<InstrTy> {
    pub params: Vec<wasm_encoder::ValType>,
    pub body: Vec<InstrTy>,
}

impl<InstrTy> pp::SExpr for FunImpl_<InstrTy>
where
    InstrTy: pp::SExpr,
{
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::List(vec![
            pp::SExprTerm::call(
                "params",
                &self
                    .params
                    .iter()
                    .map(|x| pp::SExprTerm::symbol(&format!("{:?}", x)))
                    .collect::<Vec<_>>(),
            ),
            pp::SExprTerm::call("body", &self.body),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum Instr {
    Call(Call),
    Raw(wasm_encoder::Instruction<'static>),
}

impl pp::SExpr for Instr {
    fn to_sexpr(&self) -> pp::SExprTerm {
        match self {
            Instr::Call(call) => call.to_sexpr(),
            Instr::Raw(instr) => pp::SExprTerm::symbol(&format!("{:?}", instr)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub fun: plain::GlobalIdent,
    pub arity: usize,
}

impl pp::SExpr for Call {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::call(
            "call",
            &[
                pp::SExprTerm::number(self.fun.id),
                pp::SExprTerm::Symbol(format!("[{}]", self.arity)),
            ],
        )
    }
}
