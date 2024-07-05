use std::io::{self, BufRead};
use crate::lexic::get_tokens;

pub fn tokenize_command(_options: Vec<String>) -> Result<(), ()> {
    // Get the input from stdin
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                lines.push(line)
            }
            Err(reason) => {
                eprintln!("Error reading input: {}", reason);
                return Err(())
            }
        }
    }

    let input_code = lines.join("\n");
    let tokens = get_tokens(&input_code);

    let json = serde_json::to_string(&tokens).unwrap();
    println!("{}", json);

    Ok(())
}
