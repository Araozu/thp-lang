use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("REPL: Enter expressions to evaluate. Type Ctrl-D to exit.");
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let read = stdin.read_line(&mut buffer);

        match read {
            Ok(0) => {
                println!("\nBye");
                return Ok(());
            },
            Ok(_) => {
                println!("{}", buffer);
            },
            Err(error) => {
                eprintln!("Error reading stdin.");
                return Err(error);
            }
        };
    }
}
