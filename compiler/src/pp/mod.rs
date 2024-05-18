use pretty::RcDoc;

// Trait

pub trait SExpr {
    fn to_sexpr(&self) -> SExprTerm;
}

// Terms

pub enum SExprTerm {
    Atom(String),
    List(Vec<SExprTerm>),
}

impl SExprTerm {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            SExprTerm::Atom(ref x) => RcDoc::as_string(x),
            SExprTerm::List(ref xs) => RcDoc::text("(")
                .append(
                    RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), RcDoc::line())
                        .nest(1)
                        .group(),
                )
                .append(RcDoc::text(")")),
        }
    }
}
