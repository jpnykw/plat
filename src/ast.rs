use std::boxed::Box;

#[derive(Debug)]
pub struct ExprAST {
    token: i64,
    llvm_ir: String,
    node_left: Option<Box<ExprAST>>,
    node_right: Option<Box<ExprAST>>
}

impl ExprAST {
    fn new(token: i64) -> Self {
        Self {
            token: token,
            llvm_ir: String::new(),
            node_left: None,
            node_right: None
        }
    }

    fn _insert(&mut self,  token: i64) {
        let node = Self::new(token);
        self.node_left = Some(Box::new(node));

        self.llvm_ir = String::from(match token {
            _ => "# None"
        });
    }
}

pub fn new(token: i64) -> ExprAST {
    ExprAST::new(token)
}
