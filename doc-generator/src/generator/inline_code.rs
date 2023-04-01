use markdown::mdast::InlineCode;

use super::Printable;

impl Printable for InlineCode {
    fn to_html(&self) -> String {
        let s = self.value
            .replace("<", "&lt;")
            .replace(">", "&gt;");
        
        format!("<code>{}</code>", s)
    }

    fn get_text(&self) -> String {
        self.value.clone()
    }
}
