use std::fs;
use lexer::Lexer;
use crate::interpreter::Interpreter;
use crate::parser::Parser;

mod lexer;
mod error;
mod tests;
mod token;
mod parser;
mod ast;
mod interpreter;

fn main(){
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- filename");
        return;
    }

    let filename = &args[1];

    let source = match fs::read_to_string(filename) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Error reading file");
            return;
        }
    };

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize(){
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("lexer error at: {}", error.pos);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program(){
        Ok(program) => program,
        Err(error) => {
            eprintln!("parser error: {} at: {}", error.message,error.pos);
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.run(program) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("runtime error: {}", error.message);
        }
    }
}
