use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use super::constants::is_whitespace;
use super::constants::is_alphabetic;
use super::constants::is_hex;
use super::constants::is_alphanumeric;
use super::constants::is_reserved_literal;
use super::constants::is_valid_symbol;
use super::constants::is_numeric;

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
        if cur_token.chars().last().unwrap() == 'L' {
            return true;
        } else if cur_token.chars().nth(1).unwrap() == 'x' {
            if is_hex(next_char) {
                cur_token.push(next_char);
                return false;
            }
        } else if next_char.is_numeric() {
            cur_token.push(next_char);
            return false;
        } else if next_char == 'L' {
            cur_token.push(next_char);
            return false;
        }
    }
    return true;
 }

/*
Add next_char to start state (most special symbols)
*/
fn add_start(cur_token: &mut String, next_char: char, scanner_state: &ScannerState) -> Result<bool, String> {
    if is_whitespace(next_char) {
        // reject single & and | tokens
        if cur_token == "&" || cur_token == "|" {
            return Err(format!("Scanner: Line {} - Error: invalid symbol: {}", scanner_state.line_num, cur_token).to_string())
        }
        return Ok(true);
    }
    // transition if next char is alpahnumeric, string, or char
    if is_alphanumeric(next_char) || next_char == '\"' || next_char == '\'' {
        if cur_token.len() > 0 {
            return Ok(true);
        }
        cur_token.push(next_char);
        return Ok(false);
    }
    
    // check valid non-alphanumeric char
    if !is_valid_symbol(next_char) {
        return Err(format!("Scanner: Line {} - Error: invalid symbol: {}", scanner_state.line_num, cur_token).to_string());
    }

    let test_token = format!("{}{}", cur_token, next_char);
    match cur_token.len() {
        0 => (),
        1 => {
            match test_token.as_str() {
                "++" | "--" | "==" | "!=" | "<=" | ">=" | "&&" | "||" | "+=" | "-=" | "*=" | "/=" | "%=" | "//" | "/*" => (),
                _ => {
                    return Ok(true);
                }
            }
        },
        _ => {
            for c in test_token.chars() {
                if c != '-' {
                    return Ok(true);
                }
            }
        }
    }

    cur_token.push(next_char);
    return Ok(false);
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
        if cur_token.len() == 1 {
            return Err(format!("Scanner: Line {} - Error: empty char", scanner_state.line_num).to_string());
        }
        return Ok(true);
    }
    // check if next token is valid
    match is_valid_char(&cur_token, next_char) {
        true => {
            return Ok(false);
        }
        false => {
            return Err(format!("Scanner: Line {} - Error: invalid char: {}", scanner_state.line_num, cur_token).to_string());
        }
    }
}

/*
Process incoming string chars
*/
fn process_str_char(scanner_state: &mut ScannerState, str_char_phrase: &mut String, next_char: char) -> Result<bool, String> {
    // check if next token is valid
    if !is_valid_char(&str_char_phrase, next_char) {
        return Err(format!("Scanner: Line {} - Error: invalid char: {}", scanner_state.line_num, str_char_phrase).to_string());
    }
    match str_char_phrase.len() {
        1 => {
            if next_char != '\\' {
                return Ok(true);
            } else {
                return Ok(false);
            }
        },
        _ => return Ok(true),
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
        // println!("STARTING: state {:?} cur_token {} next_char {} str_char_phrase {}", scanner_state.state, cur_token, next_char, str_char_phrase);
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
                if next_char == '\"' && str_char_phrase != "\'\\" {
                    scanner_state.state = ScanType::Start;
                    cur_token.push(next_char);
                    tokens.push(format!("{} STRINGLITERAL {}", scanner_state.line_num, cur_token));
                    cur_token = String::new();
                } else {
                    match process_str_char(&mut scanner_state, &mut str_char_phrase, next_char) {
                        Ok(finish_char) => {
                            str_char_phrase.push(next_char);
                            if finish_char {
                                cur_token.push_str(&str_char_phrase[1..]);
                                str_char_phrase = "\'".to_string();                        
                            } 
                        }
                        Err(e) => {
                            return Err(e);
                        }
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
                        match cur_token.as_str() {
                            "true" | "false" => {
                                tokens.push(format!("{} BOOLEANLITERAL {}", scanner_state.line_num, cur_token));
                            },
                            _ => {
                                tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
                            }
                        }
                    } else {
                        tokens.push(format!("{} IDENTIFIER {}", scanner_state.line_num, cur_token));
                    }
                    cur_token = String::new();
                    if is_valid_symbol(next_char) || is_whitespace(next_char) || is_numeric(next_char) {
                        if !is_whitespace(next_char) {
                            cur_token.push(next_char);
                        }
                        scanner_state.state = ScanType::Start;
                    } else {
                        return Err(format!("Scanner: Line {} - Error: invalid symbol: {}", scanner_state.line_num, next_char).to_string());
                    }
                }
            },
            ScanType::Integer => {
                if add_integer(&mut cur_token, next_char) {
                    if cur_token.chars().last().unwrap() == 'L' {
                        tokens.push(format!("{} LONGLITERAL {}", scanner_state.line_num, cur_token)); 
                    } else {
                        tokens.push(format!("{} INTLITERAL {}", scanner_state.line_num, cur_token));
                    }
                    cur_token = String::new();
                    if is_valid_symbol(next_char) || is_whitespace(next_char) || is_alphabetic(next_char) {
                        if !is_whitespace(next_char) {
                            cur_token.push(next_char);
                        }
                        if is_alphabetic(next_char) {
                            scanner_state.state = ScanType::Identifier;
                        } else {
                            scanner_state.state = ScanType::Start;
                        }
                    } else {
                        return Err(format!("Scanner: Line {} - Error: invalid symbol: {}", scanner_state.line_num, next_char).to_string());
                    }
                }
            },
            ScanType::Start => {
                match add_start(&mut cur_token, next_char, &scanner_state) {
                    Ok(finish_char) => {
                        if finish_char {
                            if cur_token.len() > 0 {
                                tokens.push(format!("{} {}", scanner_state.line_num, cur_token));
                            }
                            cur_token = String::new();
                            // check if next_char is whitespace, if not we add it to cur_token
                            if !is_whitespace(next_char) {
                                cur_token.push(next_char);
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
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
                    str_char_phrase = "\'".to_string();
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
        return Err(format!("Scanner: Line {} - Error: invalid token: {}", scanner_state.line_num - 1, cur_token).to_string());
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
            std::process::exit(0);
        }
        Err(e) => {
            if let Err(write_error) = writeln!(writer, "{}", e) {
                eprintln!("Failed to write line to output: {}", write_error);
            }
            std::process::exit(1);
        }
    }
}
