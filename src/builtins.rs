use std::collections::HashMap;
use value::Value;

pub type Callback = fn(Value) -> Value;

fn _print(_arg: Value) -> Value {
    println!("{}", _arg);
    Value::None
}


pub struct BuiltinsHandler {
    pub builtins: HashMap<String, Value>,
}


impl BuiltinsHandler {

    pub fn new() -> BuiltinsHandler {
        BuiltinsHandler {builtins: HashMap::new()}
    }

    pub fn register_builtins(&mut self) {
        self.builtins.insert(String::from("print"), Value::BuiltinFunction(_print));
    }
}
