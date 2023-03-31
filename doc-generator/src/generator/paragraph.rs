use markdown::mdast::Paragraph;

use super::Printable;

impl Printable for Paragraph {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(node.to_html())
        }

        let text: String = result.into_iter().collect();

        format!("<p>{}</p>", text)
    }

    fn get_text(&self) -> String {
        panic!("Paragraph cannot return its raw text")
    }
}
