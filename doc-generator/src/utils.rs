use markdown::mdast::Node;

use crate::generator::Printable;

pub fn to_html_fragment(text: &String) -> String {
    text.clone().replace(" ", "-")
}

pub fn collect_children_html(vec: &Vec<Node>) -> String {
    let mut result = Vec::<String>::new();

    for node in vec {
        result.push(node.to_html())
    }

    result.into_iter().collect()
}

pub fn collect_children_text(vec: &Vec<Node>) -> String {
    let mut result = Vec::<String>::new();

    for node in vec {
        result.push(node.get_text())
    }

    result.join("-")
}
