use super::super::parser::parser::parse_file;
use super::super::parser::AST;
use super::super::parser::visitor::Visitor;
use super::symbol_table::{Entry,
                          VarEntry,
                          ArrayEntry,
                          MethodEntry,
                          ImportEntry,
                          GlobalTable, 
                          MethodTable,
                          Type,
                        };

use std::collections::HashMap;

pub struct Interpreter {
    global_scope: GlobalTable,
    current_scope: Option<MethodTable>,
    errors: Vec<String>,
    correct: bool,
}

impl Interpreter {
    // find whether variable has been declared
    fn find_var(&mut self, var_name: &str, scope: Box<Option<MethodTable>>) -> Result<Entry, ()> {
        // Rule 2: No identifier is used before it is declared
        // Rule 12: An ⟨id⟩ used as a ⟨location⟩ must name a declared local/global variable or parameter.
        if self.current_scope.is_none() {
            if self.global_scope.entries.contains_key(var_name) {
                return Ok(self.global_scope.entries[var_name].clone());
            }
            self.correct = false;
            self.errors.push(format!("Error: Identifier {} is used before it is declared.", var_name));
            return Err(());

        } else {
            let current_scope = self.current_scope.as_ref().unwrap();
            if current_scope.entries.contains_key(var_name) {
                return Ok(current_scope.entries[var_name].clone());
            }
            let parent = current_scope.parent.clone();
            return self.find_var(var_name, parent);
        }
    }

    fn check_declared(&mut self, var_name: &str) -> bool {
        // Rule 1: No identifier is declared twice in the same scope
        if self.current_scope.is_none() {
            if self.global_scope.entries.contains_key(var_name) {
                self.errors.push(format!("Error: Identifier {} is declared twice in the same scope.", var_name));
                self.correct = false;
                return true;
            }
        } else {
            if self.current_scope.is_none() {
                return false;
            }
            let current_scope = self.current_scope.as_ref().unwrap();
            if current_scope.entries.contains_key(var_name) {
                self.errors.push(format!("Error: Identifier {} is declared twice in the same scope.", var_name));
                self.correct = false;
                return true;
            }
        }
        return false;
    }
}

impl Visitor for Interpreter {
    fn visit_program(&mut self, program: &AST::Program) {
        // create global scope
        self.global_scope = GlobalTable {
            entries: HashMap::new(),            
        };
        self.current_scope = None;
        self.errors = vec![];
        self.correct = true;

        for import_decl in &program.imports {
            self.visit_import_decl(import_decl);
        }
        for field_decl in &program.fields {
            self.visit_field_decl(field_decl);
        }
        for method_decl in &program.methods {
            self.visit_method_decl(method_decl);
        }
    }

    fn visit_import_decl(&mut self, import_decl: &AST::ImportDecl) { 
        let import_id = import_decl.import_id.name.clone();
        // Rule 1: No identifier is declared twice in the same scope
        if self.check_declared(import_id.as_str()) {
            return;
        }
        let import_entry = ImportEntry {
            name: import_id.clone(),
            return_type: Type::Int,
            is_const: false,
        };
        self.global_scope.entries.insert(import_id, Entry::Import(import_entry));
    }

    fn visit_field_decl(&mut self, field_decl: &AST::FieldDecl) {
        for var_decl in &field_decl.vars {
            self.visit_var_decl(var_decl);
        }
    }

    fn visit_method_decl(&mut self, method_decl: &AST::MethodDecl) {
        // Rule 1: No identifier is declared twice in the same scope
        // Rule 3: The program contains a definition for a method called main that has type void and takes no parameters.
    }

    fn visit_block(&mut self, block: &AST::Block) {
        
    }

    fn visit_var_decl(&mut self, var_decl: &AST::VarDecl) {
        // Rule 1: No identifier is declared twice in the same scope
        
        // Rule 4: All types of initializers must match the type of the variable being initialized.
        // Rule 6: If present, the ⟨int literal⟩ in an array declaration must be greater than 0.
        // Rule 22: Declarations of const locations must have an initializer
        // Rule 5: Array initializers have either a declared length or an initializer list, but not both.

    }

