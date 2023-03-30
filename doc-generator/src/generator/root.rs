use markdown::mdast;

use super::Printable;


impl Printable for mdast::Root {
    fn to_html(&self) -> String {
        let mut result = Vec::<String>::new();

        for node in &self.children {
            result.push(node.to_html())
        }

        result.into_iter().collect()
    }
}
