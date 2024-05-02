use std::path::{Path, PathBuf};
use std::{fs, process};

static BYTECODE_EXTENSIONS: [&str; 2] = ["pyc", "pyo"];
static BYTECODE_DIRS: [&str; 1] = ["__pycache__"];

fn walk_directory(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut results: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            results.extend(walk_directory(&path)?);
            let contains_bytecode = BYTECODE_DIRS.iter().any(|&bytecode_dir| {
                path.file_name()
                    .map_or(false, |file_name| file_name == bytecode_dir)
            });
            if contains_bytecode {
                results.push(path)
            }
        } else if path.is_file() {
            let contains_bytecode = BYTECODE_EXTENSIONS.iter().any(|&bytecode_ext| {
                path.extension()
                    .map_or(false, |file_extension| file_extension == bytecode_ext)
            });
            if contains_bytecode {
                results.push(path);
            }
        }
    }
    results.sort_by(|a, b| b.cmp(a));

    Ok(results)
}

pub fn remove_bytecode(dir: &Path, verbose: bool) {
    let results = match walk_directory(dir) {
        Ok(results) => results,
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };

    let mut files_removed: u32 = 0;
    let mut dirs_removed: u32 = 0;

    for path in results {
        let path_str = path.to_string_lossy();
        if path.is_file() {
            if let Err(error) = fs::remove_file(&path) {
                println!("Error: {}", error);
            } else {
                if verbose {
                    println!("Removed file: {}", path_str);
                    files_removed = files_removed + 1;
                }
            }
        } else if path.is_dir() {
            if let Err(error) = fs::remove_dir(&path) {
                println!("Unable to remove: {}, error: {}", path_str, error);
            } else {
                if verbose {
                    println!("Removed directory: {}", path_str);
                    dirs_removed = dirs_removed + 1;
                }
            }
        }
    }
    if verbose {
        println!(
            "Removed {} files and {} directories",
            files_removed, dirs_removed
        );
    }
}
