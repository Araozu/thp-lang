pub fn repl_command(_arguments: Vec<String>) {
    println!("{}", super::get_version());
    let _ = crate::repl::run();
}
