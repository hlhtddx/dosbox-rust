use std::fs::File;
use serde_json::Map;

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
    name: String,
    properties: Vec<Property>,
    commands: Vec<String>,
}

pub struct Config {
    sections: Vec<Section>,
}

impl Property {
    fn new(obj: &Map<String, Value>) -> &Self {
        let new_item = Property {
            name: "".to_string(),
            value: Value::Bool(true),
            default_value: Value::Bool(true),
            suggested_values: vec![],
            change: Changeable::OnlyAtStart,
            min: 0,
            max: 0
        };
        &new_item
    }
    pub fn check_value(&self) -> bool {
        true
    }
}

impl Section {
    fn new(obj: &Map<String, Value>) -> &Self {
        let new_item = Section {
            name: String::from("123"),
            properties: vec![],
            commands: vec![]
        };
        &new_item
    }
}

impl Config {
    pub fn config() {
        let f = File::open("res/config.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        let s = v.as_object().unwrap();
        println!("config.json = {}", s.get("sections").unwrap());
    }

}
