use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Cache {
    pub map: HashMap<u64, i16>,
}

impl Cache {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Cache {
            map: HashMap::new(),
        }))
    }

    pub fn get(&self, key: u64) -> Option<i16> {
        match self.map.get(&key) {
            Some(v) => Some(*v),
            None => None,
        }
    }

    pub fn set(&mut self, key: u64, val: i16) {
        self.map.insert(key, val);
    }

    pub fn clear(&mut self) {
        self.map = HashMap::new();
    }
}