use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use core::ops::DerefMut;
use core::ops::Deref;
use core::ops::Index;
use std::ops::RangeFrom;

/// ParserContext's different states
/// So brillant !
enum ParserContext {
    Empty,
    Tag,
    Attributes,
    Expression,
    InsideStringContent,
    InsideStringAttribute,
    NewBlock
}

/// A node is a complete <xsl:when test="peekmo_qi > einstein_qi"> (for example)
#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub namespace: Option<String>,
    pub attributes: Vec<Box<Attribute>>,
    pub children: Option<Vec<Rc<RefCell<Node>>>>,
    pub parent: Option<Rc<RefCell<Node>>>
}

/// A node attribute element (test="peekmo_qi > einstein_qi")
#[derive(Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String
}

/// All the process structure
/// <--- MAGIC IS STORED HERE !!!
pub struct Parser {
    xslg_file: Box<Vec<String>>,
    current_attribute: Option<Box<Attribute>>,
    current_node: Option<Rc<RefCell<Node>>>,
    buffer: String,
    context: ParserContext,
    line_number: isize,
    char_number: isize,
    pub nodes: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    /// Creates the node with some params.
    /// Parser is used to determine the parent (if any)
    pub fn new(parser: &Parser, namespace: Option<String>, name: Option<String>) -> Self {
        Node {
            name: match name {
                Some (n) => { n },
                None     => String::new()
            },
            namespace: namespace,
            attributes: Vec::new(),
            children: None,
            parent: match parser.current_node {
                Some (ref n) => { Some(n.clone()) },
                None         => { None }
            }
        }
    }
}

impl Attribute {
    /// LOL
    pub fn new(key: String) -> Self {
        Attribute {
            key: key,
            value: String::new()
        }
    }
}

impl Parser {
    /// Nothing interesting
    pub fn new(xslg_file: Box<Vec<String>>) -> Self {
        Parser {
            xslg_file: xslg_file,
            nodes: Vec::new(),
            current_node: None,
            current_attribute: None,
            buffer: String::new(),
            line_number: 0,
            char_number: 0,
            context: ParserContext::Empty
        }
    }

    /// Sets the current_node to the process
    /// It will also set node's parent if needed
    fn set_current_node(&mut self, node: Rc<RefCell<Node>>) {
        let mut_node = node.borrow();
        let deref_node = mut_node.deref();

        // Updates children of the given parent.
        // Yes, I can build a family. So genius !
        match deref_node.parent {
            None        => { self.nodes.push(node.clone()); },
            Some(ref parent) => {
                let mut mut_parent = parent.borrow_mut();
                let mut ref_parent = mut_parent.deref_mut();

                match ref_parent.children {
                    Some(ref mut children) => {
                        children.push(node.clone());
                    },
                    None => {
                        let mut children = Vec::new();
                        children.push(node.clone());

                        ref_parent.children = Some(children);
                    }
                }
            }
        }

        self.current_node = Some(node.clone());
    }

    /// Parse its xslg_file to store a Node representation of the file (to build the XSL file from
    /// them !!)
    /// <--- MAGIC HAPPENED HERE !!!
    /// <--- HEARTH OF THE EARTH !!!
    /// <--- THANK YOU LORD !!!
    pub fn parse(&mut self) {
        self.line_number = 0;

        // We will read all the lines.. Yes.. So boring.
        for line in self.xslg_file.clone().iter() {
            self.line_number += 1;
            self.buffer.clear();

            // What to do with the last context
            match self.context {
                ParserContext::InsideStringContent => {},
                ParserContext::Attributes => {
                    match self.current_attribute {
                        None => {},
                        Some (ref attribute) => { self.parsing_error(format!("Unexpected new line. Attribute {} is not closed", attribute.key).as_slice()); }
                    }
                },
                ParserContext::NewBlock | ParserContext::Empty | ParserContext::Expression => {
                    self.context = ParserContext::Empty;
                },
                ParserContext::InsideStringAttribute => {
                    self.parsing_error("Unexpected new line");
                },
                ParserContext::Tag => {
                    self.current_node = match self.current_node {
                        None => { None },
                        Some(ref node) => { node.borrow().deref().parent.clone() }
                    };

                    self.context = ParserContext::Empty;
                }
            }

            self.char_number = 0;

            // A char by char work.. Yes.. That's life
            // THUG LIFE
            for current_char in line.as_bytes().iter() {
                self.char_number += 1;
                let cha = *current_char as char;

                match self.context {
                    // Empty context ? Yes we don't know where we are !
                    ParserContext::Empty => {
                        self.parse_empty_context(cha);
                    },

                    ParserContext::Tag   => {
                        self.parse_tag_context(cha);
                    },

                    ParserContext::Attributes => {
                        self.parse_attribute_context(cha);
                    },

                    ParserContext::InsideStringAttribute | ParserContext::InsideStringContent => {
                        match cha {
                            '"' => {
                                if !self.buffer.is_empty() && (self.buffer.index(&RangeFrom {start: self.buffer.len() - 1}) == "\\") {
                                    self.buffer.push(cha);
                                } else {
                                    match self.context {
                                        ParserContext::InsideStringAttribute => {
                                            self.buffer.push(cha);
                                            self.context = ParserContext::Attributes;
                                        },
                                        ParserContext::InsideStringContent   => { self.context = ParserContext::Empty }
                                        _ => { self.parsing_error("How the hell did you come here ?"); }
                                    }
                                }
                            },
                            _ => { self.buffer.push(cha); }
                        }
                    }

                    // WTF ! Poor lazy man, do your job ! (Yes, I'm not paid for it but..)
                    _ => { self.parsing_error("Transpiler error -- Unimplemented context"); }
                }
            }
        }
    }

