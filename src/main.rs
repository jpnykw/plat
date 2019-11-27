extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod ast;
mod lexer;
// mod llvmir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    code = format!("{}\n", code).to_string();

    let mut token_buffer: Vec<[i64; 2]> = Vec::with_capacity(255);
    let mut index: usize = 0;
    loop {
        let token = lexer::get(&code, index);
        println!("  -> token: {}, index: {}", token[0], token[1]);
        index = token[1] as usize;

        if index >= code.len() {
            break;
        } else {
            token_buffer.push(token);
        }
    }

    println!();
    println!("\x1b[31mAll token was displayed.\x1b[m");
    println!("token buffer -> {:?}", token_buffer);
    // println!("llvmir -> {:?}", llvmir::main());
}

#[test]
fn token_m2() {
    let res = lexer::get(&String::from("fun hoge"), 0);
    assert_eq!(-2, res[0]);
}

#[test]
fn token_m3() {
    let res = lexer::get(&String::from("ext hoge"), 0);
    assert_eq!(-3, res[0]);
}

#[test]
fn token_m4() {
    let res = lexer::get(&String::from("# hoge\n"), 0);
    assert_eq!(-4, res[0]);
}
