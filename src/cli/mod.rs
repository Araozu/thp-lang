mod compile;
mod empty;
mod help;
mod repl;
mod types;

use types::CommandType;

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
  -v, --version Print version & exit
"#,
        "_file_".green()
    )
}

fn get_version() -> String {
    let crate_version = env!("CARGO_PKG_VERSION");
    format!("The THP compiler, linter & formatter, v{}", crate_version)
}

pub fn run_cli() -> Result<(), ()> {
    let (command, args) = match parse_args() {
        Ok(c) => c,
        Err(reason) => {
            eprintln!("{}", get_help_text());
            eprintln!("{}: {}", "error".on_red(), reason);
            return Err(());
        }
    };

    command.run(args)
}

fn parse_args() -> Result<(CommandType, Vec<String>), String> {
    let mut args = std::env::args().collect::<Vec<String>>();

    // Remove the first argument, which is the path to the executable
    args.remove(0);

    let command = match args.get(0) {
        Some(command) if !command.starts_with('-') => match command.as_str() {
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
        _ => CommandType::None,
    };

    if command != CommandType::None {
        args.remove(0);
    }

    Ok((command, args))
}
