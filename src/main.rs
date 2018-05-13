extern crate classfile_parser;
extern crate failure;

use std::env;

mod runtime;
use runtime::Runtime;

mod class;
mod method;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let runtime = Runtime::new(file).unwrap();
    runtime.run();
}

