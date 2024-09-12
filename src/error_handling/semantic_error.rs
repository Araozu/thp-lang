use ariadne::{Color, Label, Report, ReportKind, Source};
use serde::Serialize;

use super::utils::{get_line, get_line_number};
use super::PrintableError;

#[derive(Serialize, Debug)]
pub struct SemanticError {
    pub error_start: usize,
    pub error_end: usize,
    pub reason: String,
}

impl PrintableError for SemanticError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        let (line, before, length) = get_line(chars, self.error_start, self.error_end);

        let line_number = get_line_number(chars, self.error_start);
        let line_number_whitespace = " ".repeat(line_number.to_string().len());

        let whitespace = vec![' '; before].iter().collect::<String>();
        let indicator = vec!['^'; length].iter().collect::<String>();
        let reason = &self.reason;

        format!(
            r#"
{line_number_whitespace} |
{line_number           } | {line}
{line_number_whitespace} | {whitespace}{indicator}

{reason} at line {line_number}:{before}"#,
        )
    }

    fn print_ariadne(&self, source: &String) {
        let report = Report::build(ReportKind::Error, "sample.thp", self.error_start)
            .with_label(
                Label::new(("sample.thp", self.error_start..self.error_end))
                    .with_message(self.reason.clone())
                    .with_color(Color::Red),
            )
            .finish();

        report.eprint(("sample.thp", Source::from(source)));
    }
}
