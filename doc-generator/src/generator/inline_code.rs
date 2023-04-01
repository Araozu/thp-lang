use markdown::mdast::InlineCode;

use super::Printable;
use misti::TokenType;

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

fn highlight(input: &String) -> String {
    let tokens = misti::tokenize(&input);

    if tokens.is_err() {
        eprintln!(
            "Found a lexical error processing an inline-code with input {}",
            input
        );
        return input.clone();
    }

    let mut output = input.clone();

    for token in tokens.unwrap() {
        match &token.token_type {
            TokenType::Datatype => {
                let start_pos = token.position;
                let end_pos = token.get_end_position();

                let range = start_pos..end_pos;
                let html = format!("<span class=\"token class-name\">{}</span>", token.value);

                output.replace_range(range, html.as_str());
            }
            _ => {}
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_simple_string() {
        assert_eq!("sample", highlight(&String::from("sample")))
    }

    #[test]
    fn should_highlight_datatype() {
        assert_eq!(
            "<span class=\"token class-name\">Num</span>",
            highlight(&String::from("Num"))
        )
    }
}
