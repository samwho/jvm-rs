use class::Class;
use classfile_parser::method_info::MethodInfo;
use classfile_parser::attribute_info::CodeAttribute;
use classfile_parser::attribute_info::code_attribute_parser;

pub struct Method<'a> {
    class: &'a Class,
    method_info: &'a MethodInfo,
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

    pub fn run(&self) {
        let code_attribute = self.code_attribute();
        let code = &mut code_attribute.code.clone();
        
        loop {
            if code.len() == 0 {
                break;
            }

            match take_u8(code) {
                0x00 => println!("nop"),
                0xb2 => println!("getstatic #{}", take_u16(code)),
                0xbb => println!("new #{}", take_u16(code)),
                0x59 => println!("dup"),
                0x12 => println!("ldc #{}", take_u8(code)),
                0xb7 => println!("invokespecial #{}", take_u16(code)),
                0xb6 => println!("invokevirtual #{}", take_u16(code)),
                0xb1 => println!("return"),
                unknown => {
                    println!("unknown: 0x{:02x}", unknown);
                    break;
                }
            }
        }
    }
}

fn take_u16(code: &mut Vec<u8>) -> u16 {
    let b1 = code.remove(0) as u16;
    let b2 = code.remove(0) as u16;
    (b1 << 8) + b2
}

fn take_u8(code: &mut Vec<u8>) -> u8 {
    code.remove(0)
}
