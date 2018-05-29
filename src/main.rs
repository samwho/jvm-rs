extern crate classfile_parser;
extern crate failure;
extern crate nom;
extern crate zip;

use std::env;
use failure::Error;

mod runtime;
use runtime::Runtime;

mod class;
mod class_loader;

use classfile_parser::constant_info::ConstantInfo;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let classpath = &args[1];
    let main_class = &args[2];

    let mut runtime = Runtime::new(classpath).unwrap();
    runtime.run(main_class)
}

pub fn resolve_constant(const_pool: &Vec<ConstantInfo>, idx: u16) -> String {
    return match const_pool.get((idx - 1) as usize).unwrap() {
        ConstantInfo::Utf8(c) => c.utf8_string.clone(),
        ConstantInfo::Integer(c) => c.value.to_string(),
        ConstantInfo::Float(c) => c.value.to_string(),
        ConstantInfo::Long(c) => c.value.to_string(),
        ConstantInfo::Double(c) => c.value.to_string(),
        ConstantInfo::Class(c) => resolve_constant(const_pool, c.name_index),
        ConstantInfo::String(c) => resolve_constant(const_pool, c.string_index),
        ConstantInfo::FieldRef(c) => {
            let mut s = resolve_constant(const_pool, c.class_index);
            s.push_str(".");
            s.push_str(&resolve_constant(const_pool, c.name_and_type_index));
            s
        },
        ConstantInfo::MethodRef(c) => {
            let mut s = resolve_constant(const_pool, c.class_index);
            s.push_str(".");
            s.push_str(&resolve_constant(const_pool, c.name_and_type_index));
            s
        },
        ConstantInfo::InterfaceMethodRef(c) => {
            let mut s = resolve_constant(const_pool, c.class_index);
            s.push_str("#");
            s.push_str(&resolve_constant(const_pool, c.name_and_type_index));
            s
        },
        ConstantInfo::NameAndType(c) => {
            let mut s = resolve_constant(const_pool, c.name_index);
            s.push_str(":");
            s.push_str(&resolve_constant(const_pool, c.descriptor_index));
            s
        },
        ConstantInfo::MethodHandle(c) => resolve_constant(const_pool, c.reference_index),
        ConstantInfo::MethodType(c) => resolve_constant(const_pool, c.descriptor_index),
        ConstantInfo::InvokeDynamic(c) => {
            let mut s = resolve_constant(const_pool, c.bootstrap_method_attr_index);
            s.push_str("#");
            s.push_str(&resolve_constant(const_pool, c.name_and_type_index));
            s
        },
        ConstantInfo::Unusable => panic!("hit unusable constant"),
    }
}

