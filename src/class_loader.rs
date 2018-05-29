use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;
use std::fs::File;
use nom::IResult;
use failure::{Error, err_msg};
use std::io::Read;
use zip::ZipArchive;

use class::Class;
use classfile_parser::class_parser;
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

    pub fn load(&mut self, class_name: &str) -> Result<Option<&Class>, Error> {
        match self.classes.entry(class_name.to_owned()) {
            Occupied(o) => Ok(Some(o.into_mut())),
            Vacant(v) => match try_load(&self.paths, class_name)? {
                Some(c) => Ok(Some(v.insert(c))),
                None => Ok(None),
            }
        }
    }
}

fn try_load(paths: &Vec<String>, class_name: &str) -> Result<Option<Class>, Error> {
    for path in paths {
        if let Some(c) = try_load_from(path, class_name)? {
            return Ok(Some(Class::new(c)));
        }
    }

    Ok(None)
}

fn try_load_from(classpath_entry: &str, class_name: &str) -> Result<Option<ClassFile>, Error> {
    let path_to_class = path_to(classpath_entry, class_name);
    let path = Path::new(&path_to_class);

    let mut class_bytes = Vec::new();

    if classpath_entry.ends_with(".jar") {
        ZipArchive::new(File::open(&path)?)?
            .by_name(&path_to_class)?
            .read_to_end(&mut class_bytes)?;
    } else {
        File::open(&path)?.read_to_end(&mut class_bytes)?;
    }

    match class_parser(&class_bytes) {
        IResult::Done(_, c) => Ok(Some(c)),
        _ => Err(err_msg("error while parsing .class file")),
    }
}

fn path_to(dir: &str, class_name: &str) -> String {
    let mut path = class_name.replace(".", "/");
    path.push_str(".class");
    Path::new(dir).join(path).to_str().unwrap().to_owned()
}
