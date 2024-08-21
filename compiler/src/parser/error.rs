use crate::util::loc::Span;

#[derive(Debug)]
pub enum Error {
    ParseFailed(ParseFailed),
}

#[derive(Debug)]
pub struct ParseFailed {
    pub msg: String,
    pub span: Span,
}
