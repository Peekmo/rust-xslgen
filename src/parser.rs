use std::vec::Vec;

enum ParserContext {
    Empty,
    Tag,
    Attribute,
    Expression,
    InsideString
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

impl Node {
    pub fn new() -> Self {
        Node {
            name: String::new(),
            namespace: None,
            attributes: Vec::new(),
            children: None,
            parent: None
        }
    }
}

impl Attribute {
    pub fn new() -> Self {
        Attribute {
            key: String::new(),
            value: String::new()
        }
    }
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

    pub fn parse(&mut self) {
        for line in self.xslg_file.iter() {
            self.buffer.clear();

            for current_char in line.as_bytes().iter() {
                let cha = *current_char as char;

                match self.context {
                    /**
                     * --------------  Empty context !
                     */
                    ParserContext::Empty => {
                        match cha {
                            '@' | '.' => {
                                let mut node = Box::new(Node::new());

                                node.namespace = match cha {
                                    '@' => { Some(String::from_str("xsl")) },
                                    '.' => { Some(self.buffer.clone()) },
                                    _   => { None }
                                };

                                self.context = ParserContext::Tag;
                                self.current_node = Some(node);
                                self.buffer.clear();
                            },
                            ' ' => {
                                match self.buffer.as_slice() {
                                    "if" => {},
                                    "elsif" => {},
                                    "else" => {},
                                    "" => {},
                                    _ => {
                                        let mut node = Box::new(Node::new());
                                        node.name = self.buffer.clone();

                                        self.context = ParserContext::Tag;
                                        self.current_node = Some(node);
                                        self.buffer.clear();
                                    }
                                }
                            },
                            _ => { self.buffer.push(cha); }
                        }
                    }
                    _ => { panic!("Transpiler error -- Unknown context"); }
                }
                self.buffer.push(*current_char as char);
            }

            println!("{}", self.buffer);
        }
    }
}
