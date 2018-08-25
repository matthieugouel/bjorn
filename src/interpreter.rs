use token::Token;
use parser::Parser;
use ast::AST;
use value::Value;

#[derive(Debug)]
pub struct Interpreter<'a> {
    parser: Parser<'a>,
}

impl<'a> Interpreter<'a> {

    pub fn new(parser: Parser<'a>) -> Interpreter<'a>  {
        Interpreter { parser: parser }
    }

    fn visit(&self, tree: AST) -> Value {
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
            AST::Empty => Value::None
        }
    }

    pub fn interpret(&mut self) -> Value {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}
