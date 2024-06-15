use crate::pp::{SExpr, SExprTerm};

#[derive(Debug, Clone)]
pub struct Located<T> {
    pub value: T,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: u32,
    pub column: u32,
    pub length: u32,
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
    pub name: Identifier,
    pub parameters: Vec<FunParam>,
    pub return_predicate: Option<Expr>,
    pub body: Vec<FunStatement>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("fun".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.parameters.iter().map(|x| x.to_sexpr()).collect()),
            self.return_predicate.to_sexpr(),
            SExprTerm::List(self.body.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct FunParam {
    pub name: Identifier,
    pub predicate: Expr,
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
    LitNumber(i64),
    LitString(String),
    ValueIdentifier(Identifier),
    FunCall(FunCall),
    Op(Box<Expr>, Identifier, Box<Expr>),
}

impl SExpr for Expr {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            Expr::LitNumber(ref x) => SExprTerm::Atom(x.to_string()),
            Expr::LitString(ref x) => SExprTerm::Atom(x.clone()),
            Expr::ValueIdentifier(ref x) => x.to_sexpr(),
            Expr::FunCall(ref x) => x.to_sexpr(),
            Expr::Op(ref a, ref op, ref b) => SExprTerm::List(vec![
                SExprTerm::Atom(op.name.clone()),
                a.to_sexpr(),
                b.to_sexpr(),
            ]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunCall {
    pub name: Identifier,
    pub arguments: Vec<Expr>,
}

impl SExpr for FunCall {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("call".to_string()),
            self.name.to_sexpr(),
            SExprTerm::List(self.arguments.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelStatement {
    FunDecl(FunDecl),
}

impl SExpr for TopLevelStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            TopLevelStatement::FunDecl(ref x) => x.to_sexpr(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunStatement {
    Return(Expr),
    Inv(Expr),
    LetDecl(Identifier, Expr),
    While(Expr, Vec<FunStatement>),
    Assignment(Identifier, Expr),
}

impl SExpr for FunStatement {
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            FunStatement::Return(ref x) => {
                SExprTerm::List(vec![SExprTerm::Atom("return".to_string()), x.to_sexpr()])
            }
            FunStatement::Inv(ref x) => {
                SExprTerm::List(vec![SExprTerm::Atom("inv".to_string()), x.to_sexpr()])
            }
            FunStatement::LetDecl(ref name, ref value) => SExprTerm::List(vec![
                SExprTerm::Atom("let".to_string()),
                name.to_sexpr(),
                value.to_sexpr(),
            ]),
            FunStatement::While(ref condition, ref body) => SExprTerm::List(vec![
                SExprTerm::Atom("while".to_string()),
                condition.to_sexpr(),
                SExprTerm::List(body.iter().map(|x| x.to_sexpr()).collect()),
            ]),
            FunStatement::Assignment(ref name, ref value) => SExprTerm::List(vec![
                SExprTerm::Atom("assign".to_string()),
                name.to_sexpr(),
                value.to_sexpr(),
            ]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub statements: Vec<TopLevelStatement>,
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
