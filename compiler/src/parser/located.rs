use crate::pp::{SExpr, SExprTerm};

#[derive(Debug, Clone, Copy)]
pub struct Located<T> {
    pub value: T,
    pub location: SrcLoc,
}

impl<T: SExpr> SExpr for Located<T> {
    fn to_sexpr(&self) -> SExprTerm {
        self.value.to_sexpr()
    }
}

impl<T> Located<T> {
    pub fn known(value: T, location: Span) -> Located<T> {
        Located {
            value,
            location: SrcLoc::Known(location),
        }
    }

    pub fn unknown(value: T) -> Located<T> {
        Located {
            value,
            location: SrcLoc::Unknown,
        }
    }

    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Located<U> {
        Located {
            value: f(&self.value),
            location: self.location,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub offset: usize,
}

impl SExpr for Pos {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("pos", vec![SExprTerm::Number(self.offset as i64)])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Pos,
    pub length: usize,
}

impl SExpr for Span {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "span",
            vec![self.start.to_sexpr(), SExprTerm::Number(self.length as i64)],
        )
    }
}

impl Span {
    // Returns the span that encloses both `self` and `other`.
    pub fn enclosing(fst: &Span, snd: &Span) -> Span {
        let self_start = fst.start.offset;
        let self_end = fst.start.offset + fst.length;

        let other_start = snd.start.offset;
        let other_end = snd.start.offset + snd.length;

        let start = self_start.min(other_start);
        let end = self_end.max(other_end);

        Span {
            start: Pos { offset: start },
            length: end - start,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SrcLoc {
    Known(Span),
    Unknown,
}

impl SrcLoc {
    pub fn attach<T>(&self, value: T) -> Located<T> {
        Located {
            value,
            location: SrcLoc::Unknown,
        }
    }

    pub fn enclosing(fst: &SrcLoc, snd: &SrcLoc) -> SrcLoc {
        match (fst, snd) {
            (SrcLoc::Known(a), SrcLoc::Known(b)) => SrcLoc::Known(Span::enclosing(a, b)),
            (SrcLoc::Known(a), SrcLoc::Unknown) => SrcLoc::Known(*a),
            (SrcLoc::Unknown, SrcLoc::Known(b)) => SrcLoc::Known(*b),
            _ => SrcLoc::Unknown,
        }
    }
}
