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
mod const_pool;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let classpath = &args[1];
    let main_class = &args[2];
    Runtime::with_classpath(classpath).run(main_class)
}


