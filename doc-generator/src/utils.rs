use std::{fs, path::Path};

use markdown::mdast::Node;

use crate::generator::Printable;

pub fn to_html_fragment(text: &String) -> String {
    text.clone().replace(" ", "-")
}

pub fn collect_children_html(vec: &Vec<Node>) -> String {
    let mut result = Vec::<String>::new();

    for node in vec {
        result.push(node.to_html())
    }

    result.into_iter().collect()
}

pub fn collect_children_text(vec: &Vec<Node>) -> String {
    let mut result = Vec::<String>::new();

    for node in vec {
        result.push(node.get_text())
    }

    result.join("-")
}

pub fn ensure_folder_exists(
    folder: &Path,
    input_folder: &Path,
    output_folder: &Path,
) -> Result<(), String> {
    // /home/fernando/misti/docs/markdown
    let input_folder = input_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/static
    let output_folder = output_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/markdown/en/
    let full_input_folder = folder.canonicalize().unwrap();

    let relative_new_folder = full_input_folder.strip_prefix(input_folder).unwrap();

    let mut full_output_folder = output_folder.clone();
    full_output_folder.push(relative_new_folder);

    // println!("Ensuring that folder exists:\n{:?}", full_output_folder);

    // If this is a "top-level" folder, remove all its contents, if it exists
    if full_output_folder.is_dir() {
        // println!("| Removing...");
        let _ = fs::remove_dir_all(&full_output_folder);
    }

    // Create folder
    match fs::create_dir(&full_output_folder) {
        Ok(_) => {
            // println!("| done\n\n");
            Ok(())
        }
        Err(_) => Err(format!("Error creating folder {:?}", full_output_folder)),
    }
}
