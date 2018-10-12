use std::iter::Peekable;

use lexer::Lexer;
use token::Token;
use ast:: AST;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: Lexer<'a>) -> Parser<'a>  {
        Parser { lexer: lexer.peekable() }
    }

    fn peek(&mut self) -> &Token {
        self.lexer.peek().unwrap_or(&Token::EOF)
    }

    fn process(&mut self) -> Token {
        self.lexer.next().unwrap_or(Token::EOF)
    }

    /// program: NEWLINE
    ///        | expression_statement
    fn program(&mut self) -> AST {
        let mut children = Vec::new();
        while *self.peek() != Token::EOF {
            if *self.peek() == Token::NEWLINE {
                self.process();
            } else {
                children.push(Box::new(self.expression_statement()));
            }
        }
        AST::Program {children: children}
    }

    /// expression_statement: expr ['=' expr]
    fn expression_statement(&mut self) -> AST {
        let mut node = self.expr();
        if *self.peek() == Token::ASSIGN {
            self.process();
            let right = self.expr();
            node = AST::Assignment {
                left: Box::new(node), right: Box::new(right)
            };
        }
        node
    }

    /// expr: term (('+' | '-') term)*
    fn expr(&mut self) -> AST {
        let mut node = self.term();
        loop {
            if *self.peek() == Token::PLUS
            || *self.peek() == Token::MINUS {
                let op = self.process();
                let right = self.term();
                node = AST::BinaryOperation {
                    left: Box::new(node),
                    op: op,
                    right: Box::new(right)
                }
            } else {
                break;
            }
        }
        node
    }

    /// term: atom (('*' | '/') atom)*
    fn term(&mut self) -> AST {
        let mut node = self.atom();
        loop {
            if *self.peek() == Token::MUL
            || *self.peek() == Token::DIV {
                let op = self.process();
                let right = self.atom();
                node = AST::BinaryOperation {
                    left: Box::new(node),
                    op: op,
                    right: Box::new(right)
                }
            } else {
                break;
            }
        }
        node
    }

    /// atom: INT_NUMBER
    ///     | FLOAT_NUMBER
    ///     | PLUS atom
    ///     | MINUS atom
    ///     | '(' expr ')'
    ///     | TRUE
    ///     | FALSE
    ///     | variable
    fn atom (&mut self) -> AST {
        let token = self.process();
        match token {
            Token::INT(_) => AST::IntNumber {token: token},
            Token::FLOAT(_) => AST::FloatNumber {token: token},
            Token::LPAREN => {
                let expr = self.expr();
                self.process();
                expr
            },
            Token::PLUS => {
                AST::UnaryOperation {op: token, right: Box::new(self.atom())}
            },
            Token::MINUS => {
                AST::UnaryOperation {op: token, right: Box::new(self.atom())}
            },
            Token::BOOL(true) => AST::Boolean {token: token},
            Token::BOOL(false) => AST::Boolean {token: token},
            Token::ID(_) => AST::Variable {id: token},
            Token::EOF => AST::Empty,
            _ => panic!("Syntax error."),
        }
    }

    pub fn parse(&mut self) -> AST {
        self.program()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lexer::Lexer;
    use token::Token;
    use ast::AST;

    fn parser_generator(input: &str) -> Parser {
        Parser::new(
            Lexer::new(input)
        )
    }

    #[test]
    fn assignment_statement() {
        let mut parser = parser_generator("a = 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::Assignment {
                    left: Box::new(AST::Variable {id: Token::ID("a".to_string())}),
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                })
            )}
        );
    }

    #[test]
    fn term_plus_operation() {
        let mut parser = parser_generator("1 + 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                    op: Token::PLUS,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                })
            )}
        );
    }

    #[test]
    fn term_minus_operation() {
        let mut parser = parser_generator("1 - 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                    op: Token::MINUS,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                })
            )}
        );
    }

    #[test]
    fn term_mul_operation() {
        let mut parser = parser_generator("1 * 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                    op: Token::MUL,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                })
            )}
        );
    }

    #[test]
    fn term_div_operation() {
        let mut parser = parser_generator("1 / 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                    op: Token::DIV,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())}),
                })
            )}
        );
    }

    #[test]
    fn atom_int_number() {
        let mut parser = parser_generator("1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::IntNumber {token: Token::INT("1".to_string())})
            )}
        );
    }

    #[test]
    fn atom_float_number() {
        let mut parser = parser_generator("1.0");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::FloatNumber {token: Token::FLOAT("1.0".to_string())})
            )}
        );
    }

    #[test]
    fn atom_parenthesis() {
        let mut parser = parser_generator("(1)");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::IntNumber {token: Token::INT("1".to_string())})
            )}
        );
    }

    #[test]
    fn atom_plus_unary() {
        let mut parser = parser_generator("+1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::UnaryOperation {
                    op: Token::PLUS,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())})
                })
            )}
        );
    }

    #[test]
    fn atom_minus_unary() {
        let mut parser = parser_generator("-1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::UnaryOperation {
                    op: Token::MINUS,
                    right: Box::new(AST::IntNumber {token: Token::INT("1".to_string())})
                })
            )}
        );
    }

    #[test]
    fn atom_boolean_true() {
        let mut parser = parser_generator("true");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::Boolean {token: Token::BOOL(true)})
            )}
        );
    }

    #[test]
    fn atom_boolean_false() {
        let mut parser = parser_generator("false");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::Boolean {token: Token::BOOL(false)})
            )}
        );
    }

    #[test]
    fn atom_variable() {
        let mut parser = parser_generator("a");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::Variable {id: Token::ID("a".to_string())})
            )}
        );
    }

    #[test]
    fn one_newline() {
        let mut parser = parser_generator("\n");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!()}
        );
    }

    #[test]
    fn multiple_newlines() {
        let mut parser = parser_generator("\n\n\n");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!()}
        );
    }

    #[test]
    fn empty() {
        let mut parser = parser_generator("");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!()}
        );
    }
}
