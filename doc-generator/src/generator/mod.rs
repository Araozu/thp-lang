use markdown::mdast::Node;

mod root;
mod heading;

pub trait Printable {
    fn to_html(&self) -> String;
}

impl Printable for Node {
    fn to_html(&self) -> String {
        match self {
            Node::Root(root) => root.to_html(),
            Node::Heading(heading) => heading.to_html(),
            _ => format!("Not implemented<br>{:?}", self),
        }
    }
}

