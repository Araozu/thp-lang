use markdown::mdast::InlineCode;

use super::Printable;

impl Printable for InlineCode {
    fn to_html(&self) -> String {
        format!("<code>{}</code>", self.value)
    }

    fn get_text(&self) -> String {
        self.value.clone()
    }
}
