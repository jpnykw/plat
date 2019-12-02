extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod ast;
mod lexer;

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

    println!("\n\x1b[31mAll token was displayed.\x1b[m\n");

    let _root: ast::ExprAST  = ast::new(0);
    for token in token_buffer {
        if token[0] != -6 {
            // Ignore comment token (-6)
            let text = match token[0] {
                -1 => "Called method `if()`",
                -2 => "Called method `for()`",
                -3 => "Created `function`",
                -4 => "Called method `print()`",
                -5 => "Created primitive `string`",
                -6 => "Created primitive `number`",
                _ => ""
            };

            if text != "" {
                println!("{0:<03}: {1}", token[1], text);
            }
        }
    }
}

#[test]
fn token_if() {
    let res = lexer::get(&String::from("if true"), 0);
    assert_eq!(lexer::TOKEN._if, res[0]);
}

#[test]
fn token_then() {
    let res = lexer::get(&String::from("then\n"), 0);
    assert_eq!(lexer::TOKEN._then, res[0]);
}

#[test]
fn token_else() {
    let res = lexer::get(&String::from("else\n"), 0);
    assert_eq!(lexer::TOKEN._else, res[0]);
}

#[test]
fn token_for() {
    let res = lexer::get(&String::from("for i<1"), 0);
    assert_eq!(lexer::TOKEN._for, res[0]);
}

#[test]
fn token_fun() {
    let res = lexer::get(&String::from("fun hoge"), 0);
    assert_eq!(lexer::TOKEN._fun, res[0]);
}

#[test]
fn token_print() {
    let res = lexer::get(&String::from("print()"), 0);
    assert_eq!(lexer::TOKEN._print, res[0]);
}

#[test]
fn i_token_string() {
    let res = lexer::get(&String::from("\"hoge\""), 0);
    assert_eq!(lexer::TOKEN._string, res[0]);
}

#[test]
fn i_token_number() {
    let res = lexer::get(&String::from("10\n"), 0);
    assert_eq!(lexer::TOKEN._number, res[0]);
}

#[test]
fn i_token_comment() {
    let res = lexer::get(&String::from("# hoge\n"), 0);
    assert_eq!(lexer::TOKEN._comment, res[0]);
}
#[test]
fn i_token_identifier() {
    let res = lexer::get(&String::from("+\n"), 0);
    assert_eq!(43, res[0]);
}

