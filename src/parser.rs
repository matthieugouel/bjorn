use std::iter::Peekable;

use lexer::Lexer;
use token::Token;
use ast:: AST;

#[derive(Debug)]
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

    /// program: expression_statement
    fn program(&mut self) -> AST {
        let mut children = Vec::new();
        while *self.peek() != Token::EOF {
            children.push(Box::new(self.expr()));
        }
        AST::Program {children: children}
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
            Token::EOF => AST::Empty,
            _ => panic!("Syntax error."),
        }
    }

    pub fn parse(&mut self) -> AST {
        self.program()
    }
}
