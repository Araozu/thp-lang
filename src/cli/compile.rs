use colored::*;

pub fn compile_command(arguments: Vec<String>) -> Result<(), ()> {
    if arguments.is_empty() {
        eprintln!("{}", compile_help());
        eprintln!("{}: {}", "error".on_red(), "No file specified");
        return Err(());
    }
    if arguments.len() > 1 {
        eprintln!("{}", compile_help());
        eprintln!(
            "{}: {}",
            "error".on_red(),
            "Only a single file can be compiled at a time"
        );
        return Err(());
    }

    let argument = &arguments[0];
    if argument.starts_with("-") {
        let opt_str = argument.as_str();

        println!("{}", compile_help());

        if opt_str != "-h" && opt_str != "--help" {
            eprintln!(
                "{}: {}",
                "error".on_red(),
                "Invalid option. The compile command only accepts the `-h` or `--help` options"
            );
        }
        return Err(());
    }

    crate::file::compile_file(argument)
}

fn compile_help() -> String {
    format!(
        r#"Compile a single file in place. If the file to compile
references other THP files, they will be (typechecked?) as well.

Usage:

  `thp compile {0}`    Compile {0} and output in the same directory
  `thp compile -h`        Print this message & exit
        "#,
        "_file_".green()
    )
}
