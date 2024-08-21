use std::collections::HashMap;

use crate::util::pp::{SExpr, SExprTerm};

// Location

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithLoc<T> {
    pub value: T,
    pub location: SrcLoc,
}

pub type LVec<T> = WithLoc<Vec<T>>;

impl<T: SExpr> SExpr for WithLoc<T> {
    fn to_sexpr(&self) -> SExprTerm {
        self.value.to_sexpr()
    }
}

impl<T> WithLoc<T> {
    pub fn known(value: T, location: Span) -> WithLoc<T> {
        WithLoc {
            value,
            location: SrcLoc::Known(location),
        }
    }

    pub fn unknown(value: T) -> WithLoc<T> {
        WithLoc {
            value,
            location: SrcLoc::Unknown,
        }
    }

    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> WithLoc<U> {
        WithLoc {
            value: f(&self.value),
            location: self.location,
        }
    }

    pub fn to_tagged(self, tag_map: &mut TagMap) -> WithTag<T> {
        let tag = tag_map.get_tag(self.location);
        WithTag {
            value: self.value,
            tag,
        }
    }

    pub fn map_result<U, E, F: FnOnce(&T) -> Result<U, E>>(&self, f: F) -> Result<WithLoc<U>, E> {
        match f(&self.value) {
            Ok(value) => Ok(WithLoc {
                value,
                location: self.location,
            }),
            Err(err) => Err(err),
        }
    }
}

pub trait Located {
    fn location(&self) -> SrcLoc;
}

impl<T> Located for WithLoc<T> {
    fn location(&self) -> SrcLoc {
        self.location
    }
}

pub fn lost<T>(value: T) -> WithLoc<T> {
    WithLoc {
        value,
        location: SrcLoc::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset(pub usize);

impl SExpr for Offset {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("pos", &[SExprTerm::Number(self.0 as i64)])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Offset,
    pub length: usize,
}

impl SExpr for Span {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "span",
            &[self.start.to_sexpr(), SExprTerm::Number(self.length as i64)],
        )
    }
}

impl Span {
    // Returns the span that encloses both `self` and `other`.
    pub fn enclosing(fst: &Span, snd: &Span) -> Span {
        let self_start = fst.start.0;
        let self_end = fst.start.0 + fst.length;

        let other_start = snd.start.0;
        let other_end = snd.start.0 + snd.length;

        let start = self_start.min(other_start);
        let end = self_end.max(other_end);

        Span {
            start: Offset(start),
            length: end - start,
        }
    }

    pub fn from_offsets(start: Offset, end: Offset) -> Span {
        Span {
            start,
            length: std::cmp::max(end.0 - start.0, 0),
        }
    }

    pub fn from_offset_bytes(start: usize, end: usize) -> Span {
        Span::from_offsets(Offset(start), Offset(end))
    }

    pub fn from_offset_len(start: usize, len: usize) -> Span {
        Span {
            start: Offset(start),
            length: len,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SrcLoc {
    Known(Span),
    Unknown,
}

impl SrcLoc {
    pub fn attach<T>(&self, value: T) -> WithLoc<T> {
        WithLoc {
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
        SExprTerm::call("tag", &[SExprTerm::Number(self.value as i64)])
    }
}

impl Tag {
    pub fn attach<T>(&self, value: T) -> WithTag<T> {
        WithTag { value, tag: *self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithTag<T> {
    pub value: T,
    pub tag: Tag,
}

impl<T: SExpr> SExpr for WithTag<T> {
    fn to_sexpr(&self) -> SExprTerm {
        self.value.to_sexpr()
    }
}

impl<T> WithTag<T> {
    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> WithTag<U> {
        WithTag {
            value: f(&self.value),
            tag: self.tag,
        }
    }
}

pub trait Tagged {
    fn tag(&self) -> Tag;
}

impl<T> Tagged for WithTag<T> {
    fn tag(&self) -> Tag {
        self.tag
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

// Synonyms
