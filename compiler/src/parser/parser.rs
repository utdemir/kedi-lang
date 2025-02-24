#![allow(unused)]

use crate::parser::error::*;
use crate::parser::syntax;
use crate::phase::TransformPhase;
use crate::util::loc::Span;
use crate::util::loc::SrcLoc;

pub fn parse(input: &str) -> Result<syntax::Module<SrcLoc>, Error> {
    let parser = crate::parser::grammar::ModuleParser::new();
    match parser.parse(input) {
        Ok(module) => Ok(module),
        Err(e) => {
            let msg: String;
            let span: Span;

            match e {
                lalrpop_util::ParseError::InvalidToken { location } => {
                    msg = "Unexpected token".to_string();
                    span = Span::from_offset_len(location, 1);
                }
                lalrpop_util::ParseError::UnrecognizedEof { location, expected } => todo!(),
                lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                    let (start, tok, end) = token;
                    msg = format!(
                        "Unexpected token `{}`. Expected one of: {}",
                        tok,
                        expected.join(", ")
                    );
                    span = Span::from_offset_bytes(start, end);
                }
                lalrpop_util::ParseError::ExtraToken { token } => todo!(),
                lalrpop_util::ParseError::User { error } => todo!(),
            }

            Err(Error::ParseFailed(ParseFailed { msg, span }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar;
    use crate::parser::syntax;

    #[test]
    fn identifier() {
        let r = grammar::IdentParser::new().parse("hello");
        assert_eq!(r, Ok(syntax::Ident("hello".to_string())));
    }

    #[test]
    fn litnum() {
        let r = grammar::LitNumParser::new().parse("42");
        assert_eq!(r, Ok(syntax::LitNum(42)));
    }

    #[test]
    fn litstr() {
        let r = grammar::LitStrParser::new().parse("\"hello\"");
        assert_eq!(r, Ok(syntax::LitStr("hello".to_string())));
    }

    #[test]
    fn funcall() {
        grammar::FunCallParser::new().parse("f 42").unwrap();
        grammar::FunCallParser::new()
            .parse("f \"hello\" 12 12")
            .unwrap();
    }
}
