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

/*
List {
    children: [
    ListItem {
        children: [
            Paragraph {
                children: [
                    InlineCode {
                    value: "\\\"", position: Some(88:3-88:7 (1728-1732)) }
                ],
                position: Some(88:3-88:7 (1728-1732)) }
        ],
        position: Some(88:1-89:1 (1726-1733)), spread: false, checked: None }
    , ListItem {
        children: [
            Paragraph {
                children: [
                    InlineCode {
                        value: "\\\\", position: Some(90:3-90:7 (1736-1740)) }
                ],
                position: Some(90:3-90:7 (1736-1740)) }
        ],
        position: Some(90:1-90:7 (1734-1740)), spread: false, checked: None }
    , ListItem {
        children: [
            Paragraph {
            children: [
                InlineCode {
                    value: "\\n", position: Some(91:3-91:7 (1743-1747)) }
            ],
            position: Some(91:3-91:7 (1743-1747)) }
        ],
        position: Some(91:1-91:7 (1741-1747)), spread: false, checked: None }
    , ListItem {
    children: [
    Paragraph {
    children: [
    InlineCode {
    value: "\\r", position: Some(92:3-92:7 (1750-1754)) }
    ],
     position: Some(92:3-92:7 (1750-1754)) }
    ],
     position: Some(92:1-92:7 (1748-1754)), spread: false, checked: None }
    , ListItem {
    children: [
    Paragraph {
    children: [
    InlineCode {
    value: "\\t", position: Some(93:3-93:7 (1757-1761)) }
    ],
     position: Some(93:3-93:7 (1757-1761)) }
    ],
     position: Some(93:1-93:7 (1755-1761)), spread: false, checked: None }
    , ListItem {
    children: [
    Paragraph {
    children: [
    InlineCode {
    value: "\\b", position: Some(94:3-94:7 (1764-1768)) }
    ],
     position: Some(94:3-94:7 (1764-1768)) }
    ],
     position: Some(94:1-95:1 (1762-1769)), spread: false, checked: None }
    ],
     position: Some(88:1-95:1 (1726-1769)), ordered: false, start: None, spread: true }
*/
