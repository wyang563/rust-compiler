use std::path::Path;
use std::fs::File;
use std::io::Read;

fn scan_line<'a>(line_num: usize, line: &'a str) {
    
}

pub fn scan(file_path: &Path) -> String {
    let mut file = File::open(file_path).expect("Failed to Open File");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Failed to read string from file");
    let mut scanner_results: Vec<&str> = Vec::new();
    return file_str;
}
