use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Program {children: Vec<Box<AST>>},
    Bloc {children: Vec<Box<AST>>},
    IfStatement {if_compound: (Box<AST>, Box<AST>), else_if_compounds: Vec<(Box<AST>, Box<AST>)>, else_compound: Box<AST>},
    WhileStatement {condition: Box<AST>, bloc: Box<AST>},
    FunctionDeclaration {identifier: Token, parameters: Vec<AST>, body: Box<AST>},
    Parameter {parameter: Box<AST>},
    ReturnStatement {expression: Box<AST>},
    Assignment {left: Box<AST>, right: Box<AST>},
    BinaryOperation {left: Box<AST>, op: Token, right: Box<AST>},
    UnaryOperation {op: Token, right: Box<AST>},
    IntNumber {token: Token},
    FloatNumber {token: Token},
    Boolean {token: Token},
    FunctionCall {identifier: Token, arguments: Vec<AST>},
    Variable {id: Token},
    Empty,
}
