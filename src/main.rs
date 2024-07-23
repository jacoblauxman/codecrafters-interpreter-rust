use interpreter_starter_rust::{Parser, Scanner};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);

        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(filename),
        "parse" => parse(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(source_str: &str) {
    let file_contents = fs::read_to_string(source_str).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", source_str);
        String::new()
    });

    if !file_contents.is_empty() {
        let scanner = Scanner::new(file_contents);
        let (tokens, errors) = scanner.scan_tokens();

        for error in &errors {
            eprintln!("{}", error)
        }

        for token in tokens {
            println!("{}", token)
        }

        if !errors.is_empty() {
            process::exit(65)
        }
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }
}

fn parse(source_str: &str) {
    let file_contents = fs::read_to_string(source_str).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", source_str);
        String::new()
    });

    if !file_contents.is_empty() {
        let scanner = Scanner::new(file_contents);
        let (tokens, errors) = scanner.scan_tokens();

        for error in &errors {
            eprintln!("{}", error)
        }

        if !errors.is_empty() {
            process::exit(65)
        }

        let mut parser = Parser::new(tokens);

        if let Ok(expr) = parser.parse() {
            println!("{expr}");
        }
    }
}
