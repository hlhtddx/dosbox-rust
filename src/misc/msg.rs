use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use super::context::Err;

#[derive(Debug)]
pub struct Messages {
    lang_map: HashMap<String, String>,
}

impl Messages {
    pub fn new() -> Option<Self> {
        let mut message = Messages {
            lang_map: Default::default(),
        };

        if let Err(_) = message.load(&PathBuf::from("res/default.lang")) {
            ()
        }

        Some(message)
    }

    pub fn load(&mut self, file_path: &Path) -> Result<(), Err> {
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

    pub fn save(&self, file_path: &Path) -> Result<(), Err> {
        log::trace!("Save config to file: {:#?}", file_path);
        let mut f = File::open(file_path)?;
        let mut writer = BufWriter::new(f);
        for (name, value) in self.lang_map.iter() {
            let mut s = String::new();
            writeln!(&mut writer, ":{}\n{}\n.", name, value).unwrap();
            // writer.write(s.as_bytes()).unwrap();
        }
        Ok(())
    }

    pub fn set(&mut self, name: &String, value: &String) {
        self.lang_map.insert(name.clone(), value.clone());
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.lang_map.get(key)
    }
}
