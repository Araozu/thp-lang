use markdown::mdast::Node;

mod heading;
mod root;
mod text;

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
            _ => format!("Not implemented<br>{:?}", self),
        }
    }

    fn get_text(&self) -> String {
        match self {
            Node::Root(root) => root.get_text(),
            Node::Heading(heading) => heading.get_text(),
            Node::Text(text) => text.get_text(),
            _ => String::from(""),
        }
    }
}
