use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::PathBuf;

struct MessageMap {
    lang_map: HashMap<&'static str, &'static str>,
}

impl MessageMap {
    pub fn new() -> Self {
        MessageMap {
            lang_map: Default::default()
        }
    }

    pub fn add(&mut self, name: &'static str, value: &'static str) {
        self.lang_map.insert(name, value);
    }

    pub fn get(&self, key: &'static str) -> &str {
        if let Some(value) = self.lang_map.get(key) {
            return value.deref()
        }
        key
    }

    pub fn load_language_file(&mut self, file_path: PathBuf) -> io::Result<()> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);

        for line in reader.lines() {
        }
        Ok(())
    }

    pub fn load_json_file(&mut self, file_path: PathBuf) -> io::Result<()> {
        log::trace!("Parsing language file: {:#?}", file_path);
        let reader = BufReader::new(File::open(file_path)?);

        for line in reader.lines() {
        }
        Ok(())
    }
}
