use crate::cli::get_help_text;
use colored::*;

pub fn help_command(arguments: Vec<String>) {
    println!("{}", get_help_text());

    if arguments.len() > 0 {
        println!(
            "{}: {}",
            "warning".yellow(),
            "The help command doesn't take any argument."
        );
    }
}
