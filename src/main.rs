use std::env;
use std::fs;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter the file path.");
        return;
    }

    let input = fs::read_to_string(&args[1])
        .expect("Impossible to read the file.");

    let _lexer = lexer::Lexer::new(input.as_str());
    let _parser = parser::Parser::new(_lexer);
    let mut _interpreter = interpreter::Interpreter::new(_parser);

    println!("{}", _interpreter.interpret());
}
