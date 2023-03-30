use markdown::mdast::Text;

use super::Printable;

impl Printable for Text {
    fn to_html(&self) -> String {
        self.value.clone()
    }

    fn get_text(&self) -> String {
        self.value.clone()
    }
}
