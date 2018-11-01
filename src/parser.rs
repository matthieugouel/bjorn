use std::iter::Peekable;
use std::iter::Flatten;

use lexer::Lexer;
use token::Token;
use ast:: AST;

pub struct Parser<'a> {
    lexer: Peekable<Flatten<Lexer<'a>>>,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: Lexer<'a>) -> Parser<'a>  {
        Parser { lexer: lexer.flatten().peekable()}
    }

    fn peek(&mut self) -> &Token {
        self.lexer.peek().unwrap_or(&Token::EOF)
    }

    fn process(&mut self, verification: Option<Token>) -> Token {
        let next = self.lexer.next().unwrap_or(Token::EOF);
        match verification {
            Some(verification) => {
                if next == verification {
                    next
                } else {
                    panic!("Parser error.")
                }
            },
            None => {
                next
            }
        }
    }

    /// program: (NEWLINE | statement)* EOF
    fn program(&mut self) -> AST {
        let mut children = Vec::new();
        while *self.peek() != Token::EOF {
            if *self.peek() == Token::NEWLINE {
                self.process(Some(Token::NEWLINE));
            } else {
                children.push(Box::new(self.statement()));
            }
        }
        AST::Program {children: children}
    }

    /// statement: compound_statement
    ///          | simple_statement
    fn statement(&mut self) -> AST {
        if *self.peek() == Token::IF
        || *self.peek() == Token::WHILE {
            self.compound_statement()
        } else {
            self.simple_statement()
        }
    }

    /// compound_statement: if_statement
    ///                   | while_statement
    fn compound_statement(&mut self) -> AST {
        if *self.peek() == Token::IF {
            self.if_statement()
        } else if *self.peek() == Token::WHILE {
            self.while_statement()
        } else {
            panic!("Parser error.")
        }
    }

    /// if_statement: 'if' logical_or_expr ':' bloc ('else' 'if' logical_or_expr ':' bloc)* ['else' ':' bloc]
    fn if_statement(&mut self) -> AST {
        self.process(Some(Token::IF));
        let if_condition = self.logical_or_expr();
        self.process(Some(Token::COLON));
        let if_bloc = self.bloc();

        let mut else_if_compounds: Vec<(Box<AST>, Box<AST>)> = Vec::new();
        let mut else_compound: Box<AST> = Box::new(AST::Empty);
        while *self.peek() == Token::ELSE {
            self.process(Some(Token::ELSE));
            if *self.peek() == Token::IF {
                self.process(Some(Token::IF));
                let else_if_condition = self.logical_or_expr();
                self.process(Some(Token::COLON));
                let else_if_bloc = self.bloc();
                else_if_compounds.push(
                    (Box::new(else_if_condition), Box::new(else_if_bloc))
                );
            } else {
                self.process(Some(Token::COLON));
                else_compound = Box::new(self.bloc());
            }
        }
        AST::IfStatement {
            if_compound: (Box::new(if_condition), Box::new(if_bloc)),
            else_if_compounds: else_if_compounds,
            else_compound: else_compound,
        }
    }

    /// while_statement: 'while' logical_or_expr ':' bloc
    fn while_statement(&mut self) -> AST {
        self.process(Some(Token::WHILE));
        let while_condition = self.logical_or_expr();
        self.process(Some(Token::COLON));
        let while_bloc = self.bloc();
        AST::WhileStatement {
            condition: Box::new(while_condition),
            bloc: Box::new(while_bloc),
        }
    }

    /// bloc: NEWLINE INDENT statement+ DEDENT
    fn bloc(&mut self) -> AST {
        self.process(Some(Token::NEWLINE));
        self.process(Some(Token::INDENT));
        let mut children = Vec::new();
        while *self.peek() != Token::DEDENT {
            children.push(Box::new(self.statement()));
        }
        self.process(Some(Token::DEDENT));
        AST::Bloc {children:children}
    }

    /// simple_statement: expression_statement NEWLINE
    fn simple_statement(&mut self) -> AST {
        let node = self.expression_statement();
        if *self.peek() != Token::EOF {
            self.process(Some(Token::NEWLINE));
        }
        node
    }

    /// expression_statement: logical_or_expr ['=' logical_or_expr]
    fn expression_statement(&mut self) -> AST {
        let mut node = self.logical_or_expr();
        if *self.peek() == Token::ASSIGN {
            self.process(Some(Token::ASSIGN));
            let right = self.logical_or_expr();
            node = AST::Assignment {
                left: Box::new(node), right: Box::new(right)
            };
        }
        node
    }

    /// logical_or_expr: logical_and_expr ('or' logical_and_expr)*
    fn logical_or_expr(&mut self) -> AST {
        let mut node = self.logical_and_expr();
        loop {
            if *self.peek() == Token::OR {
                let op = self.process(Some(Token::OR));
                let right = self.logical_and_expr();
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

    /// logical_and_expr: logical_not_expr ('and' logical_not_expr)*
    fn logical_and_expr(&mut self) -> AST {
        let mut node = self.logical_not_expr();
        loop {
            if *self.peek() == Token::AND {
                let op = self.process(Some(Token::AND));
                let right = self.logical_not_expr();
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

    /// logical_not_expr: 'not' logical_not_expr
    ///                 | comparison
    fn logical_not_expr(&mut self) -> AST {
        if *self.peek() == Token::NOT {
            let op = self.process(Some(Token::NOT));
            let right = self.logical_not_expr();
            AST::UnaryOperation {op: op, right: Box::new(right)}
        } else {
            self.comparison()
        }
    }

    /// comparison: expr (('==' | '!=' | '<=' | '>=' | '<' | '>') expr)*
    fn comparison(&mut self) -> AST {
        let mut node = self.expr();
        loop {
            let op: Token;
            if *self.peek() == Token::EQ {
                op = self.process(Some(Token::EQ));
            } else if *self.peek() == Token::NE {
                op = self.process(Some(Token::NE));
            } else if *self.peek() == Token::LE {
                op = self.process(Some(Token::LE));
            } else if *self.peek() == Token::GE {
                op = self.process(Some(Token::GE));
            } else if *self.peek() == Token::LT {
                op = self.process(Some(Token::LT));
            } else if *self.peek() == Token::GT {
                op = self.process(Some(Token::GT));
            } else {
                break;
            }
            let right = self.term();
            node = AST::BinaryOperation {
                left: Box::new(node),
                op: op,
                right: Box::new(right)
            }
        }
        node
    }

    /// expr: term (('+' | '-') term)*
    fn expr(&mut self) -> AST {
        let mut node = self.term();
        loop {
            let op: Token;
            if *self.peek() == Token::PLUS {
                op = self.process(Some(Token::PLUS));
            } else if *self.peek() == Token::MINUS {
                op = self.process(Some(Token::MINUS));
            } else {
                break;
            }
            let right = self.term();
            node = AST::BinaryOperation {
                left: Box::new(node),
                op: op,
                right: Box::new(right)
            }
        }
        node
    }

    /// term: atom (('*' | '/') atom)*
    fn term(&mut self) -> AST {
        let mut node = self.atom();
        loop {
            let op: Token;
            if *self.peek() == Token::MUL {
                op = self.process(Some(Token::MUL));
            } else if *self.peek() == Token::DIV {
                op = self.process(Some(Token::DIV));
            } else {
                break;
            }
            let right = self.atom();
            node = AST::BinaryOperation {
                left: Box::new(node),
                op: op,
                right: Box::new(right)
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
        let token = self.process(None);
        match token {
            Token::INT(_) => AST::IntNumber {token: token},
            Token::FLOAT(_) => AST::FloatNumber {token: token},
            Token::LPAREN => {
                let expr = self.expr();
                self.process(Some(Token::RPAREN));
                expr
            },
            Token::PLUS => {
                AST::UnaryOperation {op: token, right: Box::new(self.atom())}
            },
            Token::MINUS => {
                AST::UnaryOperation {op: token, right: Box::new(self.atom())}
            },
            Token::BOOL(value) => AST::Boolean {token: Token::BOOL(value)},
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
    fn while_statement() {
        let mut parser = parser_generator("while true:\n    1\n");
        assert_eq!(parser.parse(),
            AST::Program { children: vec![
                Box::new(AST::WhileStatement {
                    condition: Box::new(AST::Boolean { token: Token::BOOL(true) }),
                    bloc: Box::new(AST::Bloc { children:
                        vec![Box::new(AST::IntNumber { token: Token::INT(String::from("1")) })]
                    })
                })]
            }
        );
    }

    #[test]
    fn if_statement() {
        let mut parser = parser_generator("if true:\n    a = 1\nelse:\n    a = 2\n");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::IfStatement {
                    if_compound: (
                        Box::new(AST::Boolean {token: Token::BOOL(true)}),
                        Box::new(AST::Bloc {children:
                            vec![Box::new(AST::Assignment {
                                left: Box::new(AST::Variable {id: Token::ID(String::from("a"))}),
                                right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                            })]
                        })
                    ),
                    else_if_compounds: Vec::new(),
                    else_compound: Box::new(AST::Bloc {children:
                        vec![Box::new(AST::Assignment {
                            left: Box::new(AST::Variable {id: Token::ID(String::from("a"))}),
                            right: Box::new(AST::IntNumber {token: Token::INT(String::from("2"))}),
                        })]
                    })
                })
            )}
        );
    }

    #[test]
    fn assignment_statement() {
        let mut parser = parser_generator("a = 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::Assignment {
                    left: Box::new(AST::Variable {id: Token::ID(String::from("a"))}),
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn logical_or_operation() {
        let mut parser = parser_generator("true or true");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::Boolean {token: Token::BOOL(true)}),
                    op: Token::OR,
                    right: Box::new(AST::Boolean {token: Token::BOOL(true)}),
                })
            )}
        );
    }

    #[test]
    fn logical_and_operation() {
        let mut parser = parser_generator("true and true");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::Boolean {token: Token::BOOL(true)}),
                    op: Token::AND,
                    right: Box::new(AST::Boolean {token: Token::BOOL(true)}),
                })
            )}
        );
    }

    #[test]
    fn logical_not_operation() {
        let mut parser = parser_generator("not true");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::UnaryOperation {
                    op: Token::NOT,
                    right: Box::new(AST::Boolean {token: Token::BOOL(true)}),
                })
            )}
        );
    }

    #[test]
    fn comparison_eq() {
        let mut parser = parser_generator("1 == 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::EQ,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn comparison_ne() {
        let mut parser = parser_generator("1 != 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::NE,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn comparison_le() {
        let mut parser = parser_generator("1 <= 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::LE,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn comparison_ge() {
        let mut parser = parser_generator("1 >= 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::GE,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn comparison_lt() {
        let mut parser = parser_generator("1 < 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::LT,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn comparison_gt() {
        let mut parser = parser_generator("1 > 1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::BinaryOperation {
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::GT,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
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
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::PLUS,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
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
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::MINUS,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
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
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::MUL,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
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
                    left: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                    op: Token::DIV,
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))}),
                })
            )}
        );
    }

    #[test]
    fn atom_int_number() {
        let mut parser = parser_generator("1");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::IntNumber {token: Token::INT(String::from("1"))})
            )}
        );
    }

    #[test]
    fn atom_float_number() {
        let mut parser = parser_generator("1.0");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::FloatNumber {token: Token::FLOAT(String::from("1.0"))})
            )}
        );
    }

    #[test]
    fn atom_parenthesis() {
        let mut parser = parser_generator("(1)");
        assert_eq!(parser.parse(),
            AST::Program { children: vec!(
                Box::new(AST::IntNumber {token: Token::INT(String::from("1"))})
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
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))})
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
                    right: Box::new(AST::IntNumber {token: Token::INT(String::from("1"))})
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
                Box::new(AST::Variable {id: Token::ID(String::from("a"))})
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
