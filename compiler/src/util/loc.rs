use std::collections::HashMap;

use crate::util::pp::{SExpr, SExprTerm};

// Location

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

    pub fn to_tagged(self, tag_map: &mut TagMap) -> Tagged<T> {
        let tag = tag_map.get_tag(self.location);
        Tagged {
            value: self.value,
            tag,
        }
    }

    pub fn map_result<U, E, F: FnOnce(&T) -> Result<U, E>>(&self, f: F) -> Result<Located<U>, E> {
        match f(&self.value) {
            Ok(value) => Ok(Located {
                value,
                location: self.location,
            }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub offset: usize,
}

impl SExpr for Pos {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("pos", vec![SExprTerm::Number(self.offset as i64)])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            location: *self,
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

    pub fn all_enclosing(locs: &[SrcLoc]) -> SrcLoc {
        locs.iter()
            .fold(SrcLoc::Unknown, |acc, &loc| SrcLoc::enclosing(&acc, &loc))
    }

    pub fn to_tag(self, tag_map: &mut TagMap) -> Tag {
        tag_map.get_tag(self)
    }
}

// Tags

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag {
    pub value: u32,
}

impl SExpr for Tag {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("tag", vec![SExprTerm::Number(self.value as i64)])
    }
}

impl Tag {
    pub fn attach<T>(&self, value: T) -> Tagged<T> {
        Tagged { value, tag: *self }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tagged<T> {
    pub value: T,
    pub tag: Tag,
}

impl<T: SExpr> SExpr for Tagged<T> {
    fn to_sexpr(&self) -> SExprTerm {
        self.value.to_sexpr()
    }
}

impl<T> Tagged<T> {
    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Tagged<U> {
        Tagged {
            value: f(&self.value),
            tag: self.tag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagMap {
    next_tag: u32,
    map: HashMap<Tag, SrcLoc>,
}

impl TagMap {
    pub fn new() -> TagMap {
        TagMap {
            next_tag: 0,
            map: HashMap::new(),
        }
    }

    pub fn get_tag(&mut self, loc: SrcLoc) -> Tag {
        let tag = Tag {
            value: self.next_tag,
        };
        self.next_tag += 1;
        self.map.insert(tag, loc);

        tag
    }

    pub fn resolve_tag(&self, tag: Tag) -> SrcLoc {
        match self.map.get(&tag) {
            Some(loc) => *loc,
            None => SrcLoc::Unknown,
        }
    }
}
