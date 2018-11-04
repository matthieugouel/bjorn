use std::collections::HashMap;

use token::Token;
use parser::Parser;
use ast::AST;
use memory::Memory;
use value::Value;


pub struct Interpreter<'a> {
    parser: Parser<'a>,
    memory: Memory,
}

impl<'a> Interpreter<'a> {

    pub fn new(parser: Parser<'a>) -> Interpreter<'a>  {
        Interpreter { parser: parser, memory: Memory::new(HashMap::new()) }
    }

    fn visit(&mut self, tree: AST) -> Value {
        match tree {
            AST::Program {children} => {
                // Return the value the last child mostly for testing purposes.
                // Will be replaced by `print` in the future.
                let mut result = Value::None;

                for child in children {
                    result = self.visit(*child);
                }
                result
            },
            AST::Bloc {children} => {
                let mut result = Value::None;
                for child in children {
                    self.visit(*child);
                }
                result
            },
            AST::IfStatement {if_compound, else_if_compounds, else_compound} => {
                let (if_condition, if_bloc) = if_compound;
                if self.visit(*if_condition.clone()) == Value::Bool(true) {
                    self.visit(*if_bloc.clone());
                    Value::None
                } else {
                    for else_if_compound in &else_if_compounds {
                        let (else_if_condition, else_if_bloc) = else_if_compound;
                        if self.visit(*else_if_condition.clone()) == Value::Bool(true) {
                            self.visit(*else_if_bloc.clone());
                            return Value::None
                        }
                    }
                    self.visit(*else_compound)
                }
            },
            AST::WhileStatement {condition, bloc} => {
                loop {
                    if self.visit(*condition.clone()) == Value::Bool(true) {
                        self.visit(*bloc.clone());
                    } else {
                        break;
                    }
                }
                Value::None
            },
            AST::Assignment {left, right} => {
                let variable_name = match *left {
                    AST::Variable{id} => id.identifier().unwrap(),
                    _ => panic!("Interpreter error."),
                };
                let variable_value = self.visit(*right);

                self.memory.insert(variable_name, variable_value);
                Value::None
            },
            AST::BinaryOperation {left, op, right} => {
                if op == Token::PLUS {
                    self.visit(*left) + self.visit(*right)
                } else if op == Token::MINUS {
                    self.visit(*left) - self.visit(*right)
                } else if op == Token::MUL {
                    self.visit(*left) * self.visit(*right)
                } else if op == Token::DIV {
                    self.visit(*left) / self.visit(*right)
                } else if op == Token::EQ {
                    Value::Bool(self.visit(*left) == self.visit(*right))
                } else if op == Token::NE {
                    Value::Bool(self.visit(*left) != self.visit(*right))
                } else if op == Token::LE {
                    Value::Bool(self.visit(*left) <= self.visit(*right))
                } else if op == Token::GE {
                    Value::Bool(self.visit(*left) >= self.visit(*right))
                } else if op == Token::LT {
                    Value::Bool(self.visit(*left) < self.visit(*right))
                } else if op == Token::GT {
                    Value::Bool(self.visit(*left) > self.visit(*right))
                } else if op == Token::OR {
                    match (self.visit(*left), self.visit(*right)) {
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                        (_, _) => panic!("Invalid operation."),
                    }
                } else if op == Token::AND {
                    match (self.visit(*left), self.visit(*right)) {
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                        (_, _) => panic!("Invalid operation."),
                    }
                } else {
                    panic!("Interpreter error.")
                }
            },
            AST::UnaryOperation {op, right} => {
                if op == Token::PLUS {
                    self.visit(*right)
                } else if op == Token::MINUS {
                    -self.visit(*right)
                } else if op == Token::NOT {
                    !self.visit(*right)
                } else {
                    panic!("Interpreter error.")
                }
            },
            AST::IntNumber {token} => {
                Value::Int(token.integer().unwrap())
            },
            AST::FloatNumber {token} => {
                Value::Float(token.float().unwrap())
            },
            AST::Boolean {token} => {
                Value::Bool(token.boolean().unwrap())
            },
            AST::Variable {id} => {
                let variable_name = id.identifier().unwrap();
                let buf = self.memory.get(variable_name);
                if let Some(variable_value) = buf {
                    // This thing works thanks to the `Copy` trait added on `Value` enum.
                    // Not sure if it's the best way to handle this but it works for now.
                    *variable_value
                } else {
                    panic!("Interpreter error.")
                }
            },
            _ => Value::None
        }
    }

    pub fn interpret(&mut self) -> Value {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}
