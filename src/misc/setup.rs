use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;


use serde_json::{Map, Value};

use PropertyValue::{Pbool, Pdouble, Phex, Pint, Pnull, Ppaths, Pstr};

#[derive(Debug)]
enum Changeable {
    OnlyAtStart,
    Always,
    WhenIdle,
}

#[derive(Debug, Clone)]
enum PropertyValue {
    Pnull,
    Ppaths(String),
    Pstr(String),
    Phex(String),
    Pbool(bool),
    Pint(i64),
    Pdouble(f64),
}

#[derive(Debug)]
pub struct Property {
    name: String,
    changeable: Changeable,
    help_str: String,
    default_value: PropertyValue,
    available_values: Vec<String>,
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
pub struct Config {
    sections: Vec<Rc<Section>>,
    property_map: HashMap<String, PropertyValue>,
}

impl Property {
    fn new(obj: (&String, &Value)) -> Option<Self> {
        let name = obj.0.to_string();
        let value = obj.1;
        let prop_type = value["type"].as_str()?;
        let default_value: PropertyValue;
        let mut available_values: Vec<String> = vec![];
        let help_str = value["help"].as_str()?.to_string();
        let mut min: i64;
        let mut max: i64;

        let changeable: Changeable = match value["changeable"].as_str()? {
            "OnlyAtStart" => { Changeable::OnlyAtStart }
            "Always" => { Changeable::Always }
            "WhenIdle" => { Changeable::WhenIdle }
            &_ => { Changeable::Always }
        };
        log::trace!("changeable = {:?}", changeable);

        min = 0;
        max = 0;
        log::trace!("min max = {:?}, {:?}", min, max);

        match &value["values"] {
            Value::Array(_values) => {
                for s in _values.iter() {
                    available_values.push(s.as_str()?.to_string())
                }
            }
            Value::Null => {}
            _values => { panic!("Invalid type for 'values' field {}, Only string or string list is allowed", _values) }
        }
        log::trace!("available_values = {:?}", available_values);

        match prop_type {
            "path" => {
                default_value = Ppaths(value["default"].as_str()?.to_string());
            }
            "string" | "multi" => {
                default_value = Pstr(value["default"].as_str()?.to_string());
            }
            "int" => {
                default_value = Pint(value["default"].as_i64()?);
                min = value["min"].as_i64()?;
                max = value["max"].as_i64()?;
            }
            "hex" => {
                default_value = Phex(value["default"].as_str()?.to_string());
            }
            "double" => {
                default_value = Pdouble(value["default"].as_f64()?);
            }
            "bool" => {
                default_value = Pbool(value["default"].as_bool()?);
            }
            &_ => { panic!("Unknown property type {}", prop_type) }
        };
        log::trace!("default_value = {:?}", default_value);

        Some(Property {
            name,
            changeable,
            help_str,
            default_value,
            available_values,
            min,
            max,
        })
    }
}

impl Section {
    fn new(obj: (&String, &Value)) -> Option<Self> {
        let name = obj.0;
        let t = obj.1["type"].as_str()?;
        let section_type: SectionType;
        match t {
            "property" => {
                let mut props: Vec<Property> = vec![];
                let properties = obj.1["properties"].as_object()?;
                for obj in properties.iter() {
                    log::trace!("New property {}", obj.0);
                    props.push(Property::new(obj)?);
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

impl Config {
    pub fn new<'a>() -> Option<Self> {
        let f = File::open("res/config.json").expect("");
        let v: Value = serde_json::from_reader(f).expect("");

        let mut config = Config {
            sections: vec![],
            property_map: HashMap::new(),
        };

        let items = v["sections"].as_object()?;

        {
            let sections = &mut config.sections;
            for obj in items.iter() {
                let section = Rc::new(Section::new(obj)?);
                log::trace!("new section {:?}", section.name);
                sections.push(section);
            }
        }

        {
            let property_map = &mut config.property_map;
            for section in config.sections.iter() {
                match &section.section_type {
                    SectionType::PROPERTIES(properties) => {
                        for property in properties.iter() {
                            let key = String::from_iter([section.name.as_str(), ".", property.name.as_str()]);
                            property_map.insert(key, property.default_value.clone());
                        }
                    }
                    SectionType::LINES(_) => {}
                }
            }
        }

        Some(config)
    }

    fn find_property(&mut self, section: &str, property: &str) -> Option<&mut PropertyValue> {
        let key = String::from_iter([section, ".", property]);
        self.property_map.get_mut(key.as_str())
    }

    fn parse_property(&mut self, line: &String, current_section: &String) -> Option<String> {
        let pos: Vec<&str> = line.split('=').collect();
        if pos.len() == 2 {
            let name = pos[0];
            let value = pos[1];
            log::trace!("parse property {}.{} = {}", current_section, name, value);
            if let Some(prop_value) = self.find_property(current_section, name) {
                *prop_value = match prop_value {
                    Pnull => { Pnull }
                    Pstr(_) => { Pstr(String::from(value)) }
                    Ppaths(_) => { Ppaths(String::from(value)) }
                    Phex(_) => { Phex(String::from(value)) }
                    Pbool(_) => { Pbool(value == "true") }
                    Pint(_) => { Pint(i64::from_str_radix(value, 10).unwrap()) }
                    Pdouble(_) => { Pdouble(f64::from_str(value).unwrap()) }
                };
                log::trace!("set a property {}.{} = {:?}", current_section, name, *prop_value);
            } else {
                log::warn!("Property {}.{} is not defined", current_section, name);
            }
        }
        None
    }

    fn parse_section(&self, line: &String) -> Option<String> {
        let right = line.find(']')?;
        Some(String::from(line.get(1..right)?))
    }

    fn parse_line(&mut self, line: &String, current_section: &String) -> Option<String> {
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

    pub fn parse(&mut self, config_path: &Path) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing config file: {:#?}", config_path);
        let f = File::open(config_path)?;
        let reader = BufReader::new(f);
        let mut current_section = String::from("");

        for line in reader.lines() {
            match self.parse_line(&line?, &current_section) {
                None => {}
                Some(_current_section) => {
                    current_section = _current_section;
                }
            }
        }
        Ok(())
    }
}