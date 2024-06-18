use miette;

use super::core;
use crate::parser;
use crate::renamer;
use crate::util::loc;

pub fn annotate_error<T: Into<core::Error>>(error: T, src: String) -> miette::Report {
    let diagnostic: miette::MietteDiagnostic = match error.into() {
        core::Error::Parser(p) => match p {
            parser::Error::ParseFailed(err) => miette::MietteDiagnostic {
                severity: Some(miette::Severity::Error),
                code: None,
                message: err.text,
                help: None,
                url: None,
                labels: None,
            },
        },
        core::Error::Renamer(e) => match e {
            renamer::Error::IdentifierNotFound(err) => miette::MietteDiagnostic {
                severity: Some(miette::Severity::Error),
                code: None,
                message: "Identifier not found".to_string(),
                help: None,
                url: None,
                labels: Some(vec![miette::LabeledSpan::new_primary_with_span(
                    Some("Defined here.".to_string()),
                    err.identifier.location,
                )]),
            },
            renamer::Error::DuplicateIdentifier(err) => miette::MietteDiagnostic {
                severity: Some(miette::Severity::Error),
                code: None,
                message: "Duplicate identifier".to_string(),
                help: None,
                url: None,
                labels: Some(vec![
                    miette::LabeledSpan::new_primary_with_span(
                        Some("[ERR] Duplicate identifier.".to_string()),
                        err.error.location,
                    ),
                    miette::LabeledSpan::new_with_span(
                        Some("Previously defined at.".to_string()),
                        err.original_loc,
                    ),
                ]),
            },
        },
    };

    return miette::Report::new(diagnostic).with_source_code(src);
}

impl Into<miette::SourceSpan> for loc::Span {
    fn into(self) -> miette::SourceSpan {
        miette::SourceSpan::new(self.start.offset.into(), self.length)
    }
}

impl Into<miette::SourceSpan> for loc::SrcLoc {
    fn into(self) -> miette::SourceSpan {
        match self {
            loc::SrcLoc::Known(span) => span.into(),
            loc::SrcLoc::Unknown => miette::SourceSpan::new(0.into(), 0),
        }
    }
}
