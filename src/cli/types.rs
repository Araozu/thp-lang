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
        println!("Running command! {:?}", self);
        self.command.run(&self.options);
    }
}

impl CommandType {
    pub fn run(&self, options: &Vec<String>) {
        println!("Running command! {:?}", self)
    }
}
