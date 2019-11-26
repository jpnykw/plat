use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Token {
    eol: i64,
    fun: i64,
    ext: i64,
    identifier: i64,
    number: i64
}

const TOKEN: Token = Token{
    eol: -1,
    fun: -2,
    ext: -3,
    identifier: -4,
    number: -5
};

fn get_token() -> i64 {
    1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();

    // Tokenize
    let mut index = 0;

    while code.chars().nth(index) == Some(' ') {
        index += 1;
    }

    println!("id -> {}", index);
}

