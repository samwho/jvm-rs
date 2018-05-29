extern crate classfile_parser;
extern crate failure;
extern crate nom;
extern crate zip;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::env;
use failure::Error;

mod runtime;
use runtime::Runtime;

mod class;
mod class_loader;
mod const_pool;
mod stack;
mod frame;
mod variable;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    let classpath = &args[1];
    let main_class = &args[2];
    Runtime::with_classpath(classpath).run(main_class)
}


