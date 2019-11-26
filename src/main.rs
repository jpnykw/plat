extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();

    // Tokenize
    let mut index: usize = 0;
    let token: i64 = tokenize::get(code, index);
    println!("token: {}", token);
}

