use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::{Map, Value};

#[derive(Debug)]
enum Changeable {
    OnlyAtStart,
    Always,
    WhenIdle,
}

#[derive(Debug, Clone)]
enum PropertyValue {
    Null,
    Path(String),
    Str(String),
    Hex(String),
    Bool(bool),
    Int(i64),
    Double(f64),
}

#[derive(Debug)]
pub struct Property {
    name: String,
    current_value: PropertyValue,
    default_value: PropertyValue,
    suggested_values: Vec<String>,
    changeable: Changeable,
    help_str: String,
    min: i64,
    max: i64,
}

#[derive(Debug)]
pub enum SectionType {
    PROPERTIES(Vec<Property>),
    LINES(Vec<String>),
}

#[derive(Debug)]
pub struct Section {
    name: String,
    section_type: SectionType,
}

#[derive(Debug)]
pub struct Config<'a> {
    sections: Vec<Section>,
    property_map: HashMap<String, &'a Property>,
}

impl Property {
    fn new(obj: (&String, &Value), value_list: &Map<String, Value>) -> Option<Self> {
        let name = obj.0.to_string();
        let value = obj.1;
        let prop_type = value["type"].as_str()?;
        let changeable: Changeable;
        let current_value: PropertyValue;
        let default_value: PropertyValue;
        let mut suggested_values: Vec<String> = vec![];
        let help_str = value["help"].as_str()?.to_string();
        let min: i64;
        let max: i64;

        match value["changeable"].as_str()? {
            "OnlyAtStart" => { changeable = Changeable::OnlyAtStart }
            "Always" => { changeable = Changeable::Always }
            "WhenIdle" => { changeable = Changeable::WhenIdle }
            &_ => { changeable = Changeable::Always }
        }

        match prop_type {
            "path" | "string" | "multi" => {
                default_value = PropertyValue::Path(value["default"].as_str()?.to_string());
                current_value = default_value.clone();
                min = 0;
                max = 0;
                match &value["values"] {
                    Value::String(_values) => {
                        for s in value_list[_values].as_array()? {
                            suggested_values.push(s.as_str()?.to_string())
                        }
                    }
                    Value::Array(_values) => {
                        for s in _values.iter() {
                            suggested_values.push(s.as_str()?.to_string())
                        }
                    }
                    Value::Null => {}
                    _values => { panic!("Invalid type for 'values' field {}, Only string or string list is allowed", _values) }
                }
            }
            "int" => {
                default_value = PropertyValue::Int(value["default"].as_i64()?);
                current_value = default_value.clone();
                let _values = value["values"].as_array()?;
                min = _values[0].as_i64()?;
                max = _values[1].as_i64()?;
            }
            "bool" => {
                default_value = PropertyValue::Bool(value["default"].as_bool()?);
                current_value = default_value.clone();
                min = 0;
                max = 0;
            }
            &_ => { panic!("Unknown property type {}", prop_type) }
        };

        Some(Property {
            name,
            changeable,
            help_str,
            current_value,
            default_value,
            suggested_values,
            min,
            max,
        })
    }
    pub fn check_value(&self) -> bool {
        true
    }
}

impl Section {
    fn new(obj: (&String, &Value), value_list: &Map<String, Value>) -> Option<Self> {
        let name = obj.0;
        let t = obj.1["type"].as_str()?;
        let section_type: SectionType;
        match t {
            "property" => {
                let mut props: Vec<Property> = vec![];
                let properties = obj.1["properties"].as_object()?;
                for obj in properties.iter() {
                    props.push(Property::new(obj, value_list)?);
                }
                section_type = SectionType::PROPERTIES(props)
            }
            "line" => {
                section_type = SectionType::LINES(vec![])
            }
            &_ => { panic!("Invalid section type, only 'property' and 'line' are allowed!") }
        }

        Some(Section {
            name: name.to_string(),
            section_type,
        })
    }
}

impl Config<'_> {
    fn load_json<'a>(&mut self, v: &Value) -> Option<&mut Config<'a>> {
        let value_list = v["value_list"].as_object()?;
        let items = v["sections"].as_object()?;
        let sections = &mut self.sections;
        let property_map = &mut self.property_map;

        for obj in items.iter() {
            let section = Section::new(obj, value_list)?;
            sections.push(section);
            match &section.section_type {
                SectionType::PROPERTIES(properties) => {
                    for property in properties.iter() {
                        let mut key = section.name.clone();
                        key.push_str(&property.name);
                        property_map.insert(key.to_string(), property);
                    }
                }
                SectionType::LINES(_) => {}
            }
        }
        Some(self)
    }

    pub fn new<'a>() -> Option<&mut Config<'a>> {
        let f = File::open("res/config.json").expect("");
        let v: Value = serde_json::from_reader(f).expect("");

        Config {
            sections: vec![],
            property_map: HashMap::new(),
        }.load_json(&v)
    }

    fn find_property(&self, section: &String, property: &String) -> Option<&&Property> {
        let mut key = section.clone();
        key.push_str(property);
        self.property_map.get(key.as_str())
    }

    fn parse_property(&self, line: &String, current_section: &String) -> Option<String> {
        None
    }

    fn parse_section(&self, line: &String) -> Option<String> {
        let right = line.find(']')?;
        Some(String::from(line.get(1..right)?))
    }

    fn parse_line(&self, line: &String, current_section: &String) -> Option<String> {
        match line.get(0..1)? {
            // for comments
            "%" | "#" | " " => {
                None
            }
            // for section
            "[" => {
                self.parse_section(line)
            }
            // Property or command line
            _ => {
                self.parse_property(line, current_section)
            }
        }
    }

    pub fn parse(&self, config_path: &Path) -> Result<(), Box<dyn Error>> {
        println!("Parsing config file: {:#?}", config_path);
        let f = File::open(config_path)?;
        let reader = BufReader::new(f);
        let mut current_section = String::from("");

        for line in reader.lines() {
            // let _ = self.parse_line(&line?.expect("Unexpected empty line"));
            match self.parse_line(&line?, &current_section) {
                None => {}
                Some(_current_section) => {
                    println!("The new section is {}", _current_section);
                    current_section = _current_section;
                }
            }
        }
        Ok(())
    }
}