use crate::pages::{compile_md_to_html, generate_pages_html, parse_yaml};
use crate::utils;
use std::{
    fs,
    path::Path,
};
use yaml_rust::YamlLoader;

enum EntryFound {
    YamlFile,
    OtherFile,
    None,
}

// Traverses the current path searching for a YAML file
pub fn search_config_file(current_path: &Path, input_folder: &Path, output_folder: &Path) {
    // Iterate over all the files searching for a YAML file
    let result = current_path
        .read_dir()
        .unwrap()
        .fold(&EntryFound::None, |acc, next| {
            let p = next.unwrap().path();
            let is_file = p.is_file();
            let ext = p.extension();

            match (acc, is_file, ext) {
                (EntryFound::YamlFile, true, Some(x)) if x == "yaml" => {
                    panic!("FOUND A SECOND YAML FILE!!!")
                }
                (EntryFound::YamlFile, _, _) => acc,
                (EntryFound::OtherFile, true, Some(x)) if x == "yaml" => &EntryFound::YamlFile,
                (EntryFound::None, true, Some(x)) if x == "yaml" => &EntryFound::YamlFile,
                (EntryFound::None, true, Some(_)) => &EntryFound::OtherFile,
                _ => acc,
            }
        });

    match result {
        // If a file other than a YAML file is found, panic
        EntryFound::OtherFile => panic!(
            "Found an orphan file without a YAML parent at {:?}",
            current_path
        ),
        // Process the YAML file
        EntryFound::YamlFile => process_yaml(current_path, input_folder, output_folder),
        // No files found, recursively read children folders
        EntryFound::None => {
            for entry in current_path.read_dir().unwrap() {
                // Should always succeed, and countain a folder
                let x = entry.unwrap();
                let path = x.path();

                utils::ensure_folder_exists(&path, input_folder, output_folder).unwrap();
                search_config_file(&path, input_folder, output_folder);
            }
        }
    };
}

fn process_yaml(current_path: &Path, input_folder: &Path, output_folder: &Path) {
    //
    // Read YAML file
    //
    let mut yaml_path = current_path.canonicalize().unwrap();
    yaml_path.push("index.yaml");

    let yaml_bytes = fs::read(yaml_path).expect("File index.yaml MUST exist");
    let yaml = String::from_utf8(yaml_bytes).expect("YAML index file MUST be valid UTF-8");

    let yaml_docs =
        YamlLoader::load_from_str(yaml.as_str()).expect("YAML file MUST contain valid YAML");
    let yaml = &yaml_docs[0];

    //
    // Parse YAML
    //
    let file_tree = parse_yaml(&yaml);

    //
    // Generate File Tree HTML
    //
    let tree_html = {
        let input_folder = input_folder.canonicalize().unwrap();
        let yaml_folder_temp = current_path.canonicalize().unwrap();
        let web_absolute_path = yaml_folder_temp.strip_prefix(input_folder).unwrap();

        generate_pages_html(&file_tree, web_absolute_path)
    };

    //
    // Compile MD to HTML
    //
    compile_md_to_html(
        &file_tree,
        current_path,
        input_folder,
        output_folder,
        &tree_html,
    );
}
