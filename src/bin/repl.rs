use std::io::{self, BufRead, Write};

use lang::{lexer, parser::Parser};

fn main() {
    println!("Lang REPL - Enter expressions (Ctrl+D to exit)");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "exit" || line == "quit" {
            println!("Goodbye!");
            break;
        }

        match lexer::scan(line) {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(expr) => println!("{:#?}", expr),
                    Err(e) => eprintln!("Parse error: {}", e),
                }
            }
            Err(e) => eprintln!("Lexer error: {}", e),
        }
    }
}
