use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::constants::ScannerState;
use super::constants::is_reserved_literal;
use super::constants::is_whitespace;

fn scan_line<'a>(line_num: usize, line: &'a str, state: &mut ScannerState) -> Vec<&'a str> {
    let mut scan_line_result: Vec<&str> = Vec::new();
    let mut scan_fragment = String::new();
    for char in line.chars() {
        if state.in_multi_line_comment {
            if scan_fragment == "*/" {
                state.in_multi_line_comment = false;
                scan_fragment = String::new();
            }
        } else {
            // check if we're in a comment
            if scan_fragment == "//" {
                break;
            } else if scan_fragment == "/*" {
                state.in_multi_line_comment = true;
                scan_fragment = String::new();
            }

            // process string fragment if we see whitespace
        }
    }
    
    return scan_line_result;
}

pub fn scan(file_path: &Path) -> String {
    let mut file = File::open(file_path).expect("Failed to Open File");
    let mut file_str = String::new();
    let mut state = ScannerState {
        in_multi_line_comment: false,
        in_string: false,
        in_char: false,
    };
    file.read_to_string(&mut file_str).expect("Failed to read string from file");
    let mut scanner_results: Vec<&str> = Vec::new();
    for (line_num, line) in file_str.split("\n").enumerate() {
        scanner_results.extend(scan_line(line_num, line, &mut state));
    }
    return file_str;
}

// scanner tests
#[cfg(test)]
mod tests {
    use super::*;
    use walkdir::WalkDir;

    #[test]
    fn test_scanner() {
        let test_input_dir = String::from("./src/private-tests-main/scanner/input");
        for entry in WalkDir::new(test_input_dir).into_iter().filter_map(|e| e.ok()) {
            let input_path = entry.path();
            let mut output_path_str = input_path.to_str().unwrap().to_string();
            output_path_str = output_path_str.replace("input", "output");
            let output_path = Path::new(&output_path_str).with_extension("out");
            if output_path.is_file() {
                let scan_result = scan(input_path);

                // extract expected result from output file
                let mut out_file = File::open(output_path).expect("Failed to Open File");
                let mut out_file_contents = String::new();
                out_file.read_to_string(&mut out_file_contents).expect("Failed to read string from file");

                if output_path_str.contains("invalid") {
                    assert_eq!(scan_result, "invalid");
                } else {
                    assert_eq!(out_file_contents, scan_result);
                }
            }
        }
    }
}
