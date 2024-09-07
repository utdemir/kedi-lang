use crate::codegen::fragment;
use crate::parser::syntax;
use crate::util::loc::WithLoc;
use crate::util::pp;

#[derive(Debug)]
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
}

impl pp::SExpr for FunDecl {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::call(
            "fun",
            &[
                self.name.to_sexpr(),
                pp::SExprTerm::call("export", &[self.export]),
                self.implementation.to_sexpr(),
            ],
        )
    }
}

pub type FunImpl = fragment::FunImpl_<Instr>;

#[derive(Debug, Clone)]
pub struct Instr {
    pub instr: wasm_encoder::Instruction<'static>,
}

impl pp::SExpr for Instr {
    fn to_sexpr(&self) -> pp::SExprTerm {
        pp::SExprTerm::symbol(&format!("{:?}", self.instr))
    }
}
