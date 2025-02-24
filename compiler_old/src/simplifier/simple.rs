use bimap::BiHashMap;

use crate::renamer::plain::LLocalIdent;
use crate::util::bimap::Bimap;
use crate::util::loc::{Located, SrcLoc, Tagged, WithLoc, WithTag};

use crate::{parser::syntax, renamer::plain, util::loc};

use sexpr_derive::SExpr;

#[derive(Clone, Debug, SExpr)]
pub struct Module {
    pub statements: Vec<TopLevelStmt>,
}

#[derive(Clone, Debug, SExpr)]
pub enum TopLevelStmt {
    FunDecl(WithLoc<FunDecl>),
}

impl Located for TopLevelStmt {
    fn location(&self) -> loc::SrcLoc {
        match self {
            TopLevelStmt::FunDecl(f) => f.location(),
        }
    }
}

#[derive(Clone, Debug, SExpr)]
pub struct FunDecl {
    pub name: syntax::LIdent,
    pub implementation: WithLoc<FunImpl>,
    pub tag_map: loc::TagMap,
    pub refs: Bimap<plain::GlobalIdent, syntax::Ident>,
}

#[derive(Clone, Debug, SExpr)]
pub struct FunImpl {
    pub parameters: WithTag<Vec<WithTag<plain::LocalIdent>>>,
    pub body: WithTag<Vec<FunStmt>>,
}

pub type LFunImpl = WithLoc<FunImpl>;

pub type LitNum = syntax::LitNum;
pub type TLitNum = WithTag<LitNum>;

pub type LitStr = syntax::LitStr;
pub type TLitStr = WithTag<syntax::LitStr>;

#[derive(Clone, Debug, SExpr)]
pub enum Expr {
    LitNum(TLitNum),
    LitStr(TLitStr),
    Ident(WithTag<plain::LocalIdent>),
    FunCall(FunCall),
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

#[derive(Clone, Debug, SExpr)]
pub struct FunCall {
    pub name: WithTag<plain::GlobalIdent>,
    pub args: WithTag<Vec<Expr>>,
}

impl Located for FunCall {
    fn location(&self) -> SrcLoc {
        SrcLoc::all_enclosing(&[self.name.location(), self.args.location()])
    }
}

#[derive(Clone, Debug, SExpr)]
pub struct Return(pub Expr);

pub type LReturn = WithLoc<Return>;

#[derive(Clone, Debug, SExpr)]
pub enum FunStmt {
    Loop(WithTag<Loop>),
    Assignment(WithTag<Assignment>),
    Break(),
    Return(Expr),
    If(If),
    Nop,
}

#[derive(Clone, Debug, SExpr)]
pub struct LetDecl {
    pub name: LLocalIdent,
    pub value: Expr,
}

pub type LLetDecl = WithLoc<LetDecl>;

#[derive(Clone, Debug, SExpr)]
pub struct While {
    pub condition: Expr,
    pub body: WithTag<Vec<FunStmt>>,
}

pub type LWhile = WithLoc<While>;

#[derive(Clone, Debug, SExpr)]
pub struct Assignment {
    pub target: WithTag<plain::LocalIdent>,
    pub value: AssignmentValue,
}

pub type LAssignment = WithLoc<Assignment>;

#[derive(Clone, Debug, SExpr)]
pub struct If {
    pub condition: Expr,
    pub then: WithTag<Vec<FunStmt>>,
    pub else_: Option<WithTag<Vec<FunStmt>>>,
}

pub type LIf = WithLoc<If>;

#[derive(Clone, Debug, SExpr)]
pub struct Label {
    pub id: u32,
}

#[derive(Clone, Debug, SExpr)]
pub struct Call {
    pub fun_name: WithTag<plain::GlobalIdent>,
    pub arguments: WithTag<Vec<Expr>>,
}

#[derive(Clone, Debug, SExpr)]
pub enum AssignmentValue {
    Call(WithTag<Call>),
    Ident(WithTag<plain::LocalIdent>),
    LitNum(WithTag<plain::LitNum>),
}

impl Tagged for AssignmentValue {
    fn tag(&self) -> loc::Tag {
        match self {
            AssignmentValue::Call(c) => c.tag(),
            AssignmentValue::Ident(i) => i.tag(),
            AssignmentValue::LitNum(n) => n.tag(),
        }
    }
}

#[derive(Clone, Debug, SExpr)]
pub struct Loop {
    pub body: WithTag<Vec<FunStmt>>,
}
