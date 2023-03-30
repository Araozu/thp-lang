pub fn to_html_fragment(text: &String) -> String {
    text.clone().to_lowercase().replace(" ", "-")
}
