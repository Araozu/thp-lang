use markdown::mdast::Code;

use super::Printable;

impl Printable for Code {
    fn to_html(&self) -> String {
        format!("<pre>{}</pre>", self.value)
    }

    fn get_text(&self) -> String {
        panic!("Code cannot return its raw text")
    }
}
