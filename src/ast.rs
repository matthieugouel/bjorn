use token::Token;

#[derive(Debug, PartialEq)]
pub enum AST {
    Program {children: Vec<Box<AST>>},
    Assignment {left: Box<AST>, right: Box<AST>},
    Variable {id: Token},
    BinaryOperation {left: Box<AST>, op: Token, right: Box<AST>},
    UnaryOperation {op: Token, right: Box<AST>},
    IntNumber {token: Token},
    FloatNumber {token: Token},
    Empty,
}
