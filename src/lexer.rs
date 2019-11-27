use regex::Regex;

struct Token {
    eol: i64,
    fun: i64,
    ext: i64,
    comment: i64,
    identifier: i64,
    number: i64
}

const TOKEN: Token = Token{
    eol: -1,
    fun: -2,
    ext: -3,
    comment: -4,
    identifier: -5,
    number: -6
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
            Some(_) => "Ok".to_string(),
            None => "_".to_string(),
        };
        // Found
        if res == "Ok".to_string() {
            loop {
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at method)").to_string();
                let reg = Regex::new(r"(\d|[a-zA-Z])+").expect("Failed to create REGEX");
                let res = match reg.captures(text) {
                    Some(_) => "Ok".to_string(),
                    None => "_".to_string()
                };

                if res == "_".to_string() { break; }
                identifier_str += text;
                index += 1;
            }

            println!("\x1b[36mType::method:\x1b[m \x1b[32m{}\x1b[m", identifier_str);
            if identifier_str == "fun".to_string() { return [TOKEN.fun, index as i64]; }
            if identifier_str == "ext".to_string() { return [TOKEN.ext, index as i64]; }
            return [1, index as i64];
        }
    }

    // Number (hit Number)
    {
        let reg = Regex::new(r"(\.)?\d+").expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok".to_string(),
            None => "_".to_string(),
        };
        // Found
        if res == "Ok".to_string() {
            loop {
                let text = &code.chars().nth(index).expect("aailed to unwrap chars (at Number)").to_string();

                let reg = Regex::new(r"(\.)?\d+").expect("Failed to create REGEX");
                let res = match reg.captures(&text.to_string()) {
                    Some(_) => "Ok".to_string(),
                    None => "_".to_string()
                };

                if res == "_".to_string() { break; }
                identifier_str += &text.to_string();
                index += 1;
            }

            println!("\x1b[36mType::Primitive Number:\x1b[m \x1b[32m{}\x1b[m", identifier_str);
            return [TOKEN.number, index as i64];
        }
    }

    // Comments (hit String)
    {
        let reg = Regex::new(r"#").expect("Failed to create REGEX");
        let res = match reg.captures(last_str) {
            Some(_) => "Ok".to_string(),
            None => "_".to_string(),
        };
        // Found
        if res == "Ok".to_string() {
            loop {
                let text = &code.chars().nth(index).expect("Failed to unwrap chars (at Comments)");
                if text == &'\n' { break; }
                identifier_str += &text.to_string();
                index += 1;
            }

            println!("\x1b[36mType::Comments:\x1b[m \x1b[32m{}\x1b[m", identifier_str);
            return [TOKEN.comment, index as i64];
        }
    }

    // Operators and Others
    println!("\x1b[36mType::Unknown:\x1b[m \x1b[32m{}\x1b[m", code.chars().nth(index).expect("Failed to unwrap chars (at Unknown)"));
    [1, (index + 1) as i64]
}

