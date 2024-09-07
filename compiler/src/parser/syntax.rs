use crate::util::loc::{LVec, Located, SrcLoc, WithLoc};
use crate::util::pp::{SExpr, SExprTerm};

// Identifier

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Ident(pub String);

impl SExpr for Ident {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::symbol(self.0.as_str())
    }
}

pub type LIdent = WithLoc<Ident>;

// Literals

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LitNum(pub i32);

impl SExpr for LitNum {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::number(self.0)
    }
}

pub type LLitNum = WithLoc<LitNum>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitStr(pub String);

impl SExpr for LitStr {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::string(self.0.as_str())
    }
}

pub type LLitStr = WithLoc<LitStr>;

// Expressions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    LitNum(LLitNum),
    LitStr(LLitStr),
    Ident(LIdent),
    FunCall(FunCall),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            Expr::LitNum(ref x) => x.to_sexpr(),
            Expr::LitStr(ref x) => x.to_sexpr(),
            Expr::Ident(ref x) => x.to_sexpr(),
            Expr::FunCall(ref x) => x.to_sexpr(),
        }
    }
}

impl Located for Expr {
    fn location(&self) -> SrcLoc {
        match *self {
            Expr::LitNum(ref x) => x.location(),
            Expr::LitStr(ref x) => x.location(),
            Expr::Ident(ref x) => x.location(),
            Expr::FunCall(ref x) => x.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunDef {
    pub name: LIdent,
    pub params: LVec<LIdent>,
    pub preds: LVec<Expr>,
    pub body: LVec<FunStmt>,
}

impl SExpr for FunDef {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun",
            &[
                self.name.to_sexpr(),
                SExprTerm::call("params", &self.params.value),
                SExprTerm::call("preds", &self.preds.value),
                SExprTerm::call("body", &self.body.value),
            ],
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunCall {
    pub name: LIdent,
    pub args: LVec<Expr>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "call",
            &[
                self.name.to_sexpr(),
                SExprTerm::call("args", &self.args.value),
            ],
        )
    }
}

impl Located for FunCall {
    fn location(&self) -> SrcLoc {
        SrcLoc::all_enclosing(&[self.name.location(), self.args.location()])
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelStmt {
    FunDef(WithLoc<FunDef>),
}

impl SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            TopLevelStmt::FunDef(ref x) => x.to_sexpr(),
        }
    }
}

impl Located for TopLevelStmt {
    fn location(&self) -> SrcLoc {
        match *self {
            TopLevelStmt::FunDef(ref x) => x.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunStmt {
    Return(WithLoc<Return>),
    Inv(WithLoc<Inv>),
    LetDecl(WithLoc<LetDecl>),
    While(WithLoc<While>),
    Assignment(WithLoc<Assignment>),
    If(WithLoc<If>),
}

impl SExpr for FunStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            FunStmt::Return(ref x) => x.to_sexpr(),
            FunStmt::Inv(ref x) => x.to_sexpr(),
            FunStmt::LetDecl(ref x) => x.to_sexpr(),
            FunStmt::While(ref x) => x.to_sexpr(),
            FunStmt::Assignment(ref x) => x.to_sexpr(),
            FunStmt::If(ref x) => x.to_sexpr(),
        }
    }
}

impl Located for FunStmt {
    fn location(&self) -> SrcLoc {
        match *self {
            FunStmt::Return(ref x) => x.location(),
            FunStmt::Inv(ref x) => x.location(),
            FunStmt::LetDecl(ref x) => x.location(),
            FunStmt::While(ref x) => x.location(),
            FunStmt::Assignment(ref x) => x.location(),
            FunStmt::If(ref x) => x.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Return(pub Expr);

impl SExpr for Return {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("return".to_string()),
            self.0.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct Inv {
    pub value: WithLoc<Expr>,
}

impl SExpr for Inv {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("inv".to_string()),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: WithLoc<Ident>,
    pub value: Expr,
}

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
    pub body: WithLoc<Vec<FunStmt>>,
}

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
    pub name: WithLoc<Ident>,
    pub value: Expr,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Symbol("assign".to_string()),
            self.name.to_sexpr(),
            self.value.to_sexpr(),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then: WithLoc<Vec<FunStmt>>,
    pub else_: Option<WithLoc<Vec<FunStmt>>>,
}

impl SExpr for If {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "if",
            &[
                self.condition.to_sexpr(),
                SExprTerm::call("then", &self.then.value),
                SExprTerm::call("else", &self.else_),
            ],
        )
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub statements: LVec<TopLevelStmt>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("module", &self.statements.value)
    }
}
