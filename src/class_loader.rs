use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;
use std::fs::File;
use nom::IResult;
use failure::{Error, err_msg};
use std::io::Read;
use zip::ZipArchive;

use classfile_parser::class_parser;
use classfile_parser::ClassFile;

pub struct ClassLoader {
    paths: Vec<String>,
}

impl ClassLoader {
    pub fn bootstrap() -> Self {
        Self::new("/usr/lib/jvm/java-8-openjdk/jre/lib/resources.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/rt.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/sunrsasign.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/jsse.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/jce.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/charsets.jar:/usr/lib/jvm/java-8-openjdk/jre/lib/jfr.jar:/usr/lib/jvm/java-8-openjdk/jre/classes")
    }

    pub fn new(classpath: &str) -> Self {
        ClassLoader { 
            paths: classpath.split(":").map(|s| s.to_string()).collect(),
        }
    }

    pub fn load(&mut self, class_name: &str) -> Result<Option<ClassFile>, Error> {
        for path in &self.paths {
            if let Some(c) = try_load_from(path, class_name)? {
                return Ok(Some(c));
            }
        }

        Ok(None)
    }
}

fn try_load_from(classpath_entry: &str, class_name: &str) -> Result<Option<ClassFile>, Error> {
    let mut class_bytes = Vec::new();
    let path_to_class = format!("{}.class", class_name);

    if classpath_entry.ends_with(".jar") {
        let mut file = match File::open(&Path::new(classpath_entry)) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };

        match ZipArchive::new(file)?.by_name(&path_to_class) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        }.read_to_end(&mut class_bytes)?;
    } else {
        let path = Path::new(classpath_entry).join(path_to_class);
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };
        file.read_to_end(&mut class_bytes)?;
    }

    match class_parser(&class_bytes) {
        IResult::Done(_, c) => Ok(Some(c)),
        _ => Err(err_msg("error while parsing .class file")),
    }
}
