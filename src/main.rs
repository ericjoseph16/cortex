use std::error::Error;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    // rename("/Users/ericjoseph/Documents/rust-projects/cortex/poem.txt");

    // let args = Cli::parse();
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    
    let path = Path::new("/Users/ericjoseph/Documents/rust-projects/cortex/poem.txt");
    
    let result = add_file_to_docs(path);

    if let Err(e) = result {
        eprintln!("Oh no! A problem occurred: {}", e);
        std::process::exit(1);
    }
}

// move file to docs folder - UNUSED
// fn add_file_to_docs(source_path: &Path) -> Result<(), Box<dyn Error>> {
//     let doc_path = PathBuf::from("docs");
//     fs::create_dir_all(&doc_path)?;
    
//     if let Some(filename) = source_path.file_name() {

//         let mut dest_path = doc_path;
//         dest_path.push(filename);

//         fs::rename(source_path, dest_path)?;

//         println!("Successfully added file: {:?}", filename);

//     } else {
//         eprintln!("Error: Could not get filename from {:?}", source_path);
//     }
//     Ok(())
// }