use ::std::io::{self, BufRead, BufReader, Error, Read, Write};
use ::std::process::{Command, Stdio};
use ::std::thread;
use ::std::time::Duration;

use crate::codegen::Transpilable;
use crate::error_handling::PrintableError;
use crate::semantic::std;
use crate::semantic::symbol_table::SymbolTable;

use super::lexic;
use super::syntax;

use crate::php_ast::transformers::PHPTransformable;

/// Executes the REPL, reading from stdin, compiling and emitting PHP to stdout
pub fn run() -> io::Result<()> {
    // attempt to spawn a php repl

    let php_repl = Command::new("php")
        .arg("-a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
    let mut php_repl = match php_repl {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Couldn't open a PHP REPL session: {:?}", error);
            return Err(error);
        }
    };

    let mut php_stdin = match php_repl.stdin.take() {
        Some(handle) => handle,
        None => {
            eprintln!("Error: couldn't get stdin handle from PHP REPL");
            return Err(Error::new(
                io::ErrorKind::Other,
                "Can't get PHP REPL stdin handle",
            ));
        }
    };

    let mut php_stdout = match php_repl.stdout.take() {
        Some(h) => h,
        None => {
            eprintln!("Error: couldn't get stdout handle from PHP REPL");
            return Err(Error::new(
                io::ErrorKind::Other,
                "Can't get PHP REPL stdout handle",
            ));
        }
    };

    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut repl_symbol_table = SymbolTable::new();
    std::populate(&mut repl_symbol_table);

    // start a thread that prints whatever php sends back
    let php_stdout_handle = thread::spawn(move || {
        let mut reader = BufReader::new(php_stdout);

        loop {
            // sleep for 50ms
            thread::sleep(Duration::from_millis(50));

            // read a line from php
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        // EOF
                        break;
                    }
                    if n == 1 {
                        // just a newline
                        continue;
                    }

                    // Suppress some php outputs
                    if line == "Interactive shell\n" {
                        continue;
                    }
                    // Ignore anything that starts with `php > `
                    if line.starts_with("php > ") {
                        continue;
                    }

                    print!("php output: `{line}`")
                }
                Err(error) => {
                    // log error and exit
                    eprint!("Error while reading from PHP STDOUT: {:?}", error);
                    break;
                }
            };
        }

        println!("php stdout thread finished");
    });

    println!("REPL: Enter expressions to evaluate. Type Ctrl-D to exit.");
    let result = loop {
        // TODO: syncronize the writes to thp stdout
        // such that the php output doesnt overlap with this
        print!("> ");
        io::stdout().flush()?;
        buffer.clear();
        let read = stdin.read_line(&mut buffer);

        match read {
            Ok(0) => {
                println!("\nBye");
                break Ok(());
            }
            Ok(_) => {
                match compile(&buffer, &mut repl_symbol_table) {
                    Some(php_code) => {
                        // TODO: this cant be efficient, fix
                        let php_code = format!("{php_code}\n");

                        // send php code
                        //println!("{php_code}");
                        match php_stdin.write_all(php_code.as_bytes()) {
                            Ok(_) => {}
                            Err(error) => {
                                eprintln!("Error writing the generated code to the PHP process.");
                                break Err(error);
                            }
                        };

                        // the php repl should respond with its output, and that
                        // will be printed by another thread
                    }
                    None => {}
                }
            }
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error);
            }
        };
    };

    // kill the php process
    php_repl.kill().expect("Couldnt KILL child php process...");

    php_stdout_handle
        .join()
        .expect("STDOUT thread failed to join...");

    result
}

/// Compiles THP code and returns the generated PHP code as a String
fn compile(input: &String, symbol_table: &mut SymbolTable) -> Option<String> {
    //
    // Lexical analysis
    //
    let tokens = match lexic::get_tokens(input) {
        Ok(t) => t,
        Err(error) => {
            error.print_ariadne(input);
            return None;
        }
    };

    //
    // Syntax analysis
    //
    let ast = match syntax::build_ast(&tokens) {
        Ok(ast) => ast,
        Err(error) => {
            error.print_ariadne(input);
            return None;
        }
    };

    //
    // Semantic analysis
    //
    let res1 = crate::semantic::check_semantics_with(&ast, symbol_table);
    match res1 {
        Ok(_) => {}
        Err(error) => {
            error.print_ariadne(input);
            return None;
        }
    }

    //
    // Intermediate representation (THP -> PHP ast)
    //
    let php_ast = ast.into_php_ast();

    //
    // Codegen
    //
    Some(php_ast.transpile_without_header())
}
