use std::path::{Path, PathBuf};
use std::{fs, io, process};

static BYTECODE_DIR: &str = "__pycache__";

fn find_bytecode_dirs(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut results: Vec<PathBuf> = Vec::new();

    if let Ok(paths) = fs::read_dir(dir) {
        for path in paths.filter_map(Result::ok) {
            let path_path = path.path();
            if path_path.is_dir() {
                if let Some(file_name) = path_path.file_name() {
                    if file_name == BYTECODE_DIR {
                        results.push(path_path.clone())
                    } else {
                        results.extend(find_bytecode_dirs(&path_path).unwrap());
                    }
                }
            }
        }
    }

    Ok(results)
}

fn remove_bytecode_dirs(results: &Vec<PathBuf>, verbose: bool) -> Result<u32, std::io::Error> {
    let mut dirs_removed: u32 = 0;

    for path in results {
        if let Err(error) = fs::remove_dir_all(&path) {
            println!(
                "Unable to remove: {}, error: {}",
                path.to_string_lossy(),
                error
            );
            return Err(error);
        } else if verbose {
            println!("Removed directory: {}", path.to_string_lossy());
        }
        dirs_removed += 1;
    }
    Ok(dirs_removed)
}

pub fn start(dir: &Path, skip_confirmation: Option<bool>, verbose: Option<bool>) {
    let skip_confirmation = skip_confirmation.unwrap_or(false);
    let verbose = verbose.unwrap_or(false);

    let results = match find_bytecode_dirs(dir) {
        Ok(results) => results,
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };

    if results.len() == 0 {
        println!("No '{}' directories found", BYTECODE_DIR);
        process::exit(0);
    }

    if skip_confirmation {
        match remove_bytecode_dirs(&results, verbose) {
            Ok(dirs_removed) => {
                println!("Removed {} directories", dirs_removed);
                process::exit(0);
            }
            Err(error) => {
                println!("Error: {}", error);
                process::exit(1);
            }
        }
    }

    if verbose {
        for path in &results {
            println!("{}", path.to_string_lossy());
        }
    }

    println!(
        "\nAre you sure you want to remove {} '{}' directories at '{}'?\n(yes/no)",
        results.len(),
        BYTECODE_DIR,
        dir.to_string_lossy()
    );

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_lowercase();

    if input == "yes" {
        match remove_bytecode_dirs(&results, verbose) {
            Ok(dirs_removed) => {
                println!("Removed {} directories", dirs_removed);
                process::exit(0);
            }
            Err(error) => {
                println!("Error: {}", error);
                process::exit(1);
            }
        }
    } else if input == "no" {
        println!("Cancelled");
    } else {
        println!("Invalid input: please type 'yes' or 'no'");
        process::exit(1);
    }
}
