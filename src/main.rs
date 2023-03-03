use clap::{App, Arg};
use std::fs;
use std::io::{self, Read};
use std::os::unix::prelude::PermissionsExt;
use walkdir::WalkDir;

struct FileInfo {
    path: String,
    mode: u32,
    bytes: Option<Vec<u8>>,
}

fn main() -> io::Result<()> {
    let matches = App::new("File Reader")
        .arg(
            Arg::with_name("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path to read files from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("print_files")
                .long("print-files")
                .help("Prints the files out")
                .takes_value(false),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");

    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;
        
        if entry.file_type().is_symlink(){
            println!("Skipping symlink: {}", entry.path().display());
            continue;
        }
        
        if !entry.file_type().is_file(){
            continue;
        }

        let path = entry.path().to_string_lossy().into_owned();
        let metadata = entry.metadata()?;
        let mode = metadata.permissions().mode();

        
        let mut file = fs::File::open(&entry.path())?;
        let mut contents = Vec::new();
        let _ = file.read_to_end(&mut contents);
        let info = FileInfo {
            path,
            mode,
            bytes: Some(contents),
        };
        files.push(info);
    }

    if matches.is_present("print_files") {
        for file in &files {
            println!("File: {} with mode {}", file.path, file.mode);
        }
    }
    
    // Print out the number of files found
    println!("Found {} files", files.len());

    // Print out the total bytes loaded
    let total_bytes: usize = files.iter().map(|f| 
        match &f.bytes {
            Some(b) => b.len(),
            None => 0,
        }
    ).sum();
    
    println!("Total bytes: {}", total_bytes);

    Ok(())
}
