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
                    }
                    results.extend(find_bytecode_dirs(&path_path).unwrap());
                }
            }
        }
    }

    Ok(results)
}

fn remove_bytecode_dirs(results: &Vec<PathBuf>, verbose: bool) -> Result<(), std::io::Error> {
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
        if verbose {
            println!("Removed {} directories", dirs_removed);
        }
    }
    Ok(())
}

pub fn start(dir: &Path, confirm: Option<bool>, verbose: Option<bool>) {
    let confirm = confirm.unwrap_or(false);
    let verbose = verbose.unwrap_or(false);

    let results = match find_bytecode_dirs(dir) {
        Ok(results) => results,
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };

    if confirm {
        println!("Do you want to remove the following directories? (yes/no)");

        for path in &results {
            println!("{}", path.to_string_lossy());
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "yes" {
            remove_bytecode_dirs(&results, verbose).unwrap();
        } else if input == "no" {
            println!("Cancelled");
            process::exit(0);
        } else {
            println!("Invalid input. Please type 'yes' or 'no'.");
            process::exit(1);
        }
    } else {
        remove_bytecode_dirs(&results, verbose).unwrap();
    }
}
