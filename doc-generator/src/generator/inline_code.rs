use markdown::mdast::InlineCode;

use super::Printable;
use crate::highlighter::highlight;

impl Printable for InlineCode {
    fn to_html(&self) -> String {
        /*
        let tokens = misti::tokenize(&self.value);
        println!("INLINE CODE ==== tokens ====\n\n{:?}\n\n==== code ====\n\n{}\n\n", tokens, self.value);

        let s = self.value
            .replace("<", "&lt;")
            .replace(">", "&gt;");
         */

        format!("<code class=\"border border-border-color dark:border-transparent\">{}</code>", highlight(&self.value))
    }

    fn get_text(&self) -> String {
        self.value.clone()
    }
}
