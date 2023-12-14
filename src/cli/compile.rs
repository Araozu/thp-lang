use crate::cli::{get_help_text, get_version};
use colored::*;

#[derive(Eq, PartialEq, Hash)]
enum CompileOptions {
    Help,
}
