#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub options: Vec<String>,
}

#[derive(Debug)]
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

impl Command {
    pub fn run(&self) {
        self.command.run(&self.options);
    }
}

impl CommandType {
    pub fn run(&self, options: &Vec<String>) {
        match self {
            CommandType::Help => super::help::help_command(options),
            CommandType::None => super::empty::empty_command(options),
            _ => {
                println!("Not implemented yet! {:?} {:?}", self, options);
            }
        }
    }
}
