use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
    None,
}

impl Value {

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(a) => a.to_string(),
            Value::Float(a) => a.to_string(),
            Value::None => "".to_string(),
            Value::Bool(true) => String::from("true"),
            Value::Bool(false) => String::from("false"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(a) => write!(f, "{}", a),
            Value::Float(a) => write!(f, "{}", a),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::None => write!(f, ""),
        }

    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a + b as f64),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 - b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a - b as f64),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a * b as f64),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Float(a as f64 / b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Int(a) => Value::Int(-a),
            Value::Float(a) => Value::Float(-a),
            Value::Bool(_) => panic!("Invalid operation."),
            Value::None => panic!("Invalid operation."),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => &(*a as f64) == b,
            (Value::Float(a), Value::Int(b)) => a == &(*b as f64),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}

impl PartialOrd for Value {

    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Bool(_), Value::Int(_)) => panic!("Invalid operation."),
            (Value::Int(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Float(_)) => panic!("Invalid operation."),
            (Value::Float(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::Bool(_), Value::Bool(_)) => panic!("Invalid operation."),
            (Value::None, _) => panic!("Invalid operation."),
            (_, Value::None) =>panic!("Invalid operation."),
        }
    }
}
