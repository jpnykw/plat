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
}

pub fn get() -> ExprAST {
        let root = ExprAST::new(0);
    // println!("{:#?}", root);
    root
}

