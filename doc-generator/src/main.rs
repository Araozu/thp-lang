use clap::Parser;
use markdown::to_html;
use std::fs::File;
use std::io::Write;
use std::{
    fs,
    path::Path,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input folder
    #[arg(short, long)]
    input: String,
    /// Output folder
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let input_folder = Path::new(&args.input);
    let output_folder = Path::new(&args.output);

    if input_folder.is_dir() && output_folder.is_dir() {
        process_folder(&input_folder, input_folder, output_folder);
    } else {
        eprint!("Input folder is not a valid path to a folder")
    }
}

fn process_folder(path: &Path, input_folder: &Path, output_folder: &Path) {
    for entry in path.read_dir().unwrap() {
        match entry {
            Ok(entry) if entry.path().is_dir() => {
                let path = entry.path();

                match ensure_folder_exists(&entry.path(), input_folder, output_folder) {
                    Ok(_) => {
                        process_folder(&path, input_folder, output_folder);
                    }
                    Err(reason) => panic!("{}", reason),
                }
            }
            Ok(entry) if entry.path().is_file() => {
                let _ = process_markdown(&entry.path(), input_folder, output_folder);
            }
            _ => panic!(),
        }
    }
}

fn ensure_folder_exists(folder: &Path, input_folder: &Path, output_folder: &Path) -> Result<(), String> {
    // /home/fernando/misti/docs/markdown
    let input_folder = input_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/static
    let output_folder = output_folder.canonicalize().unwrap();

    // /home/fernando/misti/docs/markdown/en/
    let full_input_folder = folder.canonicalize().unwrap();

    let relative_new_folder = full_input_folder.strip_prefix(input_folder).unwrap();

    let mut full_output_folder = output_folder.clone();
    full_output_folder.push(relative_new_folder);

    println!("Ensuring that folder exists:\n{:?}", full_output_folder);

    // If this is a "top-level" folder, remove all its contents, if it exists
    if full_output_folder.is_dir() {
        println!("| Removing...");
        let _ = fs::remove_dir_all(&full_output_folder);
    }

    // Create folder
    match fs::create_dir(&full_output_folder) {
        Ok(_) => {
            println!("| done\n\n");
            Ok(())
        }
        Err(_) => Err(format!("Error creating folder {:?}", full_output_folder)),
    }
}

fn process_markdown(file: &Path, input_folder: &Path, output_folder: &Path) -> Result<(), String> {
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


    let file_content_bytes = fs::read(&input_file).unwrap();
    let markdown_text = String::from_utf8(file_content_bytes).unwrap();

    let html_text = to_html(markdown_text.as_str());

    // Write the HTML to disk

    println!("Compiling: from -> to\n{:?}\n{:?}\n", input_file, output_file);

    let _ = File::create(&output_file)
        .unwrap()
        .write_all(html_text.as_bytes())
        .unwrap();

    Ok(())
}
