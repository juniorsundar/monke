use std::collections::HashMap;

use crate::object::Object;

pub type EnvId = usize;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<EnvId>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: EnvId) -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        }
        None
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        let _ = self.store.insert(name, val.clone());
        val
    }

    pub fn get_outer(&self) -> Option<EnvId> {
        self.outer
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
