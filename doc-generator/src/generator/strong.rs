use markdown::mdast::Strong;

use crate::utils;

use super::Printable;

impl Printable for Strong {
    fn to_html(&self) -> String {
        let text = utils::collect_children_html(&self.children);

        format!("<b>{}</b>", text)
    }

    fn get_text(&self) -> String {
        utils::collect_children_text(&self.children)
    }
}
