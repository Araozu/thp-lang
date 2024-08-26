use crate::cli::get_help_text;
use colored::*;

pub fn help_command(arguments: Vec<String>) -> Result<(), ()> {
    println!("{}", get_help_text());

    if !arguments.is_empty() {
        println!(
            "{}: {}",
            "warning".yellow(),
            "The help command doesn't take any argument."
        );
    }

    Ok(())
}
