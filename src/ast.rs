use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Program {children: Vec<Box<AST>>},
    Bloc {children: Vec<Box<AST>>},
    IfStatement {if_compound: (Box<AST>, Box<AST>), else_if_compounds: Vec<(Box<AST>, Box<AST>)>, else_compound: Box<AST>},
    WhileStatement {condition: Box<AST>, bloc: Box<AST>},
    Assignment {left: Box<AST>, right: Box<AST>},
    Variable {id: Token},
    BinaryOperation {left: Box<AST>, op: Token, right: Box<AST>},
    UnaryOperation {op: Token, right: Box<AST>},
    IntNumber {token: Token},
    FloatNumber {token: Token},
    Boolean {token: Token},
    Empty,
}
