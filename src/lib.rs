//!
//! Interpreter for `bjørn` language.
//!

#[macro_use] extern crate lazy_static;
extern crate unicode_segmentation;
extern crate regex;

mod token;
mod lexer;
mod parser;
mod ast;
pub mod memory;
mod interpreter;
mod value;
pub mod builtins;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

///
/// Only proceed to the lexical analysis.
/// For testing purposes.
///
/// ```
/// extern crate bjorn;
///
/// let input = "2 + 2";
/// println!("{:?}", bjorn::scan(input));
/// ```
///
pub fn scan(input: &str) -> Vec<token::Token> {
    let lexer = Lexer::new(input);
    let mut scan = Vec::new();
    for t in lexer {
        scan.push(t);
    }
    scan.into_iter().flatten().collect::<Vec<token::Token>>()
}

///
/// Only proceed to the lexical and syntaxic analysis.
/// For testing purposes.
///
/// ```
/// extern crate bjorn;
///
/// let input = "2 + 2";
/// println!("{:?}", bjorn::parse(input));
/// ```
///
pub fn parse(input: &str) -> ast::AST {
    Parser::new(
        Lexer::new(input)
    ).parse()
}

///
/// Entrypoint of `bjorn` library.
///
/// ```
/// extern crate bjorn;
///
/// let input = "2 + 2";
/// println!("{}", bjorn::interpret(input));
/// ```
///
pub fn interpret(input: &str) -> String {
    Interpreter::new(
        Parser::new(
            Lexer::new(input)
        )
    ).interpret().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_entrypoint() {
        assert_eq!(interpret(""), "")
    }
}
