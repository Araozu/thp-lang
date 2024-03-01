use crate::cli::{get_help_text, get_version};
use colored::*;

#[derive(Eq, PartialEq, Hash)]
enum EmptyOptions {
    Help,
    Version,
}

pub fn empty_command(arguments: Vec<String>) -> Result<(), ()> {
    // Add all options to a set
    let mut options_set = std::collections::HashSet::new();
    for option in arguments {
        match expand_option(&option) {
            Ok(o) => {
                options_set.insert(o);
            }
            Err(invalid_option) => {
                eprintln!("{}", get_help_text());
                eprintln!("{}: invalid option: `{}`", "error".on_red(), invalid_option);
                return Err(());
            }
        };
    }
    let options = options_set;

    if options.is_empty() {
        println!("{}", get_help_text());
    } else {
        if options.contains(&EmptyOptions::Version) {
            println!("{}\n", get_version());
        }

        if options.contains(&EmptyOptions::Help) {
            println!("{}", get_help_text());
        }
    }

    Ok(())
}

fn expand_option(option: &String) -> Result<EmptyOptions, String> {
    match option.as_str() {
        "-h" | "--help" => Ok(EmptyOptions::Help),
        "-v" | "--version" => Ok(EmptyOptions::Version),
        _ => Err(option.clone()),
    }
}
