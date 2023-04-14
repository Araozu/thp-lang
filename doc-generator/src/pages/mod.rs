use std::path::Path;

use yaml_rust::Yaml;

use crate::utils;

mod md_compiler;

pub enum Node<'a> {
    File(File<'a>),
    Folder(Folder<'a>),
}

pub struct File<'a> {
    /// Name of the file
    path: &'a String,
}

pub struct Folder<'a> {
    /// Name of the folder
    path: &'a String,
    /// Display name of the folder
    name: &'a String,
    /// If true, then there MUST be a `File {path: "index"}` in the `children` field
    has_index: bool,
    /// Sub files or folders
    children: Box<Vec<Node<'a>>>,
}

/// Creates a `YAML::String` from a `&str`
macro_rules! y_str {
    ($str:literal) => {
        &Yaml::String(String::from($str))
    };
}

pub fn parse_yaml(values: &Yaml) -> Node {
    let Yaml::Hash(table) = values
    else {panic!("YAML: input MUST be an object")};

    // Node path
    let Yaml::String(path) = table.get(y_str!("path")).expect("YAML: Node MUST have a `path` key")
    else { panic!("YAML: `path` MUST be a String") };

    let input_data = (
        table.get(y_str!("name")),
        table.get(y_str!("has_index")),
        table.get(y_str!("children")),
    );

    match input_data {
        (None, None, None) => Node::File(File { path }),
        (Some(name), has_index, Some(children)) => {
            let Yaml::String(name) = name
            else { panic!("YAML: `name` MUST be a String") };

            let has_index = match has_index {
                Some(Yaml::Boolean(v)) => *v,
                Some(_) => panic!("YAML: if key `has_index` exists, it MUST be a Boolean"),
                None => false,
            };

            let Yaml::Array(children) = children
            else {panic!("YAML: `children` MUST be an Array")};

            let children_nodes: Vec<Node> = children
                .into_iter()
                .map(|values| parse_yaml(values))
                .collect();

            Node::Folder(Folder {
                path,
                name,
                has_index,
                children: Box::new(children_nodes),
            })
        }
        _ => {
            panic!("YAML: A Node is missing a `name` or `children` key")
        }
    }
}

pub fn generate_pages_html(file_tree: &Node, current_path: &Path) -> String {
    match file_tree {
        Node::File(file) => {
            if file.path == "index" {
                format!(
                    "<li class=\"my-2\">
                        <a class=\"inline-block w-full hover:text-c2-primary\" href=\"/{}\">Index</a>
                    </li>",
                    current_path.to_str().unwrap()
                )
            } else if file.path == "" {
                String::from("")
            } else {
                format!(
                    "<li class=\"my-2\">
                        <a class=\"inline-block w-full hover:text-c2-primary\" href=\"/{}/{}.html\">{}</a>
                    </li>",
                    current_path.to_str().unwrap(),
                    file.path,
                    file.path
                )
            }
        }
        Node::Folder(folder) => {
            let mut new_path = current_path.to_path_buf();
            new_path.push(folder.path);

            let sub_nodes_html: Vec<String> = folder
                .children
                .iter()
                .map(|n| generate_pages_html(n, &new_path))
                .collect();

            // This is true for the root of the YAML file
            if folder.path == "" {
                format!("<ul>{}</ul>", sub_nodes_html.join(""))
            } else {
                format!(
                    "<li class=\"my-2\">
                    <div class=\"uppercase opacity-80 mt-6 font-semibold\">{}</div>
                    <ul>{}</ul>
                </li>",
                    folder.name,
                    sub_nodes_html.join("")
                )
            }
        }
    }
}

pub fn compile_md_to_html(
    file_tree: &Node,
    current_path: &Path,
    input_folder: &Path,
    output_folder: &Path,
    file_tree_html: &String,
) {
    match file_tree {
        Node::File(file) if file.path != "" => {
            let mut file_path = current_path.canonicalize().unwrap();
            file_path.push(format!("{}.md", file.path));

            md_compiler::compile(&file_path, input_folder, output_folder, file_tree_html);
        }
        Node::File(_) => {
            panic!("YAML: A file cannot have an empty `path` key")
        }
        Node::Folder(folder) if folder.path != "" => {
            let mut new_path = current_path.canonicalize().unwrap();
            new_path.push(folder.path);
            utils::ensure_folder_exists(&new_path, input_folder, output_folder)
                .expect("SHOULD be able to create folder");

            for node in folder.children.iter() {
                compile_md_to_html(node, &new_path, input_folder, output_folder, file_tree_html);
            }
        }
        Node::Folder(folder) => {
            for node in folder.children.iter() {
                compile_md_to_html(
                    node,
                    &current_path,
                    input_folder,
                    output_folder,
                    file_tree_html,
                );
            }
        }
    }
}
