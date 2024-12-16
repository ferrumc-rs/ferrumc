use crate::message::Span;
use colored::Colorize;
use nom::error::{ErrorKind, FromExternalError, ParseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct MessageError<'a> {
    /*pub input: &'a str,
    pub line: usize,
    pub column_start: usize,
    pu*b column_width: usize,*/
    pub span: Span<'a>,
    pub source: InnerMessageError,
}

impl<'a> ParseError<Span<'a>> for MessageError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Self {
            span: input,
            source: InnerMessageError::NomError(kind),
        }
    }

    fn append(input: Span<'a>, kind: ErrorKind, _other: Self) -> Self {
        Self::from_error_kind(input, kind)
    }
}

impl<'a, E> FromExternalError<Span<'a>, E> for MessageError<'a> {
    fn from_external_error(input: Span<'a>, kind: ErrorKind, _e: E) -> Self {
        Self::from_error_kind(input, kind)
    }
}

#[derive(Debug, PartialEq, Clone, Error)]
pub enum InnerMessageError {
    #[error("{0:#?}")]
    NomError(ErrorKind),
    #[error("Unexpected end tag (expected \"</{1}>\", found \"</{0}>\")")]
    ExpectedEndTag(&'static str, &'static str),
    #[error("Missing end tag for \"<{0}>\"")]
    MissingEndTag(&'static str),
    #[error("Unexpected end tag for \"<{0}>\"")]
    UnexpectedEndTag(&'static str),
    #[error("Missing start tag for \"</{0}>\"")]
    MissingStartTag(&'static str),
}

#[derive(Debug, PartialEq, Clone, Error)]
pub enum ResolveError {
    #[error("Unable to parse a {expected_type} from '{expected}'. {description}")]
    ExpectedType {
        expected_type: &'static str,
        expected: String,
        description: String,
    },
    #[error("Process function is unimplemented for tag resolver \"<{0}>\"")]
    ProcessUnimplemented(String),
    #[error("Couldn't find resolver for tag \"<{0}>\"")]
    NoResolverFor(String),
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    CustomStatic(&'static str),
}

fn fill_carets(col: (usize, usize), caret: &str) -> String {
    format!("{}{}", " ".repeat(col.0), caret.repeat(col.1))
}

impl fmt::Display for MessageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = self.span.location_line().to_string();
        let space = " ".repeat(std::cmp::max(5 - (2 + line.len()), 0));
        let ln = " |".blue().to_string();
        write!(
            f,
            "{}:
        \r {ln}{space}
        \r {ln}{space}{}
        \r{num}{}
        \r {ln}{space}
        ",
            "error".bright_red(),
            self.span.fragment(),
            format!(
                "{} {}",
                fill_carets((self.span.get_column(), self.span.location_offset()), "^"),
                self.source
            )
            .bright_red(),
            num = line.blue().to_string() + &ln + &space
        )
    }
}
