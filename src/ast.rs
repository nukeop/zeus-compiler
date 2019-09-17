use token::Token;

pub enum ASTNodeType {
    Root,
    Token
}

pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub token: Option<Token>,
    pub children: Vec<ASTNode>
}

impl ASTNode {
    pub fn new(node_type: ASTNodeType) -> ASTNode {
        ASTNode {
            node_type,
            token: None,
            children: vec![]
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    pub fn build_tree(tokens: Vec<Token>) -> ASTNode {
        let mut root = ASTNode::new(ASTNodeType::Root);

        let mut iter = tokens.iter();
        let mut current_token = iter.next();
        while (current_token != None) {
            let unwrapped_token = current_token.unwrap();
            current_token = iter.next();
        }

        return root;
    }
}
