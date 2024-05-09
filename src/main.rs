use std::error::Error;

use chumsky::Parser;

mod lexer;

#[derive(clap::Parser)]
enum Cli {
    Tokenize { source: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = <Cli as clap::Parser>::parse();

    match args {
        Cli::Tokenize { source } => {
            let source_text = std::fs::read_to_string(source)?;
            let tokens = match lexer::token().repeated().parse(source_text) {
                Ok(tokens) => tokens,
                Err(errors) => {
                    for error in errors {
                        println!("{:?}", error)
                    }
                    return Err("Failed to parse".into());
                }
            };

            for token in tokens {
                print!("{}", token)
            }
        }
    }

    Ok(())
}
