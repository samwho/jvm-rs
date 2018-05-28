use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;

use class::Class;
use class_loader::ClassLoader;
use classfile_parser::parse_class;

use failure::{Error, err_msg};

pub struct DirClassLoader {
    classes: HashMap<String, Class>,
    dir: String,
}

impl DirClassLoader {
    pub fn new(dir: String) -> Self {
        DirClassLoader { dir: dir, classes: HashMap::new() }
    }
}

impl ClassLoader for DirClassLoader {
    fn load(&mut self, name: &str) -> Result<&Class, Error> {
        match self.classes.entry(name.to_owned()) {
            Occupied(o) => Ok(o.into_mut()),
            Vacant(v) => match parse_class(&path_to(&self.dir, name)) {
                Ok(c) => Ok(v.insert(Class::new(c))),
                Err(e) => Err(err_msg(e)),
            },
        }
    }
}

fn path_to(dir: &str, class_name: &str) -> String {
    Path::new(dir)
        .join(class_name.replace(".", "/"))
        .to_str()
        .unwrap()
        .to_owned()
}
