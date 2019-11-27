use super::lexer;
use std::boxed::Box;

#[derive(Debug)]
pub struct AST {
    token: i64,
    node_left: Option<Box<AST>>,
    node_right: Option<Box<AST>>
}

impl AST {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node_left: None,
            node_right: None
        }
    }
}

pub fn get() -> AST {
    let root = AST::new(0);
    // println!("{:#?}", root);
    root
}
