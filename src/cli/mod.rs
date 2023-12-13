mod help;
mod types;
mod empty;

use types::{Command, CommandType};

use colored::*;

pub fn get_help_text() -> String {
    format!(
        r#"
Usage: `thp [command] [options]`

Commands

  c {0}  Compiles {0} in-place
  f {0}  Formats {0}
  r         Starts the REPL

  init      Initializes a new project in the current directory
  build     Builds the project
  fmt       Formats all files in the project
  watch, w  Starts compilation of the project in watch mode

  help, h   Print this message & exit

General options

  -h, --help    Print command-specific usage
"#,
        "_file_".green()
    )
}

fn get_version() -> String {
    let crate_version = env!("CARGO_PKG_VERSION");
    format!("The THP compiler, linter & formatter, v{}", crate_version)
}

pub fn run_cli() {
    let command = match parse_args() {
        Ok(c) => c,
        Err(reason) => {
            println!("{}", get_help_text());
            println!("{}: {}", "error".on_red(), reason);
            return;
        }
    };

    command.run();
}

fn parse_args() -> Result<Command, String> {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    let mut args = args.into_iter();

    let command = match args.next() {
        Some(command) if !command.starts_with('-') => Some(command),
        _ => None,
    };

    let mut options = Vec::new();
    for arg in args {
        if arg.starts_with('-') {
            options.push(arg);
        } else {
            return Err(format!("Unexpected command `{}` after the options", arg));
        }
    }

    let command = match command {
        Some(command) => match command.as_str() {
            "c" | "compile" => CommandType::Compile,
            "f" | "format" => CommandType::Format,
            "r" | "repl" => CommandType::Repl,
            "init" => CommandType::Init,
            "build" => CommandType::Build,
            "fmt" => CommandType::Fmt,
            "watch" | "w" => CommandType::Watch,
            "help" | "h" => CommandType::Help,
            _ => return Err(format!("Unknown command `{}`", command)),
        },
        None => CommandType::None,
    };

    Ok(Command { command, options })
}
