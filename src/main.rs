use std::fs;

use lang::{lexer, parser::Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = match fs::read_to_string("data/source.lg") {
        Ok(contents) => contents,
        Err(error) => {
            return Err(Box::new(error));
        }
    };

    let tokens = lexer::scan(&contents)?;

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    println!("{:#?}", expr);

    Ok(())
}
