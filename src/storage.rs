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
        let value_clone: String = value.clone();
        &self.entries.insert(key_clone, value_clone);
        Ok("OK")
    }
}