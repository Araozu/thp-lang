use colored::Colorize;

pub fn repl_command(_arguments: Vec<String>) -> Result<(), ()> {
    println!("{}", super::get_version());
    let result = crate::repl::run();

    if let Err(e) = result {
        eprintln!("{}: {}", "error".on_red(), e);
        return Err(());
    }

    Ok(())
}
