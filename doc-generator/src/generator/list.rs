use markdown::mdast::{List, ListItem, Node};

use crate::utils;

use super::Printable;

impl Printable for List {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(format!("<li class=\"py-2\">{}</li>", node.to_html()))
        }

        let str: String = result.into_iter().collect();

        format!("<ol class=\"list-decimal list-inside\">{}</ol>", str)
    }

    fn get_text(&self) -> String {
        panic!("List cannot return it's raw text")
    }
}

impl Printable for ListItem {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            let s = match node {
                Node::Paragraph(p) => utils::collect_children_html(&p.children),
                _ => panic!("A thing other than Paragraph inside ListItem (?)"),
            };
            result.push(format!("{}", s))
        }

        result.into_iter().collect()
    }

    fn get_text(&self) -> String {
        panic!("ListItem cannot return it's raw text")
    }
}
