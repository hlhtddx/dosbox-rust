use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Messages {
    lang_map: HashMap<String, String>,
}

impl Messages {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut message = Messages {
            lang_map: Default::default(),
        };
        message.load_lang_file(PathBuf::from("res/default.lang"))?;
        Ok(message)
    }

    pub fn set(&mut self, name: &String, value: &String) {
        self.lang_map.insert(name.clone(), value.clone());
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.lang_map.get(key)
    }

    pub fn load_lang_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut name: String = String::new();
        let mut message: String = String::new();

        let _ = reader.lines().for_each(|line| {
            if let Ok(line) = line {
                match line.get(0..1) {
                    Some(":") => {
                        /* Msg name*/
                        name = String::from(&line[1..]);
                    }
                    Some(".") => {
                        if !message.is_empty() {
                            self.set(&name, &message);
                            name = String::new();
                            message = String::new();
                        }
                    }
                    None => {}
                    _ => {
                        message.push_str(line.as_str());
                        message.push('\n')
                    }
                }
            }
        });
        log::trace!("message map = {:#?}", self.lang_map);
        Ok(())
    }
}
