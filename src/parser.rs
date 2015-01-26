use std::vec::Vec;

pub struct Node {
    pub name: String,
    pub namespace: Option<String>,
    pub attributes: Option<Vec<Box<Attribute>>>,
    pub children: Option<Vec<Box<Node>>>,
    pub parent: Option<Box<Node>>
}

pub struct Attribute {
    pub key: String,
    pub value: String
}

pub struct Parser {
    xslg_file: Box<Vec<String>>,
    pub nodes: Vec<Node>,
    pub current_node: Option<Box<Node>>
}

impl Parser {
    pub fn new(xslg_file: Box<Vec<String>>) -> Self {
        Parser {
            xslg_file: xslg_file,
            nodes: Vec::new(),
            current_node: None
        }
    }
}
