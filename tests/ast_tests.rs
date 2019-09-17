extern crate zeus_compiler;

#[cfg(test)]
mod ast_tests {
    use zeus_compiler::source_file::SourceFile;
    use zeus_compiler::token::Token;
    use zeus_compiler::ast::{ASTNode, ASTNodeType};

    #[test]
    fn new_node() {
        let labels = vec![];
        let token = Token::from_value("NOOP", &labels);
        let mut node = ASTNode::new(ASTNodeType::Token).add_token(token);
        assert_eq!(node.token.unwrap(), Token::from_value("NOOP", &labels));
    }

    #[test]
    fn empty_node() {
        let node = ASTNode::new(ASTNodeType::Root);
        assert_eq!(node.token, None);
    }

    #[test]
    fn build_tree() {
        let mut sf = SourceFile::new();
        sf.content=Some("
        begin:
        MVIX 20
        MVIY 50
        Copy 1 1030
        test:
        cpid 1234 1236
        ".to_string());
        sf.tokenize().unwrap();
        let tree = ASTNode::build_tree(sf.tokens);
    }
}