    /// A new char in an Empty context comes here to be judged.
    fn parse_empty_context(&mut self, cha: char) {
        match cha {
            // '@' is an XSL tag
            // '.' is a separator for namespace.tagname
            //
            // So... Let's build the tag !!!
            '@' | '.' => {
                let mut node = Node::new(self, None, None);

                node.namespace = match cha {
                    '@' => { Some(String::from_str("xsl")) },
                    '.' => { Some(self.buffer.clone()) },
                    _   => { None }
                };

                self.context = ParserContext::Tag;
                self.set_current_node(Rc::new(RefCell::new(node)));
                self.buffer.clear();
            },

            // A space is breaking the Empty context !!! DAAAAA
            ' ' => {
                // Let's have a look to the buffer
                match self.buffer.as_slice() {
                    // Nothing in the buffer ? False alert
                    // It should never happened, but.. I'm not sure so.. :D
                    "" => {},

                    // Conditional expressions (for too perhaps ?)
                    "if" | "elsif" | "else" => {
                        let mut node = Node::new(self, None, None);

                        // TODO update here after antoine's answer (XSL MASTER)
                        node.namespace = Some(String::from_str("xsl"));
                        node.name =  String::from_str("when");

                        self.context = ParserContext::Expression;
                        self.set_current_node(Rc::new(RefCell::new(node)));
                        self.buffer.clear();
                    },

                    // Everything else is a tag without namespace (like a lonely cowboy)
                    _ => {
                        let node = Node::new(
                            self,
                            None,
                            Some(self.buffer.clone())
                            );

                        self.context = ParserContext::Tag;
                        self.set_current_node(Rc::new(RefCell::new(node)));
                        self.buffer.clear();
                    }
                }
            },

            // End of a block (so so sad...)
            // Changes the current_node to the current one parent
            '}' => {
                self.current_node = match self.current_node {
                    None => {
                        self.parsing_error("Syntax error - Found '}' (end block) without a block before");
                        None
                    },
                    Some(ref node) => {
                        match node.borrow().deref().parent {
                            None => { None },
                            Some(ref parent) => { Some(parent.clone()) }
                        }
                    }
                }
            }

            // If nothing interesting happened... let's continue !
            _ => { self.buffer.push(cha); }
        }
    }

    /// A new character in tag context comes will be welcome here :)
    /// Hey !! I'M DORA !!!
    fn parse_tag_context(&mut self, cha: char) {
        match cha {
            // Starting block (LOL) attributes or tags
            '{' | '[' => {
                match self.current_node {
                    None => { self.parsing_error("No node found"); },
                    Some(ref node) => {
                        if node.borrow_mut().deref_mut().name.is_empty() {
                            node.borrow_mut().deref_mut().name = self.buffer.clone();
                        }
                    }
                }

                self.context = match cha {
                    '{' => ParserContext::NewBlock,
                    '[' => ParserContext::Attributes,
                    _   => { panic!("How did you come here ? o.O"); }
                };

                self.buffer.clear();
            },

            ' ' => {},
            _   => {
                match self.current_node {
                    None => { self.parsing_error("No node found"); },
                    Some(ref node) => {
                        if !node.borrow_mut().deref_mut().name.is_empty() {
                            self.parsing_error(format!("Unexpected character - Expected '[' or '{}'", "{").as_slice());
                        }
                    }
                }

                self.buffer.push(cha);
            }
        }
    }

    /// When we are in the attribute context... We are building attributes :)
    fn parse_attribute_context(&mut self, cha: char) {
        match cha {
            ']' => {
                match self.current_attribute {
                    None => {},
                    Some(ref attribute) => {
                        let mut attr = attribute.clone();
                        attr.value = self.buffer.clone();

                        match self.current_node {
                            None => { self.parsing_error("Found the end of an attribute without node"); },
                            Some(ref node) => {
                                node.borrow_mut().deref_mut().attributes.push(attr.clone());
                            }
                        }
                    }
                }

                self.current_attribute = None;
                self.buffer.clear();
                self.context = ParserContext::Tag;
            },
            ':' => {
                // If there's a ':' with en empty buffer... You're stupid. Sorry for you.
                if self.buffer.len() == 0 {
                    self.parsing_error("Unexpected token ':'");
                }

                self.current_attribute = Some(Box::new(Attribute::new(self.buffer.clone())));
                self.buffer.clear();
            },
            ',' => {
                match self.current_attribute {
                    None => { self.parsing_error("Unexpected character (attribute delimiter)"); },
                    Some(ref attribute) => {
                        let mut attr = attribute.clone();
                        attr.value = self.buffer.clone();

                        match self.current_node {
                            None => { self.parsing_error("Found the end of an attribute without node"); },
                            Some(ref node) => {
                                node.borrow_mut().deref_mut().attributes.push(attr.clone());
                            }
                        }
                    }
                }

                self.current_attribute = None;
                self.buffer.clear();
            },
            '"' => {
                match self.current_attribute {
                    None => { self.parsing_error("Unexpected token \""); },
                    Some (ref attribute) => {
                        if !self.buffer.is_empty() {
                            self.parsing_error("Unexpected token \"");
                        }
                    }
                }

                self.buffer.push(cha);
                self.context = ParserContext::InsideStringAttribute;
            },
            ' ' => {},
            _ => { self.buffer.push(cha); }
        }
    }

    /// Sends the error message !
    /// The killer method !
    fn parsing_error(&self, message: &str) {
        panic!("Parser error line {} char {} - {}", self.line_number, self.char_number, message);
    }
}

