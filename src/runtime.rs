use class_loader::ClassLoader;
use class::Class;
use failure::{Error, err_msg};

pub struct Runtime {
    class_loaders: Vec<ClassLoader>,
}

impl Runtime {
    pub fn with_classpath(classpath: &str) -> Self {
        Self::new(vec![ClassLoader::new(classpath)])        
    }

    pub fn new(mut class_loaders: Vec<ClassLoader>) -> Self {
        class_loaders.push(ClassLoader::bootstrap());
        class_loaders.push(ClassLoader::new("."));
        Runtime { class_loaders }
    }

    pub fn run(&mut self, class_name: &str) -> Result<(), Error> {
        let current_class = match load_class(&mut self.class_loaders, class_name)? {
            Some(class) => class,
            None => return Err(err_msg("could not find main class")),
        };

        let current_method = current_class.main().unwrap();
        let code_attribute = current_class.code_attribute(current_method);
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
                    let class_name = current_class.const_pool().class_name(idx)?;
                    //let class = load_class(&mut self.class_loaders, &class_name)?;
                    println!("getstatic #{} // {}", idx, current_class.const_pool().resolve(idx));
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

        Ok(())
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

fn load_class<'a>(class_loaders: &'a mut Vec<ClassLoader>, class_name: &str) -> Result<Option<&'a Class>, Error> {
   for class_loader in class_loaders {
        if let Some(class) = class_loader.load(class_name)? {
            return Ok(Some(class));
        }
   }

   Ok(None)
}

