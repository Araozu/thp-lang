use std::fmt::format;

use markdown::mdast::Heading;

use super::Printable;


impl Printable for Heading {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(node.to_html())
        }

        let text: String = result.into_iter().collect();

        format!("<h{}>{}</h{}>", self.depth, text, self.depth)
    }
}

