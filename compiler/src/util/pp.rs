use pretty::RcDoc;

// Trait

pub trait SExpr {
    fn to_sexpr(&self) -> SExprTerm;
}

// Terms

#[derive(Debug, Clone)]
pub enum SExprTerm {
    Symbol(String),
    String(String),
    Number(i64),
    List(Vec<SExprTerm>),
}

impl SExprTerm {
    pub fn symbol(x: &str) -> SExprTerm {
        SExprTerm::Symbol(x.to_string())
    }

    pub fn string(x: &str) -> SExprTerm {
        SExprTerm::String(x.to_string())
    }

    pub fn number<T: Into<i64>>(x: T) -> SExprTerm {
        SExprTerm::Number(x.into())
    }

    pub fn call<'t, I, T>(name: &str, args: I) -> SExprTerm
    where
        I: IntoIterator<Item = &'t T>,
        T: SExpr + 't,
    {
        let mut list = vec![SExprTerm::Symbol(name.to_string())];
        for arg in args.into_iter() {
            list.push(arg.to_sexpr());
        }
        SExprTerm::List(list)
    }

    pub fn list<I>(args: I) -> SExprTerm
    where
        I: IntoIterator,
        I::Item: SExpr,
    {
        SExprTerm::List(args.into_iter().map(|arg| arg.to_sexpr()).collect())
    }

    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            SExprTerm::Symbol(ref x) => RcDoc::as_string(x),
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

    pub fn to_pretty_string(&self) -> String {
        self.to_doc().pretty(40).to_string()
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

impl SExpr for () {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![])
    }
}

impl SExpr for bool {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Symbol(if *self { "true" } else { "false" }.to_string())
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

impl SExpr for &str {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::String(self.to_string())
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
            None => SExprTerm::Symbol("None".to_string()),
        }
    }
}
