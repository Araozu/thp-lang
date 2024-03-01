#[derive(Debug, PartialEq)]
pub enum CommandType {
    Compile,
    Format,
    Repl,
    Init,
    Build,
    Fmt,
    Watch,
    Help,
    None,
}

impl CommandType {
    pub fn run(&self, options: Vec<String>) -> Result<(), ()> {
        match self {
            CommandType::Help => super::help::help_command(options),
            CommandType::Compile => super::compile::compile_command(options),
            CommandType::Repl => super::repl::repl_command(options),
            CommandType::None => super::empty::empty_command(options),
            _ => {
                eprintln!("Not implemented yet! {:?} {:?}", self, options);
                Err(())
            }
        }
    }
}
