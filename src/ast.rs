use std::boxed::Box;

#[derive(Debug)]
pub struct ExprAST {
    token: i64,
    node: Vec<Option<Box<ExprAST>>>
}

impl ExprAST {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            node: Vec::with_capacity(32)
        }
    }

    fn _insert(&mut self,  token: i64) {
        let node = Self::new(token);
        self.node.push(Some(Box::new(node)));
    }
}

pub fn new(token: i64) -> ExprAST {
    ExprAST::new(token)
}
