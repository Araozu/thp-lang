use markdown::mdast::{Heading, Node};

use crate::{generator::Printable, utils};

pub trait SidebarGenerator {
    fn generate_sidebar(&self) -> String;
}

impl SidebarGenerator for Node {
    fn generate_sidebar(&self) -> String {
        match self {
            Node::Root(root) => {
                let children_nodes = root
                    .children
                    .clone()
                    .into_iter()
                    .filter_map(|x| match x {
                        Node::Heading(h) if h.depth <= 3 => Some(h),
                        _ => None,
                    })
                    .collect();

                // A top level topic that contains other topics
                let topic = extract_topics(&children_nodes, 0, 1);

                match topic {
                    Some((t, _)) => {
                        let html: String = t.children.iter().map(|x| x.get_html()).collect();
                        format!("<ul>{}</ul>", html)
                    }
                    None => String::from("D:"),
                }
            }
            _ => panic!("??"),
        }
    }
}

#[derive(Debug)]
struct Topic {
    text: String,
    children: Box<Vec<Topic>>,
}

impl Topic {
    pub fn get_html(&self) -> String {
        let extra = if self.children.len() > 0 {
            let children_html: String = self.children.iter().map(|x| x.get_html()).collect();

            format!("<ol class=\"px-4\">{}</ol>", children_html)
        } else {
            String::from("")
        };

        let html_fragment_link = utils::to_html_fragment(&self.text);
        format!(
            "<li class=\"m-2\"><a href=\"#{}\" class=\"inline-block w-full\">{}</a>{}</li>",
            html_fragment_link, self.text, extra
        )
    }
}

// Return the next heading and all its children
// current_level: the depth of the heading to match
fn extract_topics<'a>(
    headings: &'a Vec<Heading>,
    current_pos: usize,
    current_level: u8,
) -> Option<(Topic, usize)> {
    match headings.get(current_pos) {
        Some(h) if h.depth == current_level => {
            let mut new_vec = Vec::new();
            let mut next_pos = current_pos + 1;

            while let Some((topic, next)) = extract_topics(headings, next_pos, current_level + 1) {
                new_vec.push(topic);
                next_pos = next;
            }

            let title = h.get_text();
            let topic = Topic {
                text: title,
                children: Box::new(new_vec),
            };

            Some((topic, next_pos))
        }
        _ => None,
    }
}
