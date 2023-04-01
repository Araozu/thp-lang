use markdown::mdast::Emphasis;

use crate::utils;

use super::Printable;

impl Printable for Emphasis {
    fn to_html(&self) -> String {
        let html = utils::collect_children_html(&self.children);

        format!("<em>{}</em>", html)
    }

    fn get_text(&self) -> String {
        utils::collect_children_text(&self.children)
    }
}
