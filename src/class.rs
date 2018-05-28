use std::collections::HashMap;
use classfile_parser::ClassFile;
use classfile_parser::constant_info::ConstantInfo;
use classfile_parser::method_info::MethodAccessFlags;
use method::Method;

pub struct Class<'a> {
    class_file: ClassFile,
    methods: HashMap<String, Method<'a>>,
}

impl <'a> Class<'a> {
    pub fn new(class_file: ClassFile) -> Self {
        Class { 
            class_file: class_file,
            methods: HashMap::with_capacity(class_file.methods_count as usize),
        }
    }

    pub fn name(&self) -> String {
        for constant in &self.class_file.const_pool {
            if let ConstantInfo::Class(c) = constant {
                return self.resolve_constant(c.name_index);
            }
        }

        panic!("no class name found");
    }

    pub fn resolve_constant(&self, idx: u16) -> String {
        return match &self.class_file.const_pool[(idx - 1) as usize] {
            ConstantInfo::Utf8(c) => c.utf8_string.clone(),
            ConstantInfo::Integer(c) => c.value.to_string(),
            ConstantInfo::Float(c) => c.value.to_string(),
            ConstantInfo::Long(c) => c.value.to_string(),
            ConstantInfo::Double(c) => c.value.to_string(),
            ConstantInfo::Class(c) => self.resolve_constant(c.name_index),
            ConstantInfo::String(c) => self.resolve_constant(c.string_index),
            ConstantInfo::FieldRef(c) => {
                let mut s = self.resolve_constant(c.class_index);
                s.push_str(".");
                s.push_str(&self.resolve_constant(c.name_and_type_index));
                s
            },
            ConstantInfo::MethodRef(c) => {
                let mut s = self.resolve_constant(c.class_index);
                s.push_str(".");
                s.push_str(&self.resolve_constant(c.name_and_type_index));
                s
            },
            ConstantInfo::InterfaceMethodRef(c) => {
                let mut s = self.resolve_constant(c.class_index);
                s.push_str("#");
                s.push_str(&self.resolve_constant(c.name_and_type_index));
                s
            },
            ConstantInfo::NameAndType(c) => {
                let mut s = self.resolve_constant(c.name_index);
                s.push_str(":");
                s.push_str(&self.resolve_constant(c.descriptor_index));
                s
            },
            ConstantInfo::MethodHandle(c) => self.resolve_constant(c.reference_index),
            ConstantInfo::MethodType(c) => self.resolve_constant(c.descriptor_index),
            ConstantInfo::InvokeDynamic(c) => {
                let mut s = self.resolve_constant(c.bootstrap_method_attr_index);
                s.push_str("#");
                s.push_str(&self.resolve_constant(c.name_and_type_index));
                s
            },
            ConstantInfo::Unusable => panic!("hit unusable constant"),
        }
    }

    pub fn method(&self, name: &str) -> Option<&Method> {
        if let Some(m) = self.methods.get(name) {
            return Some(m);
        }

        for method in &self.class_file.methods {
            if name == self.resolve_constant(method.name_index) {
                let m = Method::new(self, method);
                self.methods.insert(name.to_owned(), m);
                return self.methods.get(name);
            }
        }
    
        None
    }

    pub fn method_with_flags(&self, name: &str, flags: MethodAccessFlags) -> Option<&Method> {
        if let Some(m) = self.method(name) {
            if (m.method_info.access_flags & flags) == flags {
                return Some(m);
            }
        }

        None
    }

    pub fn main(&self) -> Option<&Method> {
        self.method_with_flags("main", MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC)
    }
}