    fn visit_method_arg_decl(&mut self, method_arg_decl: &AST::MethodArgDecl) {
        
    }

    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement) {
        // Rule 16: The ⟨expr⟩ in an if or while statement must have type bool , as well as the second ⟨expr⟩ of a for statement.   
    }

    fn visit_for_statement(&mut self, for_statement: &AST::ForStatement) {
        // Rule 16: The ⟨expr⟩ in an if or while statement must have type bool , as well as the second ⟨expr⟩ of a for statement.
    }

    fn visit_while_statement(&mut self, while_statement: &AST::WhileStatement) {
        // Rule 16: The ⟨expr⟩ in an if or while statement must have type bool , as well as the second ⟨expr⟩ of a for statement.
    }

    fn visit_return_statement(&mut self, return_statement: &AST::ReturnStatement) {
        // Rule 10: A return statement must not have a return value unless it appears in the body of a method that is declared to return a value
        // Rule 11: The expression in a return statement must have the same type as the declared result type of the enclosing method definition.
    }

    fn visit_statement_control(&mut self, statement_control: &AST::StatementControl) {
        // Rule 24: All break and continue statements must be contained within the body of a for or a while statement.
    }

    fn visit_assignment(&mut self, assignment: &AST::Assignment) {
        // Rule 20: The ⟨location⟩ and the ⟨expr⟩ in an assignment, ⟨location⟩ = ⟨expr⟩, must have the same type.
        // Rule 21: The⟨location⟩andthe⟨expr⟩inacompoundassignment,⟨location⟩+=⟨expr⟩,⟨location⟩-=⟨expr⟩, ⟨location⟩ *= ⟨expr⟩, ⟨location⟩ /= ⟨expr⟩, and ⟨location⟩ %= ⟨expr⟩, must be of type int . The same is true of the ⟨location⟩ in ++ and -- statements.
    }

    fn visit_method_call(&mut self, method_call: &AST::MethodCall) {
        // Rule 13: The ⟨id⟩ in a method statement must be a declared method or import.
        // Rule 7: The number and types of parameters in a method call (non-import) must be the same as the number and types of the declared parameters for the method.
        // Rule 9: String literals and array variables may not be used as parameters to non-import methods.
    }

    fn visit_len_call(&mut self, len_call: &AST::LenCall) {
        // Rule 15: The argument of the len operator must be an array variable.

    }

    fn visit_unary_expression(&mut self, unary_expression: &AST::UnaryExpression) {
        // Rule 8: If a method call is used as an expression, the method must return a result.
        // Rule 17: The operands of the unary minus, ⟨arith op⟩s and ⟨rel op⟩s must have type int
        // Rule 19: The operands of ⟨cond op⟩s and the operand of logical not ( ! ) must have type bool
    }

    fn visit_binary_expression(&mut self, binary_expression: &AST::BinaryExpression) {
        // Rule 8: If a method call is used as an expression, the method must return a result.
        // Rule 17: The operands of the unary minus, ⟨arith op⟩s and ⟨rel op⟩s must have type int
        // Rule 18: The operands of ⟨eq op⟩s must have the same type, either int or bool
        // Rule 19: The operands of ⟨cond op⟩s and the operand of logical not ( ! ) must have type bool
        
    }

    fn visit_index_expression(&mut self, index_expression: &AST::IndexExpression) {
        // Rule 8: If a method call is used as an expression, the method must return a result.
        // Rule 14: For all locations of the form ⟨id⟩[⟨expr⟩], the ⟨id⟩ must be an array variable and the type of ⟨expr⟩ must be int.
    }

    fn visit_array_literal(&mut self, array_literal: &AST::ArrayLiteral) {
        
    }

    fn visit_identifier(&mut self, identifier: &AST::Identifier) {
        let id_name = identifier.name.as_str();
        match identifier.status {
            0 => {
                // Rule 1: No identifier is declared twice in the same scope
                self.check_declared(id_name);
            },
            1 => {
                // Rule 2: No identifier is used before it is declared
                self.find_var(id_name);
            },
            2 => {
                // Rule 2: No identifier is used before it is declared
                match self.find_var(id_name) {
                    Ok(id_entry) => {
                        // Rule 23: const locations may not be assigned to
                        if id_entry.get_is_const() {
                            self.correct = false;
                            self.errors.push(format!("Error: Identifier {} is a const location and may not be assigned to.", id_name));
                        }
                    },
                    Err(()) => (),
                }
            },
            _ => {
                self.errors.push(format!("Error: Identifier status {} is invalid: not one of 0, 1, 2.", identifier.status));
                self.correct = false;
            }
        }
    }

    fn visit_int_constant(&mut self, int_constant: &AST::IntConstant) {
        // Rule 25: All integer literals must be in the 64 bit integer range: −9223372036854775808 ≤ x ≤ 9223372036854775807
    }

    fn visit_string_constant(&mut self, string_constant: &AST::StringConstant) {
        
    }

    fn visit_bool_constant(&mut self, bool_constant: &AST::BoolConstant) {
        
    }

    fn visit_char_constant(&mut self, char_constant: &AST::CharConstant) {
        
    }
}

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