use markdown::mdast::Code;

use super::Printable;

impl Printable for Code {
    fn to_html(&self) -> String {
        if let Some(lang) = &self.lang {
            format!("<pre class=\"language-{}\">{}</pre>", lang, self.value)
        } else {
            format!("<pre class=\"language-none\">{}</pre>", self.value)
        }
    }

    fn get_text(&self) -> String {
        panic!("Code cannot return its raw text")
    }
}
