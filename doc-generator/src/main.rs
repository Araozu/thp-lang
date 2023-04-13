use clap::Parser;
use std::path::Path;

mod generator;
mod processor;
mod sidebar;
mod utils;

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
        processor::search_config_file(&input_folder, input_folder, output_folder);
        // process_folder(&input_folder, input_folder, output_folder);
    } else {
        eprint!("Input folder is not a valid path to a folder")
    }
}
