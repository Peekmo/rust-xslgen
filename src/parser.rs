use std::vec::Vec;

enum ParserContext {
    Empty,
    Tag,
    Attribute,
    Expression
}

pub struct Node {
    pub name: String,
    pub namespace: Option<String>,
    pub attributes: Vec<Box<Attribute>>,
    pub children: Option<Vec<Box<Node>>>,
    pub parent: Option<Box<Node>>
}

pub struct Attribute {
    pub key: String,
    pub value: String
}

pub struct Parser {
    xslg_file: Box<Vec<String>>,
    current_attribute: Option<Box<Attribute>>,
    current_node: Option<Box<Node>>,
    buffer: String,
    context: ParserContext,
    pub nodes: Vec<Box<Node>>,
}

impl Parser {
    pub fn new(xslg_file: Box<Vec<String>>) -> Self {
        Parser {
            xslg_file: xslg_file,
            nodes: Vec::new(),
            current_node: None,
            current_attribute: None,
            buffer: String::new(),
            context: ParserContext::Empty
        }
    }

    pub fn parse(&self) {
        for line in self.xslg_file.iter() {
            println!("{}", line);
        }
    }
}
