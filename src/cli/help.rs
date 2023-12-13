use crate::cli::get_help_text;
use colored::*;

pub fn help_command(options: &Vec<String>) {
    println!("{}", get_help_text());

    if options.len() > 0 {
        println!(
            "{}: {}",
            "warning".yellow(),
            "The help command doesn't take any options."
        );
    }
}
