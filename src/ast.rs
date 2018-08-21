use token::Token;

#[derive(Debug)]
pub enum AST {
    BinaryOperation {left: Box<AST>, op: Token, right: Box<AST>},
    IntNumber {token: Token},
    FloatNumber {token: Token},
    Empty,
}
