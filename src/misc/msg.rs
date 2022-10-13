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

    pub fn set(&mut self, name: &str, value: String) {
        self.lang_map.insert(String::from(name), String::from(value));
    }

    pub fn get(&self, key: &str) -> &str {
        if let Some(value) = self.lang_map.get(key) {
            return value.deref()
        }
        key
    }

    pub fn load_lang_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut name: &str = "";
        let mut message: String = String::new();

        for line in reader.lines()? {
            match line.get(0..1)? {
                "%" => { /* Msg name*/ name = line[1..]; }
                "." => { self.set(name, message.to_string()); message = String::new(); }
                _ => { message.push_str(line.as_str()); message.push('\n') }
            }
        }
        Ok(())
    }

    pub fn load_json_file(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let reader = BufReader::new(File::open(file_path)?);

        for line in reader.lines()? {
        }
        Ok(())
    }
}
