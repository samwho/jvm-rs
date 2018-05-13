use std::collections::HashMap;
use classfile_parser::parse_class;
use failure::{Error, err_msg};
use class::Class;
use method::Method;

pub struct Runtime {
    classes: HashMap<String, Class>,
}

impl Runtime {
    pub fn new(path: &str) -> Result<Self, Error> {
        let class_file = match parse_class(path) {
            Ok(c) => c,
            Err(e) => return Err(err_msg(e)),
        };

        let class = Class::new(class_file);
        let mut classes: HashMap<String, Class> = HashMap::new();
        classes.insert(class.name(), class);

        Ok(Runtime {
            classes: classes,
        })
    }

    pub fn run(&self) {
        match self.find_main() {
            Some(main) => main.run(),
            None => panic!("could not find main"),
        }
    }

    fn find_main(&self) -> Option<Method> {
        for (_, class) in &self.classes {
            if let Some(main) = class.main() {
                return Some(main);
            }
        }

        None
    }
}
