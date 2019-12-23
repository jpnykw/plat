extern crate regex;

use inkwell::context::Context;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod ast;
mod lexer;
mod llvmir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    println!("\n\x1b[32mInput ----->\x1b[m");
    println!("{}", code);
    code = format!("{}\n", code).to_string();

    // for generate LLVM IR
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    println!("\x1b[32mTokens ----->\x1b[m");

    let mut token_buffer: Vec<[i64; 2]> = Vec::with_capacity(255);
    let mut index: usize = 0;
    loop {
        let token = lexer::get(&code, index);
        index = token[1] as usize;

        if index >= code.len() {
            break;
        } else {
            println!("  token: {}", token[0]);
            println!("  index: {}", token[1]);
            println!();
            token_buffer.push(token);
        }
    }

    println!("\x1b[32mLatent Semantic Analysis ----->\x1b[m");

    // let mut scope: usize = 0; // depth
    let mut target: usize = 0; // index

    let mut root: ast::Root  = ast::new(0);
    for token in token_buffer {
        if token[0] != lexer::TOKEN._comment {
            // insert to most-high scope
            // root.insert(token[0]);

            match token[0] {
                -6 => {
                    root.insert(lexer::TOKEN._print, 0);
                    // println!("print --> \n {} --> \n {:#?}", root.node.len(), root.node[0]);
                    target = root.node.len() - 1;
                },

                -7 => {
                    // How should I insert node ...
                    match &root.node[target] {
                        ast::Types::Exp(val) => {
                            println!("{:#?}", val);
                            // println!("token -> {:#?}", val.token);
                            // val.insert(lexer::TOKEN._string);
                            // println!("val -> {:#?}", val);
                        },

                        _ => {}
                    }

                    // root.node[target].as_ref().unwrap().insert(lexer::TOKEN._string as usize, 0);
                    // let node = root.node[target].as_ref().unwrap();
                    // println!("{:#?}", node.token);
                    // root.node[target].as_ref().unwrap().insert(lexer::TOKEN._string);
                    // node.insert(lexer::TOKEN._string);
                },

                _ => { }
            };
        }
    }

    println!("\n\x1b[32mGenerate AST (test) ----->\x1b[m");
    println!("{:?}", root);

    println!("\n\x1b[33mGenerate LLVM IR using AST (test) ----->\x1b[m");
    println!("{:?}", llvmir::generate(root).expect("Failed to unwrap LLVMIR"));

    println!("\n\x1b[32mGenerate LLVM IR (test) ----->\x1b[m");

    // Test llvm ir
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(&basic_block);

    // Print any text as llvm ir
    let fun = module.get_function("putchar");
    let text = "Hello World!\n";
    for c in text.chars() {
        let ascii = c.to_string().as_bytes()[0] as u64;
        builder.build_call(fun.unwrap(), &[i32_type.const_int(ascii, false).into()], "putchar");
    }

    builder.build_return(Some(&i32_type.const_int(0, false)));
    module.print_to_stderr();

    // let meta = module.get_global_metadata("main");
    // println!("\n\x1b[31mGlobal meta deta.\x1b[m\n");
    // println!("{:#?}", meta);
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

