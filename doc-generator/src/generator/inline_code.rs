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
    // The tokens come in order
    let tokens = misti::tokenize(&input);

    if tokens.is_err() {
        eprintln!(
            "Found a lexical error processing an inline-code with input {}",
            input
        );
        return input.clone();
    }

    let mut output = input.clone();
    // Offset to the position of the tokens in the string, to allow
    // several tokens to be highlighted
    let mut offset = 0;

    for token in tokens.unwrap() {
        match &token.token_type {
            TokenType::Datatype => {
                let start_pos = token.position;
                let end_pos = token.get_end_position();

                let range = (start_pos + offset)..(end_pos + offset);
                let html = format!("<span class=\"token class-name\">{}</span>", token.value);

                // 38 is the number of extra characters added to the token
                offset += 38;

                output.replace_range(range, html.as_str());
            }
            TokenType::Number => {
                let start_pos = token.position;
                let end_pos = token.get_end_position();

                let range = (start_pos + offset)..(end_pos + offset);
                let html = format!("<span class=\"token number\">{}</span>", token.value);

                offset += 34;

                output.replace_range(range, html.as_str());
            }
            TokenType::String => {
                let start_pos = token.position;
                let end_pos = token.get_end_position();

                let range = (start_pos + offset)..(end_pos + offset);
                let html = format!("<span class=\"token string\">\"{}\"</span>", token.value);

                offset += 34;

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

    #[test]
    fn should_highlight_number() {
        assert_eq!(
            "<span class=\"token number\">322</span>",
            highlight(&String::from("322"))
        )
    }

    #[test]
    fn should_highlight_string() {
        assert_eq!(
            "<span class=\"token string\">\"Hello\"</span>",
            highlight(&String::from("\"Hello\""))
        )
    }

    #[test]
    fn should_highlight_multiple_tokens() {
        assert_eq!(
            "<span class=\"token class-name\">Str</span> x = <span class=\"token number\">322</span>",
            highlight(&String::from("Str x = 322"))
        );

        assert_eq!(
            "<span class=\"token class-name\">Str</span> x = <span class=\"token string\">\"hello\"</span> <span class=\"token number\">322</span>",
            highlight(&String::from("Str x = \"hello\" 322"))
        );
    }
}
