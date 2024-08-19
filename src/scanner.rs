use std::env;
use std::fs;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, Read};


fn scanner(input: str) -> str {
    "";
}

// scanner tests
#[cfg(test)]
use super::*;

#[test]
mod tests {
    use super::*;
    let const directory_path = "./private-tests-main";
    for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)?;
            println!("{}", file_contents);
        }
    }
}
