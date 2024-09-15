use std::ops::Range;

use ariadne::{Label, Report, ReportKind, Source};
use serde::Serialize;

use self::semantic_error::SemanticError;

mod lex_error;
pub mod semantic_error;
mod syntax_error;
mod utils;

pub mod error_messages;

pub trait PrintableError {
    fn get_error_str(&self, chars: &Vec<char>) -> String;
    fn print_ariadne(&self, source: &String);
}

#[derive(Serialize, Debug)]
pub struct ErrorContainer {
    pub error_code: u32,
    pub error_offset: usize,
    pub labels: Vec<ErrorLabel>,
    pub note: Option<String>,
    pub help: Option<String>,
}

/// Mirrors ariadne's Label
#[derive(Serialize, Debug)]
pub struct ErrorLabel {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Serialize, Debug)]
pub enum MistiError {
    Lex(ErrorContainer),
    Syntax(ErrorContainer),
    Semantic(SemanticError),
}

#[derive(Serialize, Debug)]
pub struct LexError {
    pub position: usize,
    // TODO: Add and end position
    pub end_position: usize,
    pub reason: String,
}

#[derive(Serialize, Debug)]
pub struct SyntaxError {
    pub error_start: usize,
    pub error_end: usize,
    pub reason: String,
}

impl PrintableError for MistiError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        match self {
            Self::Lex(_) => panic!("REMOVED: manually generating an error message"),
            Self::Syntax(err) => err.get_error_str(chars),
            Self::Semantic(err) => err.get_error_str(chars),
        }
    }

    fn print_ariadne(&self, source: &String) {
        match self {
            Self::Lex(err) => err.print_ariadne(source),
            Self::Syntax(err) => err.print_ariadne(source),
            Self::Semantic(err) => err.print_ariadne(source),
        }
    }
}

impl PrintableError for ErrorContainer {
    fn get_error_str(&self, _: &Vec<char>) -> String {
        panic!("REMOVED: manually generating an error message")
    }

    fn print_ariadne(&self, source: &String) {
        let mut report: ariadne::ReportBuilder<'_, (&str, Range<usize>)> =
            Report::build(ReportKind::Error, "sample.thp", self.error_offset);

        for label in self.labels.iter() {
            let l = Label::new(("sample.thp", label.start..label.end))
                .with_message(label.message.clone());
            report = report.with_label(l)
        }

        if let Some(help) = &self.help {
            report = report.with_help(help);
        }

        if let Some(note) = &self.note {
            report = report.with_help(note);
        }

        report
            .with_code(self.error_code)
            .finish()
            .eprint(("sample.thp", Source::from(source)))
            .unwrap()
    }
}
