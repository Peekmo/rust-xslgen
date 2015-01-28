use std::string::String;
use parser::Node;
use std::rc::Rc;
use std::cell::RefCell;
use core::ops::Deref;

/// Builds an XSL string from parsed nodes
pub fn build_from_nodes(nodes: &Vec<Rc<RefCell<Node>>>) -> String {
    let mut string = String::new();
    let mut tabs = 0;

    build_string(nodes, &mut string, &mut tabs);

    return string;
}

/// Builds the string from the current nodes
fn build_string(nodes: &Vec<Rc<RefCell<Node>>>, string: &mut String, tabs: &mut isize) {
    for node in nodes.iter() {
        let mut_node = node.borrow();
        let deref_node = mut_node.deref();

        add_tabs(string, tabs);
        string.push_str("<");

        add_balise_name(deref_node, string);

        for attr in deref_node.attributes.iter() {
            string.push_str(format!(" {}={}", attr.key, attr.value).as_slice());
        }

        match deref_node.children {
            None => {
                string.push_str("/>\n");
            },
            Some (ref children) => {
                string.push_str(">\n");

                *tabs += 1;
                build_string(children, string, tabs);
                *tabs -= 1;

                string.push_str("</");
                add_balise_name(deref_node, string);
                string.push_str(">\n");
            }
        }
    }
}

/// Adds the balise name to the string (namespace:name or just name if no namespace)
fn add_balise_name(node: &Node, string: &mut String) {
    match node.namespace {
        Some(ref namespace) => string.push_str(format!("{}:", namespace).as_slice()),
        None => {}
    }

    string.push_str(node.name.as_slice());
}

/// Add the given number of tabs to the given string
fn add_tabs(string: &mut String, nb: &mut isize) {
    for _ in range(0, *nb) {
        string.push_str("\t");
    }
}
