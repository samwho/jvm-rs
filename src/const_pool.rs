use classfile_parser::constant_info::ConstantInfo;

use failure::{Error, err_msg};

pub struct ConstPool<'a> {
    const_pool: &'a Vec<ConstantInfo>,
}

impl <'a> ConstPool<'a> {
    pub fn new(const_pool: &'a Vec<ConstantInfo>) -> Self {
        Self { const_pool }
    }

    pub fn class_name(&self, idx: u16) -> Result<String, Error> {
        match self.get(idx) {
            ConstantInfo::Class(c) => Ok(self.resolve(c.name_index)),
            ConstantInfo::FieldRef(c) => self.class_name(c.class_index),
            ConstantInfo::MethodRef(c) => self.class_name(c.class_index),
            ConstantInfo::InterfaceMethodRef(c) => self.class_name(c.class_index),
            _ => Err(err_msg("index does not point to a class name")),
        }
    }

    pub fn resolve(&self, idx: u16) -> String {
        return match self.get(idx) {
            ConstantInfo::Utf8(c) => c.utf8_string.clone(),
            ConstantInfo::Integer(c) => c.value.to_string(),
            ConstantInfo::Float(c) => c.value.to_string(),
            ConstantInfo::Long(c) => c.value.to_string(),
            ConstantInfo::Double(c) => c.value.to_string(),
            ConstantInfo::Class(c) => self.resolve(c.name_index),
            ConstantInfo::String(c) => self.resolve(c.string_index),
            ConstantInfo::FieldRef(c) => {
                let mut s = self.resolve(c.class_index);
                s.push_str(".");
                s.push_str(&self.resolve(c.name_and_type_index));
                s
            },
            ConstantInfo::MethodRef(c) => {
                let mut s = self.resolve(c.class_index);
                s.push_str(".");
                s.push_str(&self.resolve(c.name_and_type_index));
                s
            },
            ConstantInfo::InterfaceMethodRef(c) => {
                let mut s = self.resolve(c.class_index);
                s.push_str("#");
                s.push_str(&self.resolve(c.name_and_type_index));
                s
            },
            ConstantInfo::NameAndType(c) => {
                let mut s = self.resolve(c.name_index);
                s.push_str(":");
                s.push_str(&self.resolve(c.descriptor_index));
                s
            },
            ConstantInfo::MethodHandle(c) => self.resolve(c.reference_index),
            ConstantInfo::MethodType(c) => self.resolve(c.descriptor_index),
            ConstantInfo::InvokeDynamic(c) => {
                let mut s = self.resolve(c.bootstrap_method_attr_index);
                s.push_str("#");
                s.push_str(&self.resolve(c.name_and_type_index));
                s
            },
            ConstantInfo::Unusable => panic!("hit unusable constant"),
        }
    }

    fn get(&self, idx: u16) -> &ConstantInfo {
        self.const_pool.get((idx - 1) as usize).unwrap()
    }
}
