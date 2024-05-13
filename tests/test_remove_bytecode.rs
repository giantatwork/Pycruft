use pycruft::start;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};

    fn setup_test_cases(temp_dir: &TempDir) -> Vec<(String, bool)> {
        let test_cases = vec![
            ("dir1/dir2/file1.pyc", true),
            ("dir3/dir4/dir5/dir6/real.py", true),
            ("dir7/__pycache__/43234.pyc", false),
        ];

        for (path_str, _) in &test_cases {
            let path = temp_dir.path().join(path_str);
            let dir = path.parent().unwrap().to_path_buf();
            let file = path.file_name().unwrap().to_owned();
            fs::create_dir_all(&dir).unwrap();
            fs::File::create(&dir.join(file)).unwrap();
        }

        test_cases
            .into_iter()
            .map(|(path_str, exists)| {
                let path = temp_dir.path().join(path_str);
                (path.to_string_lossy().to_string(), exists)
            })
            .collect()
    }

    #[test]
    fn test_start() {
        let temp_dir: TempDir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        let test_cases = setup_test_cases(&temp_dir);

        start(dir_path, Some(false), Some(false));

        for (path_str, exists) in &test_cases {
            let path = dir_path.join(path_str);
            println!("{}", path_str);
            assert!(path.exists() == *exists);
        }

        assert!(!dir_path.join("dir7/__pycache__").exists());
        assert!(dir_path.join("dir1/dir2").exists());
        assert!(dir_path.join("dir7").exists());
    }
}
