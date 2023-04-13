use super::generator::Printable;
use crate::sidebar::SidebarGenerator;
use std::io::Write;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use toml::{Table, Value};

enum EntryFound {
    TomlFile,
    OtherFile,
    None,
}

// Traverses the current path searching for a TOML file
pub fn search_config_file(current_path: &Path, input_folder: &Path, output_folder: &Path) {
    // Iterate over all the files searching for a TOML file
    let result = current_path
        .read_dir()
        .unwrap()
        .fold(&EntryFound::None, |acc, next| {
            let p = next.unwrap().path();
            let is_file = p.is_file();
            let ext = p.extension();

            match (acc, is_file, ext) {
                (EntryFound::TomlFile, true, Some(x)) if x == "toml" => {
                    panic!("FOUND A SECOND TOML FILE!!!")
                }
                (EntryFound::TomlFile, _, _) => acc,
                (EntryFound::OtherFile, true, Some(x)) if x == "toml" => &EntryFound::TomlFile,
                (EntryFound::None, true, Some(x)) if x == "toml" => &EntryFound::TomlFile,
                (EntryFound::None, true, Some(_)) => &EntryFound::OtherFile,
                _ => acc,
            }
        });

    match result {
        // If a file other than a TOML file is found, panic
        EntryFound::OtherFile => panic!(
            "Found an orphan file without a TOML parent at {:?}",
            current_path
        ),
        // Process the TOML file
        EntryFound::TomlFile => process_toml(current_path, input_folder, output_folder),
        // No files found, recursively read children folders
        EntryFound::None => {
            for entry in current_path.read_dir().unwrap() {
                // Should always succeed, and countain a folder
                let x = entry.unwrap();
                let path = x.path();

                ensure_folder_exists(&path, input_folder, output_folder).unwrap();
                search_config_file(&path, input_folder, output_folder);
            }
        }
    };
}

fn process_toml(current_path: &Path, input_folder: &Path, output_folder: &Path) {
    let mut toml_file_path = current_path.canonicalize().unwrap();
    toml_file_path.push("index.toml");

    // Read TOML file
    let toml_bytes = fs::read(toml_file_path.clone())
        .expect(format!("index.toml MUST exist ({:?})", toml_file_path).as_str());
    let toml_file = String::from_utf8(toml_bytes).expect("index.toml MUST be UTF-8");

    // Parse TOML file
    let toml_table = toml_file
        .parse::<Table>()
        .expect("index.toml MUST contain valid TOML");

    // Process MD files indicated in TOML file
    //  Expect a key named entry-point and compile it
    let Value::String(entry_point) = toml_table.get("entry-point").expect("TOML: key entry-point MUST exist")
    else {panic!("TOML: entry-point MUST be a String")};

    let mut file = current_path.canonicalize().unwrap();
    file.push(format!("{}.md", entry_point));

    compile_md_file(&file, input_folder, output_folder)
        .expect("FS: entry-point file MUST point to a valid file");

    //  Subsequent keys should have schema:
    //   [key]
    //   section-name = "Section name"
    //   children = ["file1", "file2", "file3"]
    for (key, value) in toml_table.into_iter() {
        if key == "entry-point" {
            continue;
        }

        match value {
            Value::Table(t) => {
                let Value::String(_section_name) = t.get("section-name").expect(format!("TOML: table {} MUST have a key section-name", key).as_str())
                else {panic!("TOML: key section-name of table {} MUST be a String", key)};

                let Value::Array(children) = t.get("children").expect(format!("TOML: table {} MUST have a key children", key).as_str())
                else {panic!("TOML: in table {} > children MUST be an Array", key)};

                // Ensure folder exists
                let mut folder_path = current_path.canonicalize().unwrap();
                folder_path.push(key.clone());
                ensure_folder_exists(&folder_path, input_folder, output_folder).unwrap();

                for file_name in children {
                    let Value::String(file_name) = file_name
                    else {panic!("TOML: in table {} > children's value MUST be Strings (found {:?})", key, file_name)};

                    let mut file = current_path.canonicalize().unwrap();
                    file.push(key.clone());
                    file.push(format!("{}.md", file_name));

                    compile_md_file(&file, input_folder, output_folder)
                        .expect(format!("Error compiling file {}", file.display()).as_str());
                }
            }
            _ => panic!("TOML: key {} MUST be a table", key),
        }
    }
}

fn compile_md_file(
    file: &PathBuf,
    input_folder: &Path,
    output_folder: &Path,
) -> Result<(), String> {
    // /home/fernando/misti/docs/markdown
    let input_folder = input_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/markdown/en/docs/latest/index.md
    let input_file = file.canonicalize().unwrap();

    // /home/fernando/misti/docs/static
    let output_folder = output_folder.canonicalize().unwrap();

    // en/docs/latests/index.md
    let relative_input_file = input_file.strip_prefix(input_folder).unwrap();

    let mut output_file = output_folder.clone();
    output_file.push(relative_input_file);
    output_file.set_extension("html");

    //
    //  Compilation
    //
    let file_content_bytes = fs::read(&input_file).unwrap();
    let markdown_text = String::from_utf8(file_content_bytes).unwrap();

    // let html_text = to_html(markdown_text.as_str());
    let md_ast = markdown::to_mdast(&markdown_text, &markdown::ParseOptions::gfm()).unwrap();
    let html_text = md_ast.to_html();
    let sidebar_html = md_ast.generate_sidebar();

    // Read template.html
    let mut template_path = output_folder.clone();
    template_path.push("template.html");

    let template_contents = fs::read(template_path).unwrap();
    let template_contents = String::from_utf8(template_contents).unwrap();

    let final_output = template_contents
        .replace("{{markdown}}", &html_text)
        .replace("{{sidebar}}", &sidebar_html);

    //
    // Write to disk
    //
    let _ = File::create(&output_file)
        .expect(format!("MD: Output file should be valid {:?}", &output_file).as_str())
        .write_all(final_output.as_bytes())
        .unwrap();

    Ok(())
}

fn ensure_folder_exists(
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
