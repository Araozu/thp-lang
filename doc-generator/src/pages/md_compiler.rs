use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{generator::Printable, sidebar::SidebarGenerator};

/// ## Parameters
///
/// - `file`: Path to the MD file to compile
/// - `input_folder`: Path to the input folder passed as parameter of the program
/// - `output_folder`: Path to the output folder passed as parameter of the program
/// - `file_tree_html`: HTML code of the file tree to be inserted into the generated HTML
pub fn compile(file: &PathBuf, input_folder: &Path, output_folder: &Path, file_tree_html: &String) {
    // /home/fernando/misti/docs/markdown
    let input_folder = input_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/markdown/en/docs/latest/index.md
    let input_file = file
        .canonicalize()
        .expect(format!("Expected file {:?} to exist", file).as_str());

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

    // Insert the markdown, sidebar and file tree into the template
    let final_output = template_contents
        .replace("{{markdown}}", &html_text)
        .replace("{{sidebar}}", &sidebar_html)
        .replace("{{pages}}", &file_tree_html);

    //
    // Write to disk
    //
    File::create(&output_file)
        .expect(format!("MD: Output file should be valid {:?}", &output_file).as_str())
        .write_all(final_output.as_bytes())
        .unwrap();
}
