use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::PathBuf;

struct MessageMap {
    lang_map: HashMap<String, String>,
}

impl MessageMap {
    pub fn new() -> Self {
        MessageMap {
            lang_map: Default::default()
        }
    }

    pub fn set(&mut self, name: &String, value: &String) {
        self.lang_map.insert(name.clone(), value.clone());
    }

    pub fn get(&self, key: & String) -> Option<&String> {
        self.lang_map.get(key)
    }

    pub fn load_lang_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut name: String = String::new();
        let mut message: String = String::new();

        let _ = reader.lines().map(
            |line| -> Option<i32> {
                let line = line.unwrap();
                match line.get(0..1)? {
                    "%" => { /* Msg name*/ name = String::from(&line[1..]); }
                    "." => {
                        self.set(&name, &message);
                        message = String::new();
                    }
                    _ => {
                        message.push_str(line.as_str());
                        message.push('\n')
                    }
                }
                None
            }
        );
        Ok(())
    }

    pub fn load_json_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let reader = BufReader::new(File::open(file_path)?);
        Ok(())
    }
}
