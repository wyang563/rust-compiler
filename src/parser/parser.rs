use clap::Id;

use super::AST;
use std::path::Path;
use super::super::scanner::scanner::scan_file;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    Symbol, // either a keyword or one char symbol
    Char, 
    String,
    Int,
    Bool,
    Identifier,
}

#[derive(Clone)]
#[derive(Debug)]
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

    fn check_token(&mut self, comp_token: &str, consume: bool) -> Result<(), String> {
        if self.cur_token().token_value != comp_token {
            return Err(format!("Expected token: {}, got: {}", comp_token, self.cur_token().token_value));
        }
        if consume { // we have consume as a parameter in case we need to store the value of the current token after checking
            self.consume(); 
        }
        return Ok(());
    }

    fn check_multiple_tokens(&mut self, comp_tokens: Vec<&str>, consume: bool) -> Result<(), String> {
        if comp_tokens.contains(&self.cur_token().token_value.as_str()) {
            if consume {
                self.consume();
            }
            return Ok(());
        }
        return Err(format!("Expected one of: {:?}, got: {}", comp_tokens, self.cur_token().token_value));
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

fn parse_int_literal(parser_state: &mut ParserState) -> Result<AST::IntConstant, String> {
    match parser_state.cur_token().token_type {
        TokenType::Int => {
            let int_val = AST::IntConstant::new(parser_state.cur_token().token_value.as_str());
            parser_state.consume();
            return Ok(int_val);
        },
        _ => return Err(format!("Expected int literal, got: {:?}", parser_state.cur_token().token_value)),
    }
}

fn parse_char_literal(parser_state: &mut ParserState) -> Result<AST::CharConstant, String> {
    
}

fn parse_bool_literal(parser_state: &mut ParserState) -> Result<AST::BoolConstant, String> {
    
}

fn parse_string_literal(parser_state: &mut ParserState) -> Result<AST::StringConstant, String> {
    
}

fn parse_literal(parser_state: &mut ParserState) -> Result<AST::ASTNode, String> {

}

fn parse_identifier(parser_state: &mut ParserState) -> Result<AST::Identifier, String> {
    match parser_state.cur_token().token_type {
        TokenType::Identifier => {
            let id = AST::Identifier {
                name: parser_state.cur_token().token_value.clone(),
            };
            parser_state.consume();
            return Ok(id);
        },
        _ => return Err(format!("Expected identifier, got: {:?}", parser_state.cur_token().token_value)),
    }
}

fn parse_initializer(parser_state: &mut ParserState) -> Result<AST::ASTNode, String> {
    
}

fn parse_array_initializer(parser_state: &mut ParserState) -> Result<AST::ArrayLiteral, String> {
    
}

fn parse_import_decl(parser_state: &mut ParserState) -> Result<AST::Identifier, String> {
    parser_state.check_token("import", true)?;
    let import_id = parse_identifier(parser_state)?;
    parser_state.check_token(";", true)?;
    return Ok(import_id);
}

fn parse_field_decl(parser_state: &mut ParserState) -> Result<AST::FieldDecl, String> {
    // consume const if it exists
    let mut is_const = false;
    if parser_state.cur_token().token_value == "const" {
        parser_state.consume();
        is_const = true;
    }

    // consume type
    parser_state.check_multiple_tokens(vec!["int", "bool"], false)?;
    let field_type = parser_state.cur_token().token_value.clone();
    parser_state.consume();

    // consume identifiers
    let mut vars: Vec<Box<AST::VarDecl>> = vec![];
    
    loop {
        let var_id = parse_identifier(parser_state)?;
        let mut array_len: Option<AST::IntConstant> = None;
        let mut initializer: Option<AST::ASTNode> = None;

        // case if we have id[int] initializer
        if parser_state.cur_token().token_value.clone() == "[" {
            parser_state.consume();
            array_len = Some(parse_int_literal(parser_state)?);
            parser_state.check_token("]", true)?;
        }

        // case if we have = sign in var initializer
        if parser_state.cur_token().token_value.clone() == "=" {
            parser_state.consume();
            initializer = Some(parse_initializer(parser_state)?);
        }
        
        // add new var to array
        vars.push(Box::new(AST::VarDecl {
            name: Box::new(var_id),
            array_len: Box::new(array_len),
            initializer: Box::new(initializer),
        }));

        if parser_state.cur_token().token_value.clone() != "," {
            break;
        }
        parser_state.consume();
    }
    parser_state.check_token(";", true)?;
    return Ok(AST::FieldDecl {
        is_const: is_const,
        type_name: field_type,
        vars: vars,
    });   
}

fn parse_program(parser_state: &mut ParserState) -> Result<AST::Program, String> {
    let mut program = AST::Program {
        imports: vec![],
        fields: vec![],
        methods: vec![],
    };

    while parser_state.token_idx < parser_state.tokens.len() {
        // consume imports
        while parser_state.cur_token().token_value == "import" {
            let import_id = parse_import_decl(parser_state)?;
            program.imports.push(Box::new(import_id));
        }
        // consume field declarations
        
        // consume method declarations
        
    }
    return Ok(program);
}

pub fn parse_file(file_path: &Path) -> Result<AST::Program, String> {
    // Lex file first
    match scan_file(file_path) {
        Ok(tokens) => {            
            let mut parser_state = ParserState {
                tokens: tokens.iter().map(|x| unpack_token(x.as_str())).collect(),
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

