use regex::Regex;

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

pub fn get (
    code: String,
    mut index: usize
) -> i64 {
    while code.chars().nth(index) == Some(' ') { index += 1; }
    let mut last_str = &code.chars().nth(index).expect("Failed to unwarap Last Character").to_string();
    let mut identifier_str: String = String::new();

    // std::String
    let mut reg = Regex::new(r"[a-zA-Z]+").expect("Failed to create REGEX");
    let res = match reg.captures(last_str) {
        Some(_) => "Ok".to_string(),
        None => "_".to_string(),
    };

    if res == "Ok".to_string() {
        for text in code.chars() {
            let reg = Regex::new(r"(\d|[a-zA-Z])+").expect("Failed to create REGEX");
            let res = match reg.captures(&text.to_string()) {
                Some(_) => "Ok".to_string(),
                None => "_".to_string()
            };

            if res == "_".to_string() { break; }
            identifier_str += &text.to_string();
            index += 1;
        }

        println!("name: {}", identifier_str);
        if identifier_str == "fun".to_string() { return TOKEN.fun; }
        if identifier_str == "ext".to_string() { return TOKEN.ext; }
    }

    0
}
