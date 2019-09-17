use std::fmt;
use token::Token;

#[derive(Debug)]
pub enum ASTNodeType {
    Root,
    Token
}

#[derive(Debug)]
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

    pub fn add_token(mut self, token: Token) -> ASTNode {
        self.token = Some(token);
        self
    }

    pub fn build_tree(tokens: Vec<Token>) -> ASTNode {
        let mut root = ASTNode::new(ASTNodeType::Root);

        let mut iter = tokens.iter();
        let mut current_token = iter.next();
        while (current_token != None) {
            let owned_token = Token::from_token(current_token.unwrap());
            let node = ASTNode::new(ASTNodeType::Token).add_token(owned_token);

            println!("{}", node);
            current_token = iter.next();
        }

        return root;
    }
}

impl fmt::Display for ASTNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNodeType::Root => write!(f, "ASTNodeType::Root"),
            ASTNodeType::Token => write!(f, "ASTNodeType::Token")
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode::{}", self.node_type)
    }
}
