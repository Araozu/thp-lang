use markdown::mdast::Node;

mod code;
mod emphasis;
mod heading;
mod inline_code;
mod list;
mod paragraph;
mod root;
mod strong;
mod text;

mod highlighter;

pub trait Printable {
    fn to_html(&self) -> String;
    fn get_text(&self) -> String;
}

impl Printable for Node {
    fn to_html(&self) -> String {
        match self {
            Node::Root(root) => root.to_html(),
            Node::Heading(heading) => heading.to_html(),
            Node::Text(text) => text.to_html(),
            Node::Paragraph(p) => p.to_html(),
            Node::ThematicBreak(_) => String::from("<hr />"),
            Node::InlineCode(i) => i.to_html(),
            Node::Code(c) => c.to_html(),
            Node::Html(h) => h.value.clone(),
            Node::Strong(s) => s.to_html(),
            Node::Emphasis(e) => e.to_html(),
            Node::List(l) => l.to_html(),
            Node::ListItem(l) => l.to_html(),
            _ => format!(
                "<div style=\"background-color: red\">Not implemented<br>{:?}</div>",
                self
            ),
        }
    }

    fn get_text(&self) -> String {
        match self {
            Node::Root(root) => root.get_text(),
            Node::Heading(heading) => heading.get_text(),
            Node::Text(text) => text.get_text(),
            Node::Paragraph(p) => p.get_text(),
            Node::ThematicBreak(_) => panic!("<hr> cannot return its raw text"),
            Node::InlineCode(i) => i.get_text(),
            Node::Code(c) => c.get_text(),
            Node::Html(_) => panic!("Html cannot return its raw text"),
            Node::Strong(s) => s.get_text(),
            Node::Emphasis(e) => e.get_text(),
            Node::List(l) => l.get_text(),
            Node::ListItem(l) => l.get_text(),
            _ => String::from(""),
        }
    }
}
