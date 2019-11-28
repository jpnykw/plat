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
    // println!("\nTokens Buffer -> {:?}", token_buffer);
    // println!("llvmir -> {:?}", llvmir::main());

    // let ast = ast::get();
    // println!("\nExprAST -> {:#?}", ast);
}

#[test]
fn token_if() {
    let res = lexer::get(&String::from("if true"), 0);
    assert_eq!(-1, res[0]);
}

#[test]
fn token_for() {
    let res = lexer::get(&String::from("for i<1"), 0);
    assert_eq!(-2, res[0]);
}

#[test]
fn token_fun() {
    let res = lexer::get(&String::from("fun hoge"), 0);
    assert_eq!(-3, res[0]);
}

#[test]
fn token_print() {
    let res = lexer::get(&String::from("print()"), 0);
    assert_eq!(-4, res[0]);
}

#[test]
fn i_token_string() {
    let res = lexer::get(&String::from("\"hoge\""), 0);
    assert_eq!(-4, res[0]);
}

#[test]
fn i_token_number() {
    let res = lexer::get(&String::from("10\n"), 0);
    assert_eq!(-5, res[0]);
}

#[test]
fn i_token_comment() {
    let res = lexer::get(&String::from("# hoge\n"), 0);
    assert_eq!(-6, res[0]);
}

#[test]
fn i_token_identifier() {
    let res = lexer::get(&String::from("+\n"), 0);
    assert_eq!(43, res[0]);
}

