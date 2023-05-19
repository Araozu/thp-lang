use misti::TokenType;

#[macro_export]
macro_rules! replace {
    ($classes:literal, $token:ident, $offset:ident, $output:ident) => {{
        let start_pos = $token.position;
        let end_pos = $token.get_end_position();

        let range = (start_pos + $offset)..(end_pos + $offset);
        let html = format!("<span class=\"token {}\">{}</span>", $classes, $token.value);

        $offset += 28 + $classes.len();

        $output.replace_range(range, html.as_str());
    }};
}

pub fn highlight(input: &String) -> String {
    // The tokens come in order
    let tokens = misti::tokenize(&input);

    if tokens.is_err() {
        // eprintln!("Found a lexical error processing code.\n{:?}", tokens);
        return input.clone();
    }

    let mut output = input.clone();
    // Offset to the position of the tokens in the string, to allow
    // several tokens to be highlighted
    let mut offset = 0;

    for token in tokens.unwrap() {
        match &token.token_type {
            TokenType::Datatype => replace!("class-name", token, offset, output),
            TokenType::Number => replace!("number", token, offset, output),
            TokenType::Identifier if token.value == "true" || token.value == "false" => {
                replace!("keyword", token, offset, output)
            }
            TokenType::String => replace!("string", token, offset, output),
            TokenType::Comment => replace!("comment", token, offset, output),
            TokenType::VAL | TokenType::VAR => replace!("keyword", token, offset, output),
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
