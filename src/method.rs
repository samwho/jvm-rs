use class::Class;
use classfile_parser::method_info::MethodInfo;
use classfile_parser::attribute_info::CodeAttribute;
use classfile_parser::attribute_info::code_attribute_parser;

pub struct Method<'a> {
    pub class: &'a Class<'a>,
    pub method_info: &'a MethodInfo,
}

impl <'a> Method<'a> {
    pub fn new(class: &'a Class, method_info: &'a MethodInfo) -> Self {
        Method { class, method_info }
    }

    pub fn code_attribute(&self) -> CodeAttribute {
        for attribute in &self.method_info.attributes {
            let idx = attribute.attribute_name_index;
            let name = self.class.resolve_constant(idx);
            if name == "Code" {
                let code = code_attribute_parser(&attribute.info).unwrap();
                return code.1;
            }
        }

        panic!("no code found for method");
    }
}
