use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use super::constants::is_whitespace;
use super::constants::is_alphabetic;
use super::constants::is_hex;
use super::constants::is_alphanumeric;
use super::constants::is_reserved_literal;


#[derive(PartialEq, Eq, Debug)]
enum ScanType {
    Comment,
    MultilineComment,
    String,
    Char, 
    Integer,
    Identifier,
    Start,
}

struct ScannerState {
    state: ScanType,
    line_num: u32,
}

/*
Add next_char to cur_token when in char or string
*/
fn add_char_string(cur_token: &mut String, next_char: char) {
    cur_token.push(next_char);
}

/*
Add next_char to cur_token when in identifier
*/
fn add_identifier(cur_token: &mut String, next_char: char) -> bool {
    if is_alphanumeric(next_char) {
        cur_token.push(next_char);
        return false;
    }
    return true;
}

/*
Add next_char to integer
*/
fn add_integer(cur_token: &mut String, next_char: char) -> bool {
    if cur_token.len() == 1 {
        if next_char.is_numeric() || (cur_token == "0" && next_char == 'x') {
            cur_token.push(next_char);
            return false;
        } 
    } else if cur_token.len() > 1 {
        // check if hex number, otherwise only accept decimal digits
        if cur_token.chars().nth(1).unwrap() == 'x' {
            if is_hex(next_char) {
                cur_token.push(next_char);
                return false;
            }
        } else if next_char.is_numeric() {
            cur_token.push(next_char);
            return false;
        }
    }
    return true;
 }

/*
Add next_char to start state (most special symbols)
*/
fn add_start(cur_token: &mut String, next_char: char) -> bool {
    if is_whitespace(next_char) {
        return true;
    }
    if cur_token.len() > 0 {
        let cur_token_str = cur_token.as_str();
        match cur_token_str {
            "/" => {
                match next_char {
                    '/' | '*' | '=' => (),
                    _ => return true,
                }
            },
            "+" => {
                match next_char {
                    '+' | '=' => (),
                    _ => return true,
                }
            },
            "-" => {
                match next_char {
                    '-' | '=' => (),
                    _ => return true,
                }
            },
            "*" => {
                match next_char {
                    '=' | '/' => (),
                    _ => return true,
                }
            },
            "=" | "!" | "%" | "<" | ">" => {
                match next_char {
                    '=' => (),
                    _ => return true,
                }
            },
            "&" => {
                match next_char {
                    '&' => (),
                    _ => return true,
                }
            },
            "|" => {
                match next_char {
                    '|' => (),
                    _ => return true,
                }
            },
            _ => return true,
        }
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
            '\'' | '\"' | '\n' | '\t' => return false,
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

/*
Processes incoming character when we're processing characters
*/
fn process_char(scanner_state: &mut ScannerState, cur_token: &mut String, next_char: char) -> Result<bool, String> {
    if next_char == '\'' && cur_token != "\'\\" {
        return Ok(true);
    }
    // check if next token is valid
    match is_valid_char(&cur_token, next_char) {
        true => {
            return Ok(false);
        }
        false => {
            return Err(format!("Line {} - Error: invalid char: {}", scanner_state.line_num, cur_token).to_string());
        }
    }
}

fn scan_program(file_str: String) -> Result<Vec<String>, String> {
    // init scanner state
    let mut scanner_state = ScannerState {
        state: ScanType::Start,
        line_num: 1,
    };

    let mut cur_token: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut str_char_phrase = String::new();
    for idx in 0..file_str.len() {
        let next_char = file_str.chars().nth(idx).unwrap();
        // println!("STARTING: state {:?} cur_token {} next_char {}", scanner_state.state, cur_token, next_char);
        match scanner_state.state {
            ScanType::Comment => {
                if next_char == '\n' {
                    scanner_state.state = ScanType::Start;
                    cur_token = String::new();
                }
            },
            ScanType::MultilineComment => {
                if cur_token == "*" && next_char == '/' {
                    scanner_state.state = ScanType::Start;
                    cur_token = String::new();
                } else {
                    cur_token = next_char.to_string();
                }
            },
            ScanType::String => {
                if next_char == '\"' {
                    scanner_state.state = ScanType::Start;
                    str_char_phrase = String::new();
                }
                match process_char(&mut scanner_state, &mut str_char_phrase, next_char) {
                    Ok(finish_char) => {
                        add_char_string(&mut cur_token, next_char);
                        if finish_char {
                            str_char_phrase = String::new();                            
                        } else {
                            str_char_phrase.push(next_char);
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            ScanType::Char => {
                match process_char(&mut scanner_state, &mut cur_token, next_char) {
                    Ok(finish_char) => {
                        add_char_string(&mut cur_token, next_char);
                        if finish_char {
                            tokens.push(format!("{} CHARLITERAL {}", scanner_state.line_num, cur_token));
                            cur_token = String::new();
                            scanner_state.state = ScanType::Start;
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            ScanType::Identifier => {
                if add_identifier(&mut cur_token, next_char) {
                    if is_reserved_literal(&cur_token) {
                        tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
                    } else {
                        tokens.push(format!("{} IDENTIFIER {}", scanner_state.line_num, cur_token));
                    }
                    cur_token = String::new();
                    if !is_whitespace(next_char) {
                        cur_token.push(next_char);
                    }
                    scanner_state.state = ScanType::Start;
                }
            },
            ScanType::Integer => {
                if add_integer(&mut cur_token, next_char) {
                    tokens.push(format!("{} INTLITERAL {}", scanner_state.line_num, cur_token));
                    cur_token = String::new();
                    if !is_whitespace(next_char) {
                        cur_token.push(next_char);
                    }
                    if is_alphabetic(next_char) {
                        scanner_state.state = ScanType::Identifier;
                    } else {
                        scanner_state.state = ScanType::Start;
                    }
                }
            },
            ScanType::Start => {
                if add_start(&mut cur_token, next_char) {
                    if cur_token.len() > 0 {
                        tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
                    }
                    cur_token = String::new();
                    // check if next_char is whitespace, if not we add it to cur_token
                    if !is_whitespace(next_char) {
                        cur_token.push(next_char);
                    }
                }
                // check if we need to switch states
                if cur_token == "//" {
                    scanner_state.state = ScanType::Comment;
                    cur_token = String::new();
                
                } else if cur_token == "/*" {
                    scanner_state.state = ScanType::MultilineComment;
                    cur_token = String::new();
                } else if next_char == '\'' {
                    scanner_state.state = ScanType::Char;
                } else if next_char == '\"' {
                    scanner_state.state = ScanType::String;
                } else if next_char.is_numeric() {
                    scanner_state.state = ScanType::Integer;
                } else if is_alphabetic(next_char) {
                    scanner_state.state = ScanType::Identifier;
                }
            },    
        }
        if next_char == '\n' {
            scanner_state.line_num += 1;
        }
    }
    // final state error checking plus append last cur_token to output tokens vector
    if scanner_state.state == ScanType::Char || scanner_state.state == ScanType::String {
        return Err(format!("Line {} - Error: invalid token: {}", scanner_state.line_num - 1, cur_token).to_string());
    }
    if cur_token.len() > 0 {
        match scanner_state.state {
            ScanType::Integer => {
                tokens.push(format!("{} INTLITERAL {}", scanner_state.line_num, cur_token));
            },
            ScanType::Identifier => {
                if is_reserved_literal(&cur_token) {
                    tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
                } else {
                    tokens.push(format!("{} IDENTIFIER {}", scanner_state.line_num, cur_token));
                }
            },
            ScanType::Start => {
                tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
            },
            _ => (),
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
