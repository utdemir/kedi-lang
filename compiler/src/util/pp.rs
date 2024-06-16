use pretty::RcDoc;

// Trait

pub trait SExpr {
    fn to_sexpr(&self) -> SExprTerm;
}

// Terms

#[derive(Debug, Clone)]
pub enum SExprTerm {
    Atom(String),
    String(String),
    Number(i64),
    List(Vec<SExprTerm>),
}

impl SExprTerm {
    pub fn atom(x: &str) -> SExprTerm {
        SExprTerm::Atom(x.to_string())
    }

    pub fn string(x: &str) -> SExprTerm {
        SExprTerm::String(x.to_string())
    }

    pub fn number<T: Into<i64>>(x: T) -> SExprTerm {
        SExprTerm::Number(x.into())
    }

    pub fn call(name: &str, args: Vec<SExprTerm>) -> SExprTerm {
        let mut list = vec![SExprTerm::Atom(name.to_string())];
        list.extend(args);
        SExprTerm::List(list)
    }

    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            SExprTerm::Atom(ref x) => RcDoc::as_string(x),
            SExprTerm::String(ref x) => RcDoc::text(format!("{:?}", x)),
            SExprTerm::Number(ref x) => RcDoc::as_string(x),
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

// Instances

impl SExpr for SExprTerm {
    fn to_sexpr(&self) -> SExprTerm {
        self.clone()
    }
}

impl<T: SExpr> SExpr for Vec<T> {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(self.iter().map(SExpr::to_sexpr).collect())
    }
}

impl SExpr for i32 {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::number(*self)
    }
}

impl SExpr for u32 {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::number(*self)
    }
}

impl SExpr for String {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::String(self.clone())
    }
}

// Pretty printing

pub struct Options {
    pub width: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options { width: 80 }
    }
}

pub fn print<T: SExpr>(sexpr: &T, options: &Options) -> String {
    sexpr.to_sexpr().to_doc().pretty(options.width).to_string()
}

// Instances

impl<T> SExpr for Option<T>
where
    T: SExpr,
{
    fn to_sexpr(&self) -> SExprTerm {
        match *self {
            Some(ref x) => SExprTerm::List(vec![x.to_sexpr()]),
            None => SExprTerm::Atom("None".to_string()),
        }
    }
}
