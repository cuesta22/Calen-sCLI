use std::fs;
use std::io::{self, BufRead};
// use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Calen's CLI", about = "A basic Rust CLI utility tool.")]
enum CLI {
    /// Echo the input string to the terminal
    Echo {
        #[structopt(help = "The string to echo")]
        text: String,
    },
    /// Concatenate and print file content
    Cat {
        #[structopt(help = "The file to display")]
        file: String,
    },
    /// List files and directories in the specified path
    Ls {
        #[structopt(help = "The directory to list (default is current directory)", default_value = ".")]
        path: String,
    },
    /// Find files in a directory by name
    Find {
        #[structopt(help = "The directory to search in")]
        path: String,
        #[structopt(help = "The filename to search for")]
        filename: String,
    },
    /// Search for a pattern in a file
    MiniGrep {
        #[structopt(help = "The pattern to search for")]
        pattern: String,
        #[structopt(help = "The file to search in")]
        file: String,
    },
}

fn main() {
    let args = CLI::from_args();

    match args {
        CLI::Echo { text } => {
            println!("{}", text);
        }
        CLI::Cat { file } => {
            match fs::read_to_string(&file) {
                Ok(content) => println!("{}", content),
                Err(err) => eprintln!("Error reading file: {}", err),
            }
        }
        CLI::Ls { path } => {
            match fs::read_dir(&path) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(e) => println!("{}", e.file_name().to_string_lossy()),
                            Err(err) => eprintln!("Error reading entry: {}", err),
                        }
                    }
                }
                Err(err) => eprintln!("Error reading directory: {}", err),
            }
        }
        CLI::Find { path, filename } => {
            let mut found = false;
            if let Err(err) = find_files(&path, &filename, &mut found) {
                eprintln!("Error: {}", err);
            }
            if !found {
                println!("No file found matching '{}'", filename);
            }
        }
        CLI::MiniGrep { pattern, file } => {
            match fs::File::open(&file) {
                Ok(file) => {
                    let reader = io::BufReader::new(file);
                    for (line_number, line) in reader.lines().enumerate() {
                        match line {
                            Ok(content) => {
                                if content.contains(&pattern) {
                                    println!("{}: {}", line_number + 1, content);
                                }
                            }
                            Err(err) => eprintln!("Error reading line: {}", err),
                        }
                    }
                }
                Err(err) => eprintln!("Error opening file: {}", err),
            }
        }
    }
}

fn find_files(path: &str, filename: &str, found: &mut bool) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_files(&path.to_string_lossy(), filename, found)?;
        } else if let Some(name) = path.file_name() {
            if name.to_string_lossy() == filename {
                println!("{}", path.to_string_lossy());
                *found = true;
            }
        }
    }
    Ok(())
}