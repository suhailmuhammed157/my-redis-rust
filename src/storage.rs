use std::{collections::HashMap, io::Error};

#[derive(Debug)]
pub struct Storage {
    entries: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    pub fn add_new_entry(&mut self, key: &String, value: &String) -> Result<&str, Error> {
        let key_clone = key.clone();
        let value_clone = value.clone();
        let response = &self.entries.insert(key_clone, value_clone);
        if let None = response {
            Ok("OK")
        } else {
            Ok("r OK")
        }
    }
    pub fn read(&mut self, key: &String) -> Result<&String, &str> {
        let response = &self.entries.get(key);
        if let Some(val) = response {
            Ok(val)
        } else {
            Err("no such key found")
        }
    }
}
