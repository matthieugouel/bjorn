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

    fn comment (&mut self) -> Option<Token> {
        while let Some(&c) = self.peek() {
            if c == '\n' {
                break;
            } else {
                self.advance();
            }
        }
        self.next()
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

    fn id(&mut self, id: char) -> Option<Token> {
        let mut id = id.to_string();
        while let Some(&c) = self.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            id.push(self.advance().unwrap()); // TODO handle unwrap properly
        }
        Some(Token::ID(id))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        self.whitespace();

         match self.input.next() {
            Some('\n') => self.next(),
            Some(c) if c.is_numeric() => self.number(c),
            Some(c) if c.is_alphabetic() => self.id(c),
            Some('=') => Some(Token::ASSIGN),
            Some('+') => Some(Token::PLUS),
            Some('-') => Some(Token::MINUS),
            Some('*') => Some(Token::MUL),
            Some('/') => Some(Token::DIV),
            Some('(') => Some(Token::LPAREN),
            Some(')') => Some(Token::RPAREN),
            Some('#') => self.comment(),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use token::Token;

    fn scan_generator(input: &str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        let mut scan = Vec::new();
        for t in lexer {
            scan.push(t);
        }
        scan
    }

    #[test]
    fn whitespace() {
        let scan = scan_generator(" ");
        assert_eq!(scan, vec!());
    }

    #[test]
    fn lf() {
        let scan = scan_generator("\n");
        assert_eq!(scan, vec!());
    }

    #[test]
    fn comment() {
        let scan = scan_generator("# 2+2");
        assert_eq!(scan, vec!());
    }

    #[test]
    fn integer_number() {
        let scan = scan_generator("1");
        assert_eq!(scan, vec!(Token::INT("1".to_string())));
    }

    #[test]
    fn float_number() {
        let scan = scan_generator("1.0");
        assert_eq!(scan, vec!(Token::FLOAT("1.0".to_string())));
    }

    #[test]
    fn plus_operand() {
        let scan = scan_generator("+");
        assert_eq!(scan, vec!(Token::PLUS));
    }

    #[test]
    fn minus_operand() {
        let scan = scan_generator("-");
        assert_eq!(scan, vec!(Token::MINUS));
    }

    #[test]
    fn mul_operand() {
        let scan = scan_generator("*");
        assert_eq!(scan, vec!(Token::MUL));
    }

    #[test]
    fn div_operand() {
        let scan = scan_generator("/");
        assert_eq!(scan, vec!(Token::DIV));
    }

    #[test]
    fn parenthesis() {
        let scan = scan_generator("(1)");
        assert_eq!(scan, vec!(
            Token::LPAREN,
            Token::INT("1".to_string()),
            Token::RPAREN,
        ));
    }

    #[test]
    fn assign() {
        let scan = scan_generator("=");
        assert_eq!(scan, vec!(Token::ASSIGN));
    }

    #[test]
    fn id() {
        let scan = scan_generator("bjørn");
        assert_eq!(scan, vec!(Token::ID("bjørn".to_string())));
    }
}
