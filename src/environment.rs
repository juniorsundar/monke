use std::collections::HashMap;

use crate::object::Object;

pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        let _ = self.store.insert(name, val.clone());
        val
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
