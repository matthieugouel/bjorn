use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(a) => write!(f, "{}", a),
            Value::Float(a) => write!(f, "{}", a),
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
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Float(a as f64 / b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a * b as f64),
        }
    }
}
