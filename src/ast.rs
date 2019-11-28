use super::lexer;
use std::boxed::Box;

#[derive(Debug)]
pub struct ExprAST {
    token: i64,
    node_left: Option<Box<ExprAST>>,
    node_right: Option<Box<ExprAST>>
}

impl ExprAST {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node_left: None,
            node_right: None
        }
    }

    fn insert(&mut self,  token: i64) {
        let node = Self::new(token);
        self.node_left = Some(Box::new(node));
    }
}

pub fn get() -> ExprAST {
    let mut root = ExprAST::new(0);
    root.insert(-7);

    root
}

