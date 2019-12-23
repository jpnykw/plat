// use std::boxed::Box;

#[derive(Debug)]
pub enum Types {
    Exp(Expression),
    Stat(Statement)
}

#[derive(Debug)]
pub enum ValueTypes {
    Int(i64),
    Float(f64),
    Str(String),
}

#[derive(Debug)]
pub struct Root {
    pub token: i64,
    pub node: Vec<Types>
}

#[derive(Debug)]
pub struct Expression {
    pub token: i64,
    pub value: ValueTypes,
    pub node: Vec<Types>
}

#[derive(Debug)]
pub struct Statement {
    pub token: i64,
    pub node: Vec<Types>
}

impl Root {
    pub fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(&mut self, token: i64, mode: i64) {
        if mode == 0 {
            let node: Types = Types::Exp(Expression::new(token));
            // self.node.push(Some(vec![node]));
            self.node.push(node);
        } else {
            let node: Types = Types::Stat(Statement::new(token));
            // self.node.push(Some(vec![node]));
            self.node.push(node);
        }
    }
}

impl Expression {
    pub fn new(token: i64) -> Self {
        Self {
            token: token,
            value: ValueTypes::Str("Hello World!".to_string()),
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(&mut self, token: i64, value: ValueTypes) {
        let node: Types = Types::Stat(Statement::new(token));
        self.node.push(node);
        self.value = value;
    }
}

impl Statement {
    pub fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(self, _token: i64) {
    }
}

pub fn new(token: i64) -> Root {
    Root::new(token)
}
