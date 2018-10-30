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

    fn indent (&mut self) -> Option<Vec<Token>> {
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
                    return Some(vec![Token::NEWLINE])
                } else if indent_count > self.indent_level {
                    // At least one additional identation
                    let mut indent_array: Vec<Token> = vec![Token::NEWLINE];
                    for _ in 0..(indent_count - self.indent_level) {
                        self.indent_level += 1;
                        indent_array.push(Token::INDENT);
                    }
                    return Some(indent_array)
                } else {
                    // At least one indentation in less
                    let mut dedent_array: Vec<Token> = vec![];
                    for _ in 0..(self.indent_level - indent_count) {
                        self.indent_level -= 1;
                        dedent_array.push(Token::DEDENT);
                    }
                    dedent_array.push(Token::NEWLINE);
                    return Some(dedent_array)
                }
            } else {
                spaces_count += 1;
                self.advance();
            }
        }
        Some(vec![Token::NEWLINE])
    }

    fn number(&mut self, number: &str) -> Option<Vec<Token>> {
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
                return Some(vec![Token::FLOAT(number)]);
            }
            if !NUMERIC.is_match(c) {
                break;
            }
            number.push_str(self.advance());
        }
        Some(vec![Token::INT(number)])
    }

    fn id(&mut self, id: &str) -> Option<Vec<Token>> {
        let mut id = id.to_string();
        while let Some(&c) = self.input.peek() {
            if !WORD.is_match(c) {
                break;
            }
            id.push_str(self.advance());
        }
        // Reserved keywords
        match id.as_ref() {
            "true" => Some(vec![Token::BOOL(true)]),
            "false" => Some(vec![Token::BOOL(false)]),
            "or" => Some(vec![Token::OR]),
            "and" => Some(vec![Token::AND]),
            "not" => Some(vec![Token::NOT]),
            _ => Some(vec![Token::ID(id)])
        }
    }

    fn comment (&mut self) -> Option<Vec<Token>> {
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
    type Item = Vec<Token>;

    fn next(&mut self) -> Option<Self::Item> {

        self.whitespace();

         match self.input.next() {
            Some(c) if NUMERIC.is_match(c) => self.number(c),
            Some(c) if ALPHABETIC.is_match(c) => self.id(c),
            Some("\n") => self.indent(),
            Some("=") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(vec![Token::EQ])
                } else {
                    Some(vec![Token::ASSIGN])
                }

            },
            Some("!") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(vec![Token::NE])
                } else {
                    panic!("Lexical error.") // Lexeme `!` is not supported
                }
            }
            Some("<") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(vec![Token::LE])
                } else {
                    Some(vec![Token::LT])
                }
            },
            Some(">") => {
                if self.input.peek() == Some(&"=") {
                    self.advance();
                    Some(vec![Token::GE])
                } else {
                    Some(vec![Token::GT])
                }
            },
            Some("+") => Some(vec![Token::PLUS]),
            Some("-") => Some(vec![Token::MINUS]),
            Some("*") => Some(vec![Token::MUL]),
            Some("/") => Some(vec![Token::DIV]),
            Some("(") => Some(vec![Token::LPAREN]),
            Some(")") => Some(vec![Token::RPAREN]),
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
        scan.into_iter().flatten().collect::<Vec<Token>>()
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
            Token::NEWLINE,
            Token::INDENT,
            Token::ID(String::from("b")),
            Token::NEWLINE,
            Token::ID(String::from("c")),
            Token::DEDENT,
            Token::NEWLINE,
            Token::ID(String::from("d")),
            ])
    }

    #[test]
    fn indentation_multiple() {
        let scan = scan_generator("a\n    b\n        c\nd");
        assert_eq!(scan, vec![
            Token::ID(String::from("a")),
            Token::NEWLINE,
            Token::INDENT,
            Token::ID(String::from("b")),
            Token::NEWLINE,
            Token::INDENT,
            Token::ID(String::from("c")),
            Token::DEDENT,
            Token::DEDENT,
            Token::NEWLINE,
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
        assert_eq!(scan, vec!(Token::INT(String::from("1"))));
    }

    #[test]
    fn float_number() {
        let scan = scan_generator("1.0");
        assert_eq!(scan, vec!(Token::FLOAT(String::from("1.0"))));
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
            Token::INT(String::from("1")),
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
        assert_eq!(scan, vec!(Token::ID(String::from("bjørn"))));
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
