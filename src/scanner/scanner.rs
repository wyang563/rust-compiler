use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn scan(file_path: &Path) -> String {
    let mut file = File::open(file_path).expect("Failed to Open File");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Failed to read string from file");

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
                let _scan_result = scan(input_path);

                // extract expected result from output file
                let mut out_file = File::open(output_path).expect("Failed to Open File");
                let mut out_file_contents = String::new();
                out_file.read_to_string(&mut out_file_contents).expect("Failed to read string from file");

                if output_path_str.contains("invalid") {
                    assert_eq!(_scan_result, "invalid");
                } else {
                    assert_eq!(out_file_contents, _scan_result);
                }
            }
        }
    }
}
