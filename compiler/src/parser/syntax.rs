use crate::util::loc::{Located, SrcLoc};
use crate::util::pp::{SExpr, SExprTerm};

impl SExpr for SrcLoc {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            SrcLoc::Known(ref x) => x.to_sexpr(),
            SrcLoc::Unknown => SExprTerm::Atom("unknown_loc".to_string()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    pub name: String,
}

impl SExpr for Identifier {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Atom('"'.to_string() + &self.name + "\"")
    }
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: Located<Identifier>,
    pub parameters: Located<Vec<Located<FunParam>>>,
    pub return_predicate: Option<Located<Expr>>,
    pub body: Located<Vec<Located<FunStatement>>>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("fun".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.parameters.value.iter().map(|x| x.to_sexpr()).collect()),
            self.return_predicate.to_sexpr(),
            SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct FunParam {
    pub name: Located<Identifier>,
    pub predicate: Located<Expr>,
}

impl SExpr for FunParam {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("param".to_string()),
            self.name.to_sexpr(),
            self.predicate.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    LitNumber(Located<i32>),
    LitString(Located<String>),
    ValueIdentifier(Located<Identifier>),
    FunCall(Located<FunCall>),
    Op(Box<Located<Expr>>, Located<Identifier>, Box<Located<Expr>>),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            Expr::LitNumber(ref x) => SExprTerm::Atom(x.value.to_string()),
            Expr::LitString(ref x) => SExprTerm::Atom(x.value.clone()),
            Expr::ValueIdentifier(ref x) => x.to_sexpr(),
            Expr::FunCall(ref x) => x.to_sexpr(),
            Expr::Op(ref a, ref op, ref b) => SExprTerm::List(vec![
                SExprTerm::Atom(op.value.name.clone()),
                a.to_sexpr(),
                b.to_sexpr(),
            ]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunCall {
    pub name: Located<Identifier>,
    pub arguments: Located<Vec<Located<Expr>>>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("call".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.arguments.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelStatement {
    FunDecl(Located<FunDecl>),
}

impl SExpr for TopLevelStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            TopLevelStatement::FunDecl(ref x) => x.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InlineWasm {
    pub input_stack: Located<Vec<Located<Identifier>>>,
    pub output_stack: Located<Vec<Located<Identifier>>>,
    pub wasm: Located<wast::core::Instruction<'static>>,
}

impl SExpr for InlineWasm {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("inline_wasm".to_string()),
            SExprTerm::List(
                self.input_stack
                    .value
                    .iter()
                    .map(|x| x.to_sexpr())
                    .collect(),
            ),
            SExprTerm::List(
                self.output_stack
                    .value
                    .iter()
                    .map(|x| x.to_sexpr())
                    .collect(),
            ),
            SExprTerm::Atom(format!("{:?}", self.wasm.value)),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum FunStatement {
    Return(Located<Return>),
    Inv(Located<Inv>),
    LetDecl(Located<LetDecl>),
    While(Located<While>),
    Assignment(Located<Assignment>),
    InlineWasm(Located<InlineWasm>),
}

impl SExpr for FunStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            FunStatement::Return(ref x) => x.to_sexpr(),
            FunStatement::Inv(ref x) => x.to_sexpr(),
            FunStatement::LetDecl(ref x) => x.to_sexpr(),
            FunStatement::While(ref x) => x.to_sexpr(),
            FunStatement::Assignment(ref x) => x.to_sexpr(),
            FunStatement::InlineWasm(ref x) => x.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Located<Expr>,
}

impl SExpr for Return {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("return".to_string()),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct Inv {
    pub value: Located<Expr>,
}

impl SExpr for Inv {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("inv".to_string()),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: Located<Identifier>,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: Located<Identifier>,
    pub value: Located<Expr>,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("assign".to_string()),
            self.name.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub statements: Vec<Located<TopLevelStatement>>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        let inner = vec![SExprTerm::Atom("module".to_string())]
            .into_iter()
            .chain(self.statements.iter().map(|x| x.to_sexpr()))
            .collect();

        SExprTerm::List(inner)
    }
}
