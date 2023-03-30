use markdown::mdast::Node;

mod heading;
mod root;
mod text;
mod paragraph;

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
            _ => format!("Not implemented<br>{:?}", self),
        }
    }

    fn get_text(&self) -> String {
        match self {
            Node::Root(root) => root.get_text(),
            Node::Heading(heading) => heading.get_text(),
            Node::Text(text) => text.get_text(),
            Node::Paragraph(p) => p.get_text(),
            Node::ThematicBreak(_) => panic!("<hr> cannot return its raw text"),
            _ => String::from(""),
        }
    }
}
