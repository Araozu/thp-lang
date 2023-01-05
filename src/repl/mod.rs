use std::io::{self, Write};

use super::lexic;
use super::syntax;

fn compile(input: &String) {
    let _tokens = lexic::get_tokens(input);

    match _tokens {
        Ok(tokens) => {
            for token in tokens {
                print!("[{:?} {}] ", token.token_type, token.value);
            }
            println!("");

            let _ast = syntax::construct_ast(Vec::new());
        },
        Err(error) => {
            eprintln!("Error scanning.\n{} at pos {}", error.reason, error.position)
        }
    }

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
                compile(&buffer);
            },
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error)
            }
        };
    }
}
