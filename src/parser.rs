use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use core::ops::DerefMut;
use core::ops::Deref;

/// ParserContext's different states
/// So brillant !
enum ParserContext {
    Empty,
    Tag,
    Attribute,
    Expression,
    InsideString,
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
    pub fn new() -> Self {
        Attribute {
            key: String::new(),
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
        // We will read all the lines.. Yes.. So boring.
        for line in self.xslg_file.clone().iter() {
            self.buffer.clear();

            match self.context {
                ParserContext::NewBlock | ParserContext::Empty => {},
                _ => self.current_node = match self.current_node {
                    None => { None },
                    Some(ref node) => { node.borrow().deref().parent.clone() }
                }
            }

            self.context = ParserContext::Empty;

            // A char by char work.. Yes.. That's life
            // THUG LIFE
            for current_char in line.as_bytes().iter() {
                let cha = *current_char as char;

                match self.context {
                    // Empty context ? Yes we don't know where we are !
                    ParserContext::Empty => {
                        self.parse_empty_context(cha);
                    },

                    ParserContext::Tag   => {
                        self.parse_tag_context(cha);
                    },

                    // WTF ! Poor lazy man, do your job ! (Yes, I'm not paid for it but..)
                    _ => { panic!("Transpiler error -- Unimplemented context"); }
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
                    None => { panic!("Syntax error - Found '}' (end block) without a block before"); },
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

    // A new character in tag context comes will be welcome here :)
    // Hey !! I'M DORA !!!
    fn parse_tag_context(&mut self, cha: char) {
        match cha {
            '{' => {
                match self.current_node {
                    None => { panic!("No node found"); },
                    Some(ref node) => {
                        node.borrow_mut().deref_mut().name = self.buffer.clone();
                    }
                }

                self.context = ParserContext::NewBlock;
                self.buffer.clear();
            }

            _ => { self.buffer.push(cha); }
        }
    }
}
