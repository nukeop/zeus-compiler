extern crate zeus_compiler;

#[cfg(test)]
mod ast_tests {
    use zeus_compiler::token::Token;
    use zeus_compiler::ast::{ASTNode, ASTNodeType};

    #[test]
    fn new_node() {
        let labels = vec![];
        let token = Token::from_value("NOOP", &labels);
        let mut node = ASTNode::new(ASTNodeType::Token);
        node.add_token(token);
        assert_eq!(node.token.unwrap(), Token::from_value("NOOP", &labels));
    }
}
