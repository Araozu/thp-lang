use markdown::mdast::Code;

use crate::highlighter::highlight;

use super::Printable;

impl Printable for Code {
    fn to_html(&self) -> String {
        let code = highlight(&self.value);

        if let Some(lang) = &self.lang {
            format!("<pre class=\"language-{}\">{}</pre>", lang, code)
        } else {
            format!("<pre class=\"language-none\">{}</pre>", code)
        }
    }

    fn get_text(&self) -> String {
        panic!("Code cannot return its raw text")
    }
}
