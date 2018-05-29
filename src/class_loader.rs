use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;

use class::Class;
use classfile_parser::parse_class;
use classfile_parser::ClassFile;

pub struct ClassLoader {
    paths: Vec<String>,
    classes: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(classpath: &str) -> Self {
        ClassLoader { 
            paths: classpath.split(":").map(|s| s.to_string()).collect(),
            classes: HashMap::new(),
        }
    }

    pub fn load(&mut self, class_name: &str) -> Option<&Class> {
        match self.classes.entry(class_name.to_owned()) {
            Occupied(o) => Some(o.into_mut()),
            Vacant(v) => match try_load(&self.paths, class_name) {
                Some(c) => Some(v.insert(c)),
                None => None,
            }
        }
    }
}

fn try_load(paths: &Vec<String>, class_name: &str) -> Option<Class> {
    for path in paths {
        if let Some(c) = try_load_from(path, class_name) {
            return Some(Class::new(c));
        }
    }

    None
}

fn try_load_from(path: &str, class_name: &str) -> Option<ClassFile> {
    if path.ends_with(".jar") {
        return None;
    } 

    match parse_class(&path_to(path, class_name)) {
        Ok(class) => Some(class),
        Err(_) => None,
    }
}

fn path_to(dir: &str, class_name: &str) -> String {
    Path::new(dir)
        .join(class_name.replace(".", "/"))
        .to_str()
        .unwrap()
        .to_owned()
}
