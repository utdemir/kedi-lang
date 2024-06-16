use crate::renamer;

pub enum Error {
    Renamer(renamer::Error),
}

impl From<renamer::Error> for Error {
    fn from(e: renamer::Error) -> Self {
        Error::Renamer(e)
    }
}
