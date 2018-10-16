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
    indent_level: u8,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Lexer<'a>  {
        Lexer { input: UnicodeSegmentation::graphemes(input, true).peekable(),
                indent_level: 0,
            }
    }

    fn advance(&mut self) -> &str {
        match self.input.next() {
            Some(c) => c,
            None => panic!("Lexical error.")
        }
    }

    fn whitespace (&mut self)  {
        while let Some(&c) = self.input.peek() {
            if c != " " {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn indent (&mut self) -> Option<Token> {
        // For now at least, identation is forced to 4 spaces
        let spaces_for_indent = 4;

        let mut spaces_count = 0;
        while let Some(&c) = self.input.peek() {
            if c != " " {
                if spaces_count % spaces_for_indent != 0 {
                    panic!("Indentation error.")
                }
                let indent_count = spaces_count / spaces_for_indent;
                if indent_count == self.indent_level {
                    // Same level of indentation
                    return Some(Token::NEWLINE)
                } else if indent_count > self.indent_level {
                    // At least one additional identation
                    self.indent_level +=1;
                    return Some(Token::INDENT)
                } else {
                    // At least one indentation in less
                    self.indent_level -= 1;
                    return Some(Token::DEDENT)
                }
            } else {
                spaces_count += 1;
                self.advance();
            }
        }
        Some(Token::NEWLINE)
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
        // Reserved keywords
        match id.as_ref() {
            "true" => Some(Token::BOOL(true)),
            "false" => Some(Token::BOOL(false)),
            "or" => Some(Token::OR),
            "and" => Some(Token::AND),
            "not" => Some(Token::NOT),
            _ => Some(Token::ID(id))
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        self.whitespace();

         match self.input.next() {
            Some(c) if NUMERIC.is_match(c) => self.number(c),
            Some(c) if ALPHABETIC.is_match(c) => self.id(c),
            Some("\n") => self.indent(),
            Some("=") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(Token::EQ)
                } else {
                    Some(Token::ASSIGN)
                }

            },
            Some("!") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(Token::NE)
                } else {
                    panic!("Lexical error.") // Lexeme `!` is not supported
                }
            }
            Some("<") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(Token::LE)
                } else {
                    Some(Token::LT)
                }
            },
            Some(">") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
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

            // End of file
            None => None,

            // Not supported lexeme
            _ => panic!("Lexical error.")
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
    #[should_panic]
    fn invalid_input() {
        // Must change if `§` is valid one day
        scan_generator("§");
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
    fn indentation() {
        let scan = scan_generator("a\n    b\n    c\nd");
        assert_eq!(scan, vec![
            Token::ID(String::from("a")),
            Token::INDENT,
            Token::ID(String::from("b")),
            Token::NEWLINE,
            Token::ID(String::from("c")),
            Token::DEDENT,
            Token::ID(String::from("d")),
            ])
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

    #[test]
    fn logical_or_operation() {
        let scan = scan_generator("or");
        assert_eq!(scan, vec!(Token::OR));
    }

    #[test]
    fn logical_and_operation() {
        let scan = scan_generator("and");
        assert_eq!(scan, vec!(Token::AND));
    }

    #[test]
    fn logical_not_operation() {
        let scan = scan_generator("not");
        assert_eq!(scan, vec!(Token::NOT));
    }

}
