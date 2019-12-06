use regex::Regex;

pub struct Token {
    pub _if: i64,
    pub _then: i64,
    pub _else: i64,

    pub _for: i64,
    pub _fun: i64,
    pub _print: i64,

    pub _string: i64,
    pub _number: i64,
    pub _comment: i64,
    pub _identifier: i64
}

pub static TOKEN: Token = Token {
    // method
    _if: -1,
    _then: -2,
    _else: -3,

    _for: -4,
    _fun: -5,
    _print: -6,

    // primary
    _string: -7,
    _number: -8,
    _comment: -9,
    _identifier: -10
};

pub fn get (
    code: &String,
    mut index: usize
) -> [i64; 2] {
    while code.chars().nth(index) == Some(' ') || code.chars().nth(index) == Some('\n') { index += 1; }
    if index >= code.len() { return [0, index as i64]; }
    let last_str = &code.chars().nth(index).expect("Failed to unwarap Last Character").to_string();
    let mut identifier_str: String = String::new();

    // Method (hit String)
    {
        let reg = Regex::new(r"[a-zA-Z]+").expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok",
            None => "_"
        };
        // Found
        if res == "Ok" {
            loop {
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at method)").to_string();
                let reg = Regex::new(r"(\d|[a-zA-Z])+").expect("Failed to create REGEX");
                let res = match reg.captures(text) {
                    Some(_) => "Ok",
                    None => "_"
                };

                if res == "_" { break; }
                identifier_str += text;
                index += 1;
            }

            println!("  \x1b[35mType::Method::{}\x1b[m", identifier_str);
            if identifier_str == "if".to_string() { return [TOKEN._if, index as i64]; }
            if identifier_str == "then".to_string() { return [TOKEN._then, index as i64]; }
            if identifier_str == "else".to_string() { return [TOKEN._else, index as i64]; }
            if identifier_str == "for".to_string() { return [TOKEN._for, index as i64]; }
            if identifier_str == "fun".to_string() { return [TOKEN._fun, index as i64]; }
            if identifier_str == "print".to_string() { return [TOKEN._print, index as i64]; }
            return [TOKEN._identifier, index as i64];
        }
    }

    // String (hit Quote)
    {
        let reg = Regex::new(r#"""#).expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok",
            None => "_",
        };
        // Found
        if res == "Ok" {
            identifier_str = '"'.to_string();
            loop {
                index += 1;
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at Number)");
                identifier_str += &text.to_string();
                if text == &'"' { break; }
            }

            println!("  \x1b[35mType::String::{}\x1b[m", identifier_str);
            return [TOKEN._string, (index + 1) as i64];
        }
    }

    // Number (hit Number)
    {
        let reg = Regex::new(r"(\.)?\d+").expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok",
            None => "_",
        };
        // Found
        if res == "Ok" {
            loop {
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at Number)").to_string();
                let reg = Regex::new(r"(\.)?\d+").expect("Failed to create REGEX");
                let res = match reg.captures(&text.to_string()) {
                    Some(_) => "Ok",
                    None => "_"
                };

                if res == "_" { break; }
                identifier_str += &text.to_string();
                index += 1;
            }

            println!("  \x1b[35mType::Number::{}\x1b[m", identifier_str);
            return [TOKEN._number, index as i64];
        }
    }

    // Comments (hit String)
    {
        let reg = Regex::new(r"#").expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok",
            None => "_",
        };
        // Found
        if res == "Ok".to_string() {
            loop {
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at Comments)");
                if text == &'\n' { break; }
                identifier_str += &text.to_string();
                index += 1;
            }

            println!("  \x1b[35mType::Comment::{}\x1b[m", identifier_str);
            return [TOKEN._comment, index as i64];
        }
    }

    // Operators and Others
    println!("  \x1b[35mType::Unknown::{}\x1b[m", code.chars().nth(index).expect("Failed to unwrap chars (at Unknown)"));
    [code.chars().nth(index).expect("Failed to unwrap at convert to ASCII-Code").to_string().as_bytes()[0] as i64, (index + 1) as i64]
}

