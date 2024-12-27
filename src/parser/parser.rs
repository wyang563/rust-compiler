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
    EOF,
}

#[derive(Clone)]
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_value: String,
    line_num: String,
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
            return Err(format!("Line: {} - Expected token: {}, got: {}", self.cur_token().line_num, comp_token, self.cur_token().token_value));
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
        return Err(format!("Line: {} - Expected one of: {:?}, got: {}", self.cur_token().line_num, comp_tokens, self.cur_token().token_value));
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
                line_num: parts[0].to_string(),
            }
        }
        _ => {
            return Token {
                token_type: TokenType::Symbol,
                token_value: parts[1].to_string(),
                line_num: parts[0].to_string(),
            }
        }
    }
}

// parse functions for each grammar rule

fn parse_int_literal(parser_state: &mut ParserState) -> Result<AST::IntConstant, String> {
    match parser_state.cur_token().token_type {
        TokenType::Int => {
            let int_val = AST::IntConstant {
                value: parser_state.cur_token().token_value.clone(),
            };
            parser_state.consume();
            return Ok(int_val);
        },
        _ => return Err(format!("Line: {} - Expected int literal, got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }
}

fn parse_char_literal(parser_state: &mut ParserState) -> Result<AST::CharConstant, String> {
    match parser_state.cur_token().token_type {
        TokenType::Char => {
            let extract_char = parser_state.cur_token().token_value.clone();
            let char_val = AST::CharConstant {
                value: extract_char[1..extract_char.len()-1].to_string(),
            };
            parser_state.consume();
            return Ok(char_val);
        },
        _ => return Err(format!("Line: {} - Expected char literal, got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }
}

fn parse_bool_literal(parser_state: &mut ParserState) -> Result<AST::BoolConstant, String> {
    match parser_state.cur_token().token_type {
        TokenType::Bool => {
            let bool_val = AST::BoolConstant {
                value: parser_state.cur_token().token_value == "true",
            };
            parser_state.consume();
            return Ok(bool_val);
        },
        _ => return Err(format!("Line: {} - Expected bool literal, got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }
}

fn parse_string_literal(parser_state: &mut ParserState) -> Result<AST::StringConstant, String> {
    match parser_state.cur_token().token_type {
        TokenType::String => {
            let extract_string = parser_state.cur_token().token_value.clone();
            let string_val = AST::StringConstant {
                value: extract_string[1..extract_string.len()-1].to_string(),
            };
            parser_state.consume();
            return Ok(string_val);
        },
        _ => return Err(format!("Line: {} - Expected string literal, got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }
}

fn parse_literal(parser_state: &mut ParserState) -> Result<AST::ASTNode, String> {
    if parser_state.cur_token().token_value == "-" {
        parser_state.consume();
        return Ok(AST::ASTNode::IntConstant(parse_int_literal(parser_state)?));
    }
    match parser_state.cur_token().token_type {
        TokenType::Int => {
            return Ok(AST::ASTNode::IntConstant(parse_int_literal(parser_state)?));
        },
        TokenType::Char => {
            return Ok(AST::ASTNode::CharConstant(parse_char_literal(parser_state)?));
        },
        TokenType::Bool => {
            return Ok(AST::ASTNode::BoolConstant(parse_bool_literal(parser_state)?));
        },
        _ => return Err(format!("Line: {} - Expected literal (char, int, bool), got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }  
}

fn parse_array_literal(parser_state: &mut ParserState) -> Result<AST::ArrayLiteral, String> {
    parser_state.check_token("{", true)?;
    let mut array_vals = vec![];
    loop {
        array_vals.push(Box::new(parse_literal(parser_state)?));
        if parser_state.cur_token().token_value != "," {
            break;
        }
        parser_state.check_token(",", true)?;
    }
    parser_state.check_token("}", true)?;
    return Ok(AST::ArrayLiteral {
        array_values: array_vals,
    });
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
        _ => return Err(format!("Line: {} - Expected identifier, got: {:?}", parser_state.cur_token().line_num, parser_state.cur_token().token_value)),
    }
}

fn parse_initializer(parser_state: &mut ParserState) -> Result<AST::ASTNode, String> {
    if parser_state.cur_token().token_value == "{" {
        let initializer = parse_array_literal(parser_state)?;
        return Ok(AST::ASTNode::ArrayLiteral(initializer));
    } else {
        return parse_literal(parser_state);
    }
}

fn parse_block(parser_state: &mut ParserState) -> Result<AST::Block, String> {
    
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

fn parse_method_decl(parser_state: &mut ParserState) -> Result<AST::MethodDecl, String> {
    // consume method type
    parser_state.check_multiple_tokens(vec!["int", "bool", "void"], false)?;
    let method_type = parser_state.cur_token().token_value.clone();
    parser_state.consume();
    let method_name = parse_identifier(parser_state)?;
    parser_state.check_token("(", true)?;
    let mut args: Vec<Box<AST::MethodArgDecl>> = vec![];
    // parse args
    if vec!["int", "bool"].contains(&parser_state.cur_token().token_value.as_str()) {
        loop {
            let arg_type = parser_state.cur_token().token_value.clone();
            parser_state.consume();
            let arg_name = parse_identifier(parser_state)?;
            args.push(Box::new(AST::MethodArgDecl {
                type_name: arg_type,
                name: Box::new(arg_name),
            }));
            if parser_state.cur_token().token_value != "," {
                break;
            }
            parser_state.check_token(",", true)?;
        }
    }
    parser_state.check_token(")", true)?;
    let method_block = parse_block(parser_state)?;
    return Ok(AST::MethodDecl {
        type_name: method_type,
        name: method_name,
        args: args,
        body: Box::new(method_block),
    });
}

fn parse_program(parser_state: &mut ParserState) -> Result<AST::Program, String> {
    let mut program = AST::Program {
        imports: vec![],
        fields: vec![],
        methods: vec![],
    };

    // consume imports
    while parser_state.cur_token().token_value == "import" {
        let import_id = parse_import_decl(parser_state)?;
        program.imports.push(Box::new(import_id));
    }

    // consume field declarations
    while ["int", "bool", "const"].contains(&parser_state.cur_token().token_value.as_str()) {
        let field_decl = parse_field_decl(parser_state)?;
        program.fields.push(Box::new(field_decl));
    }

    // consume method declarations
    while ["int", "bool", "void"].contains(&parser_state.cur_token().token_value.as_str()) {
        let method_decl = parse_method_decl(parser_state)?;
        program.methods.push(Box::new(method_decl));
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
            // Add EOF token
            parser_state.tokens.push(Token {
                token_type: TokenType::EOF,
                token_value: "EOF".to_string(),
                line_num: "".to_string(),
            });
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

