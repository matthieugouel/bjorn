use std::collections::HashMap;

use value::Value;


#[derive(Debug)]
pub struct Memory {
    stack: Vec<HashMap<String, Value>>,
}


impl Memory {
    pub fn new(init: HashMap<String, Value>) -> Memory {
        Memory {stack: vec![init]}
    }

    pub fn get(&self, key: String) -> Option<&Value> {
        self.stack.last().unwrap().get(&key)
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.stack.last_mut().unwrap().insert(key, value)
    }

    pub fn remove(&mut self, key: String) -> Option<Value> {
        self.stack.last_mut().unwrap().remove(&key)
    }

    pub fn push_scope(&mut self, init: HashMap<String, Value>) {
        self.stack.push(init)
    }

    pub fn pop_scope(&mut self) -> Option<HashMap<String, Value>> {
        self.stack.pop()
    }

    pub fn current_scope(&self) -> Option<&HashMap<String, Value>> {
        self.stack.last()
    }

    pub fn current_scope_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        self.stack.last_mut()
    }
}
