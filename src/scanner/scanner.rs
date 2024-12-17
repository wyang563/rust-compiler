use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

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
fn incr_cur_token(state: &mut ScannerState, cur_token: &mut String, next_char: char) -> bool {
    if is_whitespace(next_char) && !state.in_string && !state.in_char {
        return true;
    }
    if next_char == '\n' {
        state.line_num += 1;
        if state.in_string || state.in_char {
            cur_token.push(next_char);
        }
        return true;
    }
    cur_token.push(next_char);
    return false;
}

/*
check if cur_token is a valid character
*/
fn is_valid_char(cur_token: &String, next_char: char) -> bool {
    if cur_token.len() == 1 {
        match next_char {
            '\'' | '\"' => return false,
            _ => return true,
        }
    } else if cur_token.len() == 2 {
        match next_char {
            '\\' | '\'' | '\"' | 'n' | 't' => return true,
            _ => return false,
        }  
    }
    return false;
}

fn process_char(tokens: &mut Vec<String>, scanner_state: &mut ScannerState, cur_token: &mut String, next_char: char) -> Result<bool, String> {
    if next_char == '\'' && cur_token != "\\" {
        return Ok(true);
    }
    // check if next token is valid
    if is_valid_char(&cur_token, next_char) {
        return Ok(false);
    } else {
        return Err(format!("Line {} - Error: invalid char: {}", scanner_state.line_num, cur_token).to_string());
    }
}

fn scan_program(file_str: String) -> Result<Vec<String>, String> {
    // init scanner state
    let mut scanner_state = ScannerState {
        in_comment: false,
        in_multiline_comment: false,
        in_string: false,
        in_char: false,
        line_num: 1,
    };

    let mut cur_token: String = String::new();
    let mut end  = false;
    let mut tokens: Vec<String> = Vec::new();
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
            incr_cur_token(&mut scanner_state, &mut cur_token, next_char);
            if cur_token == "*/" {
                scanner_state.in_multiline_comment = false;
            }
            cur_token = String::new();
            
        } else if scanner_state.in_string {
            if next_char == '\"' {

            }

        } else if scanner_state.in_char {
            match process_char(&mut tokens, &mut scanner_state, &mut cur_token, next_char) {
                Ok(finish_char) => {
                    incr_cur_token(&mut scanner_state, &mut cur_token, next_char);
                    if finish_char {
                        tokens.push(format!("{} CHARLITERAL {}", scanner_state.line_num, cur_token));
                        cur_token = String::new();
                        scanner_state.in_char = false;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }

        } else {
            if incr_cur_token(&mut scanner_state, &mut cur_token, next_char) {
                // TODO: add additional logic for handle identifiers
                cur_token = String::new();
            }
            if cur_token == "//" {
                scanner_state.in_comment = true;
                cur_token = String::new();
            
            } else if cur_token == "/*" {
                scanner_state.in_multiline_comment = true;
                cur_token = String::new();
            } else if next_char == '\'' {
                scanner_state.in_char = true;
            }
        }
    }
    return Ok(tokens);
}

pub fn scan_file(file_path: &Path) -> Result<Vec<String>, String> {
    let mut file = File::open(file_path).expect("Failed to Open File");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Failed to read string from file");
    match scan_program(file_str) {
        Ok(tokens) => {
            return Ok(tokens);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn scan(file_path: &Path, mut writer: Box<dyn std::io::Write>) {
    match scan_file(file_path) {
        Ok(parsed_lines) => {
            for line in parsed_lines {
                if let Err(e) = writeln!(writer, "{}", line) {
                    eprintln!("Failed to write line to output: {}", e);
                }
            }
        }
        Err(e) => {
            if let Err(write_error) = writeln!(writer, "{}", e) {
                eprintln!("Failed to write line to output: {}", write_error);
            }
        }
    }
}
