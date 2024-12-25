use super::AST;
use std::path::Path;
use super::super::scanner::scanner::scan_file;
use std::collections::HashMap;

#[derive(Clone)]
enum TokenType {
    Symbol, // either a keyword or one char symbol
    Char, 
    String,
    Int,
    Bool,
    Identifier,
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    token_value: String,
}

struct ParserState {
    tokens: Vec<Token>,
    token_idx: usize, 
}

impl ParserState {
    fn cur_token(&self) -> Token {
        return self.tokens[self.token_idx].clone();
    }

    fn consume(&mut self) {
        self.token_idx += 1;
    }

    fn check_token(&mut self, comp_token: &str) -> Result<bool, String> {
        if self.cur_token().token_value != comp_token {
            return Err(format!("Expected token: {}, got: {}", comp_token, self.cur_token().token_value));
        }
        self.consume();
        return Ok(true);
    }
}

// helper functions

fn unpack_token(symbol_text: &str) -> Token {
    let parts: Vec<&str> = symbol_text.split_whitespace().collect();
    let type_map: HashMap<&str, TokenType> = HashMap::from([
        ("IDENTIFIER", TokenType::Identifier),
        ("INTLITERAL", TokenType::Int),
        ("STRINGLITERAL", TokenType::String),
        ("CHARLITERAL", TokenType::Char),
        ("BOOLLITERAL", TokenType::Bool),
    ]);

    match parts[1] {
        "IDENTIFIER" | "CHARLITERAL" | "STRINGLITERAL" | "BOOLLITERAL" | "INTLITERAL" => {
            return Token {
                token_type: type_map.get(parts[1]).unwrap().clone(),
                token_value: parts[2].to_string(),
            }
        }
        _ => {
            return Token {
                token_type: TokenType::Symbol,
                token_value: parts[1].to_string(),
            }
        }
    }
}

// parse functions for each grammar rule

fn parse_program(parser_state: &mut ParserState) -> Result<AST::Program, String> {
    let mut program = AST::Program {
        imports: vec![],
        fields: vec![],
        methods: vec![],
    };
    
    while parser_state.token_idx < parser_state.tokens.len() {
        
    }
    return Ok(program);
}

pub fn parse_file(file_path: &Path) -> Result<AST::Program, String> {
    // Lex file first
    match scan_file(file_path) {
        Ok(tokens) => {            
            let mut parser_state = ParserState {
                tokens: tokens.iter().map(|x| unpack_token(x)).collect(),
                token_idx: 0,
            };
            // Parse tokens
            return parse_program(&mut parser_state);
        }, 
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

