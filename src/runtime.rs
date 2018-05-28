use std::collections::HashMap;
use classfile_parser::parse_class;
use failure::{Error, err_msg};
use class::Class;
use method::Method;

pub struct Runtime<'a> {
    classes: HashMap<String, Class<'a>>,
    current_class: &'a Class<'a>,
    current_method: &'a Method<'a>,
}

impl <'a> Runtime<'a> {
    pub fn new(path: &str) -> Result<Self, Error> {
        let class_file = match parse_class(path) {
            Ok(c) => c,
            Err(e) => return Err(err_msg(e)),
        };

        let class = Class::new(class_file);
        let class_name = class.name();

        let main = match class.main() {
            Some(main) => main,
            None => return Err(err_msg("couldn't find main method")),
        };

        let mut classes: HashMap<String, Class> = HashMap::new();
        classes.insert(class_name, class);

        Ok(Runtime {
            classes: classes,
            current_class: classes.get(&class_name)
                            .expect("should never happen"),
            current_method: main,
        })
    }

    pub fn run(&self) {
        let code_attribute = self.current_method.code_attribute();
        let code = &mut code_attribute.code.clone();

        // .remove(0) is O(n), .pop() is O(1).
        code.reverse();
        
        loop {
            if code.len() == 0 {
                break;
            }

            match pop_u8(code) {
                0x00 => println!("nop"),
                0xb2 => {
                    let idx = pop_u16(code);
                    let resolved = self.current_class.resolve_constant(idx);
                    println!("getstatic #{} // {}", idx, resolved);
                },
                0xbb => println!("new #{}", pop_u16(code)),
                0x59 => println!("dup"),
                0x12 => println!("ldc #{}", pop_u8(code)),
                0xb7 => println!("invokespecial #{}", pop_u16(code)),
                0xb6 => println!("invokevirtual #{}", pop_u16(code)),
                0xb1 => println!("return"),
                unknown => {
                    println!("unknown: 0x{:02x}", unknown);
                    break;
                }
            }
        }
    }

    fn find_main(&self) -> Option<&Method> {
        for (_, class) in &self.classes {
            if let Some(main) = class.main() {
                return Some(main);
            }
        }
        None
    }
}

fn pop_u16(code: &mut Vec<u8>) -> u16 {
    let b1 = code.pop().expect("pop called on empty vec") as u16;
    let b2 = code.pop().expect("pop called on empty vec") as u16;
    (b1 << 8) + b2
}

fn pop_u8(code: &mut Vec<u8>) -> u8 {
    code.pop().expect("pop called on empty vec")
}
