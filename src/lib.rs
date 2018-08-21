//!
//! Interpreter for `bjørn` language.
//!

mod token;
mod lexer;
mod parser;
mod ast;
mod interpreter;
mod value;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

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
