use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::constants::is_whitespace;

struct ScannerState {
    in_comment: bool,
    in_multiline_comment: bool,
    in_string: bool,
    in_char: bool,
    line_num: u32,
}

/*
increment cur_token, and return true if we finish cur_token ie the next space is whitespace
*/
fn incr_cur_token(cur_token: &mut String, next_char: char) -> bool {
    if is_whitespace(next_char) {
        return true;
    }
    cur_token.push(next_char);
    return false;
}

fn scan_program(file_str: String) {
    // init scanner state
    let mut scanner_state = ScannerState {
        in_comment: false,
        in_multiline_comment: false,
        in_string: false,
        in_char: false,
        line_num: 1,
    };
    let mut cur_token: String = String::new();
    let mut end_token  = false;
    for idx in 0..file_str.len() {
        let next_char = file_str.chars().nth(idx).unwrap();
        // comment state
        if scanner_state.in_comment {
            if next_char == '\n' {
                scanner_state.in_comment = false;
                scanner_state.line_num += 1;
                cur_token = String::new();
            }

        // multiline comment state
        } else if scanner_state.in_multiline_comment {
            end_token = incr_cur_token(&mut cur_token, next_char);
            if end_token {
                if cur_token == "*/" {
                    scanner_state.in_multiline_comment = false;
                }
                cur_token = String::new();
            }
        } else if scanner_state.in_string {
            if next_char == '"' {

            }

        } else if scanner_state.in_char {

        } else {

        }

    }
    
}

pub fn scan(file_path: &Path) {
    let mut file = File::open(file_path).expect("Failed to Open File");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Failed to read string from file");
    scan_program(file_str);
}
