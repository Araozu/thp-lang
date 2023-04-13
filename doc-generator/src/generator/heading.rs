use markdown::mdast::Heading;

use crate::utils;

use super::Printable;

impl Printable for Heading {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(node.to_html())
        }

        let text: String = result.into_iter().collect();

        if self.depth < 4 {
            let html_fragment_text = utils::to_html_fragment(&self.get_text());

            format!(
                "<h{} id=\"{}\" class=\"heading-linked\"><a href=\"#{}\">{}</a></h{}>",
                self.depth, html_fragment_text, html_fragment_text, text, self.depth
            )
        } else {
            format!("<h{}>{}</h{}>", self.depth, text, self.depth)
        }
    }

    fn get_text(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(node.get_text())
        }

        result.join("-")
    }
}
