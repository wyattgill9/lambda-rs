mod ast;
mod parser;
mod evaluator;
mod error;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::parser::Parser;
use crate::evaluator::evaluate;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        if !Path::new(file_path).exists() {
            eprintln!("Error: File '{}' not found.", file_path);
            return Ok(());
        }

        let input = fs::read_to_string(file_path)?;
        let mut parser = Parser::new(&input);
        let results = parser.parse_multi_line();

        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(term) => {
                    let result = evaluate(term);
                    println!("Expression {}: {}", i + 1, result);
                }
                Err(e) => eprintln!("Error in expression {}: {}", i + 1, e),
            }
        }
    } else {
        // REPL mode
        println!("Lambda Calculus REPL (exit with Ctrl+C)");
        let mut input = String::new();

        loop {
            print!("λ> ");
            io::stdout().flush()?;
            input.clear();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            match Parser::new(input).parse() {
                Ok(term) => {
                    let result = evaluate(term);
                    println!("Result: {}", result);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}