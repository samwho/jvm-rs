use classfile_parser::ClassFile;
use classfile_parser::attribute_info::code_attribute_parser;
use classfile_parser::attribute_info::CodeAttribute;
use classfile_parser::method_info::{MethodAccessFlags, MethodInfo};

use resolve_constant;

pub struct Class {
    class_file: ClassFile,
}

impl Class {
    pub fn new(class_file: ClassFile) -> Self {
        Class { 
            class_file: class_file,
        }
    }

    pub fn method(&self, name: &str) -> Option<&MethodInfo> {
        for method in &self.class_file.methods {
            if self.resolve_constant(method.name_index) == name {
                return Some(method);
            }
        }

        None
    }

    pub fn method_with_flags(&self, name: &str, flags: MethodAccessFlags) -> Option<&MethodInfo> {
        if let Some(m) = self.method(name) {
            if (m.access_flags & flags) == flags {
                return Some(m);
            }
        }

        None
    }

    pub fn resolve_constant(&self, idx: u16) -> String {
        resolve_constant(&self.class_file.const_pool, idx)
    }

    pub fn main(&self) -> Option<&MethodInfo> {
        self.method_with_flags(
            "main", 
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC)
    }

    pub fn code_attribute(&self, method_info: &MethodInfo) -> CodeAttribute {
        for attribute in &method_info.attributes {
            let name = self.resolve_constant(attribute.attribute_name_index);
            if name == "Code" {
                let code = code_attribute_parser(&attribute.info).unwrap();
                return code.1;
            }
        }

        panic!("no code found for method");
    }
}
