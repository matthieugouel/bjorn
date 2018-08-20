use token::Token;

#[derive(Debug)]
pub enum AST {
    IntNumber {token: Token},
    FloatNumber {token: Token},
    BinaryOperation {left: Box<AST>, op: Token, right: Box<AST>},
}
