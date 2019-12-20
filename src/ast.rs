// use std::boxed::Box;

#[derive(Debug)]
pub enum Types {
    Exp(Expression),
    Stat(Statement)
}

#[derive(Debug)]
pub struct Root {
    pub token: i64,
    // pub node: Vec<Option<Box<Types>>>
    pub node: Vec<Option<Vec<Types>>>
}

#[derive(Debug)]
pub struct Expression {
    pub token: i64,
    // pub node: Vec<Option<Box<Types>>>
    pub node: Vec<Option<Vec<Types>>>
}

#[derive(Debug)]
pub struct Statement {
    pub token: i64,
    // pub node: Vec<Option<Box<Types>>>
    pub node: Vec<Option<Vec<Types>>>
}

impl Root {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(&mut self, token: i64, mode: i64) {
        if mode == 0 {
            let node = Types::Exp(Expression::new(token));
            // self.node.push(Some(Box::new(node)));
            self.node.push(Some(vec![node]));
        } else {
            let node = Types::Stat(Statement::new(token));
            // self.node.push(Some(Box::new(node)));
            self.node.push(Some(vec![node]));
        }
    }
}

impl Expression {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(self, token: i64) {
    }
}

impl Statement {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    pub fn insert(self, token: i64) {
    }
}

pub fn new(token: i64) -> Root {
    Root::new(token)
}
