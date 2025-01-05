use super::super::parser::parser::parse_file;
use super::super::parser::visitor;

pub fn interpret_file(input: &std::path::PathBuf, debug: bool) -> Result<(), Vec<String>> {
    let _input = std::fs::read_to_string(input).expect("Filename is incorrect.");
    match parse_file(input) {
        Ok(ast) => {
            return Ok(());
        }
        Err(errors) => {
            let errors = vec![errors];
            return Err(errors);
        }
    }
}

pub fn interpret(input: &std::path::PathBuf, mut writer: Box<dyn std::io::Write>, debug: bool) {
    match interpret_file(input, debug) {
        Ok(_) => {
            writeln!(writer, "Interpreted successfully.").unwrap();
        }
        Err(errors) => {
            for error in errors {
                writeln!(writer, "Error: {}", error).unwrap();
            }
        }
    }
}