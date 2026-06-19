use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        let mut object = self.store.get(name);
        if object.is_none()
            && let Some(outer_exists) = &self.outer
        {
            object = outer_exists.get(name);
        }
        object
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
