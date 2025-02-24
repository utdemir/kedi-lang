use crate::parser;

#[derive(Debug)]
pub enum Error {
    Parser(parser::Error),
}

// impl From<parser::Error> for Error {
//     fn from(e: parser::Error) -> Self {
//         Error::Parser(e)
//     }
// }

// impl From<renamer::Error> for Error {
//     fn from(e: renamer::Error) -> Self {
//         Error::Renamer(e)
//     }
// }
