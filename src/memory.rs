use std::collections::HashMap;

use value::Value;


#[derive(Debug)]
pub struct Memory {
    stack: Vec<Vec<HashMap<String, Value>>>,
}


impl Memory {
    pub fn new(init: HashMap<String, Value>) -> Memory {
        Memory {stack: vec![vec![init]]}
    }

    pub fn get(&self, key: String) -> Option<&Value> {
        (*self.stack.last().unwrap()).last().unwrap().get(&key)
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        (*self.stack.last_mut().unwrap()).last_mut().unwrap().insert(key, value)
    }
}
