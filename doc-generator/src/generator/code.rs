use markdown::mdast::Code;

use super::highlighter::highlight;

use super::Printable;

impl Printable for Code {
    fn to_html(&self) -> String {
        let code = highlight(&self.value);

        if let Some(lang) = &self.lang {
            if lang == "nav" {
                return generate_nav_html(&self.value);
            }

            format!("<pre class=\"language-{}\">{}</pre>", lang, code)
        } else {
            format!("<pre class=\"language-none\">{}</pre>", code)
        }
    }

    fn get_text(&self) -> String {
        panic!("Code cannot return its raw text")
    }
}

fn generate_nav_html(data: &String) -> String {
    use toml::{Table, Value};

    let table = data.parse::<Table>().unwrap();

    let previous = match table.get("previous") {
        Some(Value::Table(t)) => match (t.get("href"), t.get("title")) {
            (Some(Value::String(href)), Some(Value::String(title))) => {
                format!(
                    "
                    <a
                    class=\"inline-block px-4 py-2 transition-colors
                    border border-border-color hover:border-c2-primary
                    hover:text-c2-primary
                    rounded-md\"
                    href=\"{}\"
                    >
                        <span class=\"text-xs\">Previous</span>
                        <br>
                        <span class=\"font-bold\">{}</span>
                    </a>
                    ",
                    href, title
                )
            }
            _ => panic!("TOML error: `previous` doesn't have a href and title string."),
        },
        Some(_) => panic!("TOML error: `previous` is not a table."),
        _ => String::from("<div></div>"),
    };

    let next = match table.get("next") {
        Some(Value::Table(t)) => match (t.get("href"), t.get("title")) {
            (Some(Value::String(href)), Some(Value::String(title))) => {
                format!(
                    "
                    <a
                    class=\"inline-block px-4 py-2 transition-colors
                    border border-border-color hover:border-c2-primary
                    hover:text-c2-primary
                    rounded-md text-right\"
                    href=\"{}\"
                    >
                        <span class=\"text-xs\">Next</span>
                        <br>
                        <span class=\"font-bold\">{}</span>
                    </a>
                    ",
                    href, title
                )
            }
            _ => panic!("TOML error: `next` doesn't have a href and title string."),
        },
        Some(_) => panic!("TOML error: `next` is not a table."),
        _ => String::from("<div></div>"),
    };

    format!(
        "<div class=\"grid grid-cols-2 gap-4 my-16\">{}{}</div>",
        previous, next
    )
}
