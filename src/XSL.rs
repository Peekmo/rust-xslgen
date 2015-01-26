use std::string::String;
use parser::Node;

/// Builds an XSL string from parsed nodes
pub fn build_from_nodes(nodes: &Vec<Node>) -> String {
    let mut string = String::new();
    let mut tabs = 0;

    build_string(nodes, &mut string, &mut tabs);

    return string;
}

/// Builds the string from the current nodes
fn build_string(nodes: &Vec<Node>, string: &mut String, tabs: &mut isize) {
    for node in nodes.iter() {
        add_tabs(string, tabs);
        string.push_str("<");
    }
}

/// Add the given number of tabs to the given string
fn add_tabs(string: &mut String, nb: &mut isize) {
    for i in range(0, *nb) {
        string.push_str("\t");
    }
}
