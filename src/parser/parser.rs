use super::AST;
use std::path::Path;
use super::super::scanner::scanner::scan_file;

pub fn parse_file(file_path: &Path) -> Result<AST::Program, String> {
    // Lex file first
    match scan_file(file_path) {
        Ok(tokens) => (), 
        Err(e) => return Err(e)
    }
}

pub fn parse(file_path: &Path, mut writer: Box<dyn std::io::Write>) {
    match parse_file(file_path) {
        Ok(_) => {
            writeln!(writer, "Parsed file: {:?}", file_path.display()).unwrap();
        },
        Err(e) => {
            writeln!(writer, "Error parsing file: \n {:?}", e).unwrap();
        }
    }
}   

