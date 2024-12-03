use std::fs;

pub fn get_file_contents(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Expected file contents!");
    return contents;
}