use crate::{parser::syntax, util::ax::Ax};

#[derive(Debug)]
pub enum Error<L> {
    IdentifierNotFound(IdentifierNotFoundError<L>),
    DuplicateIdentifier(DuplicateIdentifierError<L>),
}

#[derive(Debug)]
pub struct IdentifierNotFoundError<L> {
    pub identifier: Ax<L, syntax::Ident>,
}

impl<L> From<DuplicateIdentifierError<L>> for Error<L> {
    fn from(e: DuplicateIdentifierError<L>) -> Self {
        Error::DuplicateIdentifier(e)
    }
}

#[derive(Debug)]
pub struct DuplicateIdentifierError<L> {
    pub error: Ax<L, syntax::Ident>,
    pub original_loc: L,
}

impl<L> From<IdentifierNotFoundError<L>> for Error<L> {
    fn from(e: IdentifierNotFoundError<L>) -> Self {
        Error::IdentifierNotFound(e)
    }
}
