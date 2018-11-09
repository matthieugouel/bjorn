use std::collections::HashMap;
use std::collections::hash_map::Entry;

use token::Token;
use parser::Parser;
use ast::AST;
use memory::Memory;
use value::Value;


pub struct Interpreter<'a> {
    parser: Parser<'a>,
    pub memory: Memory,
}

impl<'a> Interpreter<'a> {

    pub fn new(parser: Parser<'a>) -> Interpreter<'a>  {
        Interpreter { parser: parser, memory: Memory::new(HashMap::new()) }
    }

    fn load_functions(&mut self, tree: AST) {
        match tree {
            AST::Program {children} => {
                for child in children {
                    self.load_functions(*child);
                }
            },
            AST::FunctionDeclaration {identifier, parameters, body} => {
                let function_name = identifier.identifier().unwrap();
                self.memory.insert(function_name,
                    Value::Function(AST::FunctionDeclaration {identifier, parameters, body}));
            },
            _ => {}
        }
    }

    fn visit(&mut self, tree: AST) -> Value {
        match tree {
            AST::Program {children} => {
                // Return the value the last child mostly for testing purposes.
                // Will be replaced by `print` in the future.
                let mut result = Value::None;

                for child in children {
                    match *child {
                        AST::FunctionDeclaration {..} => {},
                        _ => {
                            result = self.visit(*child);
                        }
                    };
                }
                result
            },
            AST::FunctionDeclaration {identifier: _, parameters, body} => {
                for (i, parameter) in parameters.into_iter().enumerate() {
                    let parameter_name = match parameter {
                        AST::Parameter {parameter} => match *parameter {
                            AST::Variable {id} => id.identifier().unwrap(),
                            _ => panic!("Interpreter error.")
                        }
                        _ => panic!("Interpreter Error.")
                    };
                    let parameter_value = match self.memory.remove(i.to_string()) {
                        Some(value) => value,
                        None => panic!("Interpreter error.")
                    };
                    match self.memory.current_scope_mut().unwrap().entry(parameter_name) {
                        Entry::Occupied(_) => panic!("Interpreter error."),
                        Entry::Vacant(v) => v.insert(parameter_value)
                    };
                }
                self.visit(*body)
            },
            AST::Parameter {parameter} => {
                self.visit(*parameter)
            },
            AST::IfStatement {if_compound, else_if_compounds, else_compound} => {
                let (if_condition, if_bloc) = if_compound;
                if self.visit(*if_condition.clone()) == Value::Bool(true) {
                    self.visit(*if_bloc.clone())
                } else {
                    for else_if_compound in &else_if_compounds {
                        let (else_if_condition, else_if_bloc) = else_if_compound;
                        if self.visit(*else_if_condition.clone()) == Value::Bool(true) {
                            let result_else_if_bloc = self.visit(*else_if_bloc.clone());
                            match result_else_if_bloc {
                                Value::None => {},
                                _ => return result_else_if_bloc
                            };
                        }
                    }
                    self.visit(*else_compound)
                }
            },
            AST::WhileStatement {condition, bloc} => {
                loop {
                    if self.visit(*condition.clone()) == Value::Bool(true) {
                        let result_loop = self.visit(*bloc.clone());
                        match result_loop {
                            Value::None => {},
                            _ => return result_loop
                        };
                    } else {
                        break;
                    }
                }
                Value::None
            },
            AST::Bloc {children} => {
                let mut result = Value::None;
                for child in children {
                    match *child {
                        AST::ReturnStatement {..} => {
                            result = self.visit(*child);
                            break;
                        },
                        AST::IfStatement{..} => {
                            result = self.visit(*child);
                            match result {
                                Value::None => {},
                                _ => return result
                            };
                        },
                        AST::WhileStatement{..} => {
                            result = self.visit(*child);
                            match result {
                                Value::None => {},
                                _ => return result
                            };
                        },
                        _ => {
                            self.visit(*child);
                        },
                    };
                }
                result
            },
            AST::ReturnStatement {expression} => {
                self.visit(*expression)
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
            AST::BinaryOperation {left, op, right} => { // TODO Try to use `match` statement
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
            AST::FunctionCall {identifier, arguments} => {
                let function_name = identifier.identifier().unwrap();
                let function_ast = match self.memory.get(function_name) {
                    Some(Value::Function(ast)) => {
                        ast.clone()
                    },
                    _ => panic!("Interpreter Error.")
                };

                match function_ast {
                    AST::FunctionDeclaration {identifier: _, ref parameters, ..} => {
                        if arguments.len() != parameters.len() {
                            panic!("Interpreter error.")
                        }
                    },
                    _ => panic!("Interpreter error.")
                }

                let mut hash_map_arguments = HashMap::new();
                for (i, argument) in arguments.iter().enumerate() {
                    let argument_value = self.visit(argument.clone());
                    hash_map_arguments.insert(i.to_string(), argument_value);
                }

                self.memory.push_scope(hash_map_arguments);
                self.load_functions(function_ast.clone());
                // println!("{:?}", self.memory.current_scope().unwrap());
                let function_result = self.visit(function_ast);
                self.memory.pop_scope();
                function_result
            },
            AST::Variable {id} => {
                let variable_name = id.identifier().unwrap();
                let buf = self.memory.get(variable_name);
                if let Some(variable_value) = buf {
                    variable_value.clone()
                } else {
                    panic!("Interpreter error.")
                }
            },
            _ => Value::None
        }
    }

    pub fn interpret(&mut self) -> Value {
        let tree = self.parser.parse();
        self.load_functions(tree.clone());
        self.visit(tree)
    }
}
