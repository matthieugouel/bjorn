use std::str::Chars;
use std::iter::Peekable;

use token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Lexer<'a>  {
        Lexer { input: input.chars().peekable() }
    }

    fn advance(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek (&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn whitespace (&mut self) {
        while let Some(&c) = self.peek() {
            if !c.is_whitespace() {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn number(&mut self, number: char) -> Option<Token> {
        let mut number = number.to_string();
        while let Some(&c) = self.peek() {
            if c == '.' {
                number.push(self.advance().unwrap()); // TODO handle unwrap properly
                while let Some(&d) = self.peek() {
                    if !d.is_numeric() {
                        break;
                    }
                    number.push(self.advance().unwrap()); // TODO handle unwrap properly
                }
                return Some(Token::FLOAT(number));
            }
            if !c.is_numeric() {
                break;
            }
            number.push(self.advance().unwrap()); // TODO handle unwrap properly
        }
        Some(Token::INT(number))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        self.whitespace();

         match self.input.next() {
            Some(c) if c.is_numeric() => self.number(c),
            Some('+') => Some(Token::PLUS),
            Some('-') => Some(Token::MINUS),
            Some('*') => Some(Token::MUL),
            Some('/') => Some(Token::DIV),
            Some('(') => Some(Token::LPAREN),
            Some(')') => Some(Token::RPAREN),
            _ => None,
        }
    }
}
