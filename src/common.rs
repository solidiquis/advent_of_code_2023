use std::{env, fs, io, path::PathBuf};

pub fn load_test_case_to_string(file_name: &str) -> io::Result<String> {
    let current_dir = env::current_dir()?;
    let mut test_case_path = PathBuf::new();
    test_case_path.push(current_dir);
    test_case_path.push("test_data");
    test_case_path.push(file_name);
    fs::read_to_string(test_case_path)
}
