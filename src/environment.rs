use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        }
        if let Some(outer_env) = &self.outer {
            return outer_env.borrow().get(name);
        }
        None
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
