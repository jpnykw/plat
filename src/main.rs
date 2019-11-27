extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    code = format!("{}\n", code).to_string();

    // Tokenize
    let mut index: usize = 0;
    // for i in 0..21 {
    loop {
        let token = tokenize::get(&code, index);
        println!("  -> token: {}, index: {}", token[0], token[1]);
        index = token[1] as usize;

        if index >= code.len() {
            break;
        }
    }

    println!("\x1b[31mAll token was displayed.\x1b[m");
}

#[test]
fn token_m2() {
    let res = tokenize::get(&String::from("fun hoge"), 0);
    assert_eq!(-2, res[0]);
}

#[test]
fn token_m3() {
    let res = tokenize::get(&String::from("ext hoge"), 0);
    assert_eq!(-3, res[0]);
}
