use std::io::{self, Write};

use super::lexic;

fn compile(input: &String) {
    let _tokens = lexic::get_tokens(input);
}

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("REPL: Enter expressions to evaluate. Type Ctrl-D to exit.");
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        buffer.clear();
        let read = stdin.read_line(&mut buffer);

        match read {
            Ok(0) => {
                println!("\nBye");
                break Ok(())
            },
            Ok(_) => {
                println!("{}", buffer);
                compile(&buffer);
            },
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error)
            }
        };
    }
}
