use std::fs::File;

enum Changeable {
    OnlyAtStart,
    Always,
    WhenIdle,
}

enum Value {
    Hex(String),
    Bool(bool),
    Int(i32),
    Str(String),
    Double(f64),
}

pub struct Property {
    name: String,
    value: Value,
    default_value: Value,
    suggested_values: Vec<Value>,
    change: Changeable,
    min: i32,
    max: i32,
}

pub struct Section {

}

impl Property {
    pub fn check_value(&self) -> bool {
        true
    }
}

pub fn config() -> u32 {
    let f = File::open("res/config.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    println!("config.json = {}", v);
    1
}
