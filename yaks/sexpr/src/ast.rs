use std::collections::HashMap;

pub trait SExpr {
    fn to_sexpr(&self) -> SExprTerm;
}

#[derive(Debug, Clone)]
pub enum SExprTerm {
    Symbol(String),
    String(String),
    Number(i64),
    List(Vec<SExprTerm>),
}

// Builder

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
    let mut list = vec![symbol(name)];
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

impl<T: SExpr> SExpr for Option<T> {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Some(x) => call("Some", &[x]),
            None => symbol("None"),
        }
    }
}

impl<T: SExpr, E: SExpr> SExpr for HashMap<T, E> {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(
            self.iter()
                .map(|(k, v)| list(&[k.to_sexpr(), v.to_sexpr()]))
                .collect(),
        )
    }
}

// Ptr

impl<T: SExpr + ?Sized> SExpr for &T {
    fn to_sexpr(&self) -> SExprTerm {
        (*self).to_sexpr()
    }
}

// number types

macro_rules! impl_sexpr_for_int {
    ($($t:ty),*) => {
        $(
            impl SExpr for $t {
                fn to_sexpr(&self) -> SExprTerm {
                    SExprTerm::Number(*self as i64)
                }
            }
        )*
    };
}

impl_sexpr_for_int!(i8, i16, i32, i64, u8, u16, u32, u64, usize);
