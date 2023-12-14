use colored::*;

pub fn compile_command(arguments: Vec<String>) {
    if arguments.is_empty() {
        println!("{}", compile_help());
        println!("{}: {}", "error".on_red(), "No file specified");
        return;
    }
    if arguments.len() > 1 {
        println!("{}", compile_help());
        println!(
            "{}: {}",
            "error".on_red(),
            "Only a single file can be compiled at a time"
        );
        return;
    }

    let argument = &arguments[0];
    if argument.starts_with("-") {
        let opt_str = argument.as_str();

        println!("{}", compile_help());

        if opt_str != "-h" && opt_str != "--help" {
            println!(
                "{}: {}",
                "error".on_red(),
                "Invalid option. The compile command only accepts the `-h` or `--help` option"
            );
        }
        return;
    }

    crate::file::compile_file(argument);
}

fn compile_help() -> String {
    format!(
        r#"Compile a single file in place.

Usage:

  `thp compile {0}`    Compile {0} and output in the same directory
  `thp compile -h`        Print this message & exit
        "#,
        "_file_".green()
    )
}
