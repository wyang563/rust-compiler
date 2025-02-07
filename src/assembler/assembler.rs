use super::super::semantics::semantics::interpret_file;
use super::super::parser::parser::parse_file;
use super::super::cfg::cfg::{ProgramGraph, ControlFlowGraph, construct_program_graph};

pub fn assemble(input: &std::path::PathBuf, mut writer: Box<dyn std::io::Write>, debug: bool) {
    match parse_file(input) {
        Ok(ast) => {
            match interpret_file(input, debug) {
                Ok(_) => {
                    // CFG construction
                    let prog_graph = construct_program_graph(ast);   
                    println!("{:#?}", prog_graph);
                }
                Err(e) => {
                    writeln!(writer, "Error in semantic analysis of file with the following errors reported: \n {:?}", e).unwrap();
                }
            }
        }
        Err(e) => {
            writeln!(writer, "Error parsing input file with error: {:?}", e).unwrap();
        }
    }
}