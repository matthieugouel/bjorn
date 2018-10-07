use token::Token;
use parser::Parser;
use ast::AST;
use value::Value;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter<'a> {
    parser: Parser<'a>,
    pub memory: HashMap<String, Value>,
}

impl<'a> Interpreter<'a> {

    pub fn new(parser: Parser<'a>) -> Interpreter<'a>  {
        Interpreter { parser: parser, memory: HashMap::new() }
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
            }
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
                } else {
                    panic!("Interpreter error.")
                }
            },
            AST::UnaryOperation {op, right} => {
                if op == Token::PLUS {
                    self.visit(*right)
                } else if op == Token::MINUS {
                    -self.visit(*right)
                } else {
                    panic!("Interpreter error.")
                }
            }
            AST::IntNumber {token} => {
                Value::Int(token.integer().unwrap())
            },
            AST::FloatNumber {token} => {
                Value::Float(token.float().unwrap())
            },
            AST::Variable {id} => {
                let variable_name = id.identifier().unwrap();
                let buf = self.memory.get(&variable_name);
                if let Some(variable_value) = buf {
                    // This thing works thanks to the `Copy` trait
                    // added on `Value` enum.
                    // Not sure if it's the best way to handle this for now.
                    *variable_value
                } else {
                    panic!("Interpreter error.")
                }
            }
            _ => Value::None
        }
    }

    pub fn interpret(&mut self) -> Value {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}
