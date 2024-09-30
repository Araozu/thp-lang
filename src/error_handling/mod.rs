use std::ops::Range;

use ariadne::{Label, Report, ReportKind, Source};
use serde::Serialize;

pub mod semantic_error;
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

pub type MistiError = ErrorContainer;

/*
#[derive(Serialize, Debug)]
pub enum MistiError {
    Lex(ErrorContainer),
    Syntax(ErrorContainer),
    Semantic(ErrorContainer),
}
*/

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
