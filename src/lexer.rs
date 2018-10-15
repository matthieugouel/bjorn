use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::Graphemes;
use regex::Regex;

use std::iter::Peekable;

use token::Token;


lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"^\d").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"^[a-zA-Z]").unwrap();
    static ref WORD: Regex = Regex::new(r"^\w+").unwrap();
}

pub struct Lexer<'a> {
    input: Peekable<Graphemes<'a>>,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Lexer<'a>  {
        Lexer { input: UnicodeSegmentation::graphemes(input, true).peekable() }
    }

    fn advance(&mut self) -> &str {
        match self.input.next() {
            Some(c) => c,
            None => panic!("Lexical error.")
        }
    }

    fn comment (&mut self) -> Option<Token> {
        while let Some(&c) = self.input.peek() {
            if c == "\n" {
                break;
            } else {
                self.advance();
            }
        }
        self.next()
    }

    fn whitespace (&mut self) {
        while let Some(&c) = self.input.peek() {
            if c != " " {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn number(&mut self, number: &str) -> Option<Token> {
        let mut number = number.to_string();
        while let Some(&c) = self.input.peek() {
            if c == "." {
                number.push_str(self.advance());
                while let Some(&d) = self.input.peek() {
                    if !NUMERIC.is_match(d) {
                        break;
                    }
                    number.push_str(self.advance());
                }
                return Some(Token::FLOAT(number));
            }
            if !NUMERIC.is_match(c) {
                break;
            }
            number.push_str(self.advance());
        }
        Some(Token::INT(number))
    }

    fn id(&mut self, id: &str) -> Option<Token> {
        let mut id = id.to_string();
        while let Some(&c) = self.input.peek() {
            if !WORD.is_match(c) {
                break;
            }
            id.push_str(self.advance());
        }
        match id.as_ref() {
            "true" => Some(Token::BOOL(true)),
            "false" => Some(Token::BOOL(false)),
            _ => Some(Token::ID(id))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        self.whitespace();

         match self.input.next() {
            Some(c) if NUMERIC.is_match(c) => self.number(c),
            Some(c) if ALPHABETIC.is_match(c) => self.id(c),
            Some("\n") => Some(Token::NEWLINE),
            Some("=") => {
                if self.input.peek() == Some(&"=") {
                    self.next();
                    Some(Token::EQ)
                } else {
                    Some(Token::ASSIGN)
                }

            },
            Some("!") => {
                if self.input.peek() == Some(&"=") {
                    self.next();
                    Some(Token::NE)
                } else {
                    None
                }
            }
            Some("<") => {
                if self.input.peek() == Some(&"=") {
                    self.next();
                    Some(Token::LE)
                } else {
                    Some(Token::LT)
                }
            },
            Some(">") => {
                if self.input.peek() == Some(&"=") {
                    self.next();
                    Some(Token::GE)
                } else {
                    Some(Token::GT)
                }
            },
            Some("+") => Some(Token::PLUS),
            Some("-") => Some(Token::MINUS),
            Some("*") => Some(Token::MUL),
            Some("/") => Some(Token::DIV),
            Some("(") => Some(Token::LPAREN),
            Some(")") => Some(Token::RPAREN),
            Some("#") => self.comment(),

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
        assert_eq!(scan, vec!(Token::NEWLINE));
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
    fn boolean_true() {
        let scan = scan_generator("true");
        assert_eq!(scan, vec!(Token::BOOL(true)));
    }

    #[test]
    fn boolean_false() {
        let scan = scan_generator("false");
        assert_eq!(scan, vec!(Token::BOOL(false)));
    }

    #[test]
    fn id() {
        let scan = scan_generator("bjørn");
        assert_eq!(scan, vec!(Token::ID("bjørn".to_string())));
    }

    #[test]
    fn comparison_eq() {
        let scan = scan_generator("==");
        assert_eq!(scan, vec!(Token::EQ));
    }

    #[test]
    fn comparison_ne() {
        let scan = scan_generator("!=");
        assert_eq!(scan, vec!(Token::NE));
    }

    #[test]
    fn comparison_le() {
        let scan = scan_generator("<=");
        assert_eq!(scan, vec!(Token::LE));
    }

    #[test]
    fn comparison_ge() {
        let scan = scan_generator(">=");
        assert_eq!(scan, vec!(Token::GE));
    }

    #[test]
    fn comparison_lt() {
        let scan = scan_generator("<");
        assert_eq!(scan, vec!(Token::LT));
    }

    #[test]
    fn comparison_gt() {
        let scan = scan_generator(">");
        assert_eq!(scan, vec!(Token::GT));
    }

}
