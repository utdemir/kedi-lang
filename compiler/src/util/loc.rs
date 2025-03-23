use sexpr::SExpr;
use sexpr_derive::SExpr;
use std::collections::HashMap;

use super::ax::{ax, Ax};

// Location

pub type WithLoc<T> = Ax<SrcLoc, T>;

pub type LVec<T> = WithLoc<Vec<T>>;

impl<T: sexpr::SExpr> sexpr::SExpr for WithLoc<T> {
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        self.v.to_sexpr()
    }
}

impl<T> WithLoc<T> {
    pub fn known(value: T, location: Span) -> WithLoc<T> {
        ax(SrcLoc::Known(location), value)
    }

    pub fn unknown(value: T) -> WithLoc<T> {
        ax(SrcLoc::Unknown, value)
    }

    pub fn to_tagged(self, tag_map: &mut TagMap) -> WithTag<T> {
        let tag = tag_map.get_tag(self.a);
        ax(tag, self.v)
    }
}

pub trait Located {
    fn location(&self) -> SrcLoc;
}

impl<T> Located for WithLoc<T> {
    fn location(&self) -> SrcLoc {
        self.a
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SExpr)]
pub struct Offset(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Offset,
    pub length: usize,
}

impl sexpr::SExpr for Span {
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        sexpr::call(
            "span",
            &[self.start.to_sexpr(), sexpr::number(self.length as i64)],
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SExpr)]
pub enum SrcLoc {
    Known(Span),
    Unknown,
}

impl SrcLoc {
    pub fn attach<T>(&self, value: T) -> WithLoc<T> {
        ax(*self, value)
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

impl sexpr::SExpr for Tag {
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        sexpr::call("tag", &[sexpr::number(self.value)])
    }
}

type WithTag<T> = Ax<Tag, T>;

impl<T: sexpr::SExpr> sexpr::SExpr for WithTag<T> {
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        self.v.to_sexpr()
    }
}

pub trait Tagged {
    fn tag(&self) -> Tag;
}

impl<T> Tagged for WithTag<T> {
    fn tag(&self) -> Tag {
        self.a
    }
}

#[derive(Debug, Clone)]
pub struct TagMap {
    next_tag: u32,
    map: HashMap<Tag, SrcLoc>,
}

impl SExpr for TagMap {
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        sexpr::call("tag-map", &[self.map.to_sexpr()])
    }
}

impl Default for TagMap {
    fn default() -> Self {
        Self::new()
    }
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

pub trait LocLike: Clone + std::fmt::Debug {
    fn enclosing(fst: &Self, snd: &Self) -> Self;
}

impl LocLike for SrcLoc {
    fn enclosing(fst: &Self, snd: &Self) -> Self {
        SrcLoc::enclosing(fst, snd)
    }
}

impl LocLike for () {
    fn enclosing(_: &Self, _: &Self) -> Self {
        ()
    }
}
