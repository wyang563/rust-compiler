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
    current_scope: Option<Box<MethodTable>>,
    errors: Vec<String>,
    correct: bool,

    // func parameters + flags
    checking_type: bool, // whether we are checking the type of a variable, used during field declarations
    init_method: bool, // whether we are initializing a method, this is so we don't create two scopes when we do visit method_decl and then visit_block
    init_type: Type, // type of varaible being initialized
}

impl Interpreter {
    // find whether variable has been declared
    fn find_var(&mut self, var_name: &str, scope: Option<Box<MethodTable>>) -> Result<Entry, ()> {
        // Rule 2: No identifier is used before it is declared
        // Rule 12: An ⟨id⟩ used as a ⟨location⟩ must name a declared local/global variable or parameter.
        if self.current_scope.is_none() {
            if self.global_scope.entries.contains_key(var_name) {
                return Ok(self.global_scope.entries[var_name].clone());
            }
            self.push_error(&format!("Error: Identifier {} is used before it is declared.", var_name));
            return Err(());

        } else {
            let current_scope = scope.unwrap();
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
                self.push_error(&format!("Error: Identifier {} is declared twice in the same scope.", var_name));
                return true;
            }
        } else {
            if self.current_scope.is_none() {
                return false;
            }
            let current_scope = self.current_scope.as_ref().unwrap();
            if current_scope.entries.contains_key(var_name) {
                self.push_error(&format!("Error: Identifier {} is declared twice in the same scope.", var_name));
                return true;
            }
        }
        return false;
    }

    fn write_to_table(&mut self, var_name: &str, entry: Entry) {
        if self.current_scope.is_none() {
            self.global_scope.entries.insert(var_name.to_string(), entry);
        } else {
            let current_scope = self.current_scope.as_mut().unwrap();
            current_scope.entries.insert(var_name.to_string(), entry);
        }
    }

    fn string_to_type(&mut self, str_type: &str) -> Type {
        match str_type {
            "int" => Type::Int,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => panic!("invalid type"),
        }
    }

    fn push_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
        self.correct = false;
    }

    fn visit_literal(&mut self, literal: &AST::ASTNode) {
        self.checking_type = true;
        // Rule 4: All types of initializers must match the type of the variable being initialized.
        match literal {
            AST::ASTNode::IntConstant(int_constant) => {
                self.visit_int_constant(int_constant);
            },
            AST::ASTNode::BoolConstant(bool_constant) => {
                self.visit_bool_constant(bool_constant);
            },
            AST::ASTNode::CharConstant(char_constant) => {
                self.visit_char_constant(char_constant);
            },
            _ => {
                self.push_error("Error: invalid type for literal - expected either int or bool.");
            },
        }
        self.checking_type = false;
    }
}

impl Visitor for Interpreter {
    fn visit_program(&mut self, program: &AST::Program) {
        for import_decl in &program.imports {
            self.visit_import_decl(import_decl);
        }
        for field_decl in &program.fields {
            self.visit_field_decl(field_decl);
        }
        for method_decl in &program.methods {
            self.visit_method_decl(method_decl);
        }
        // Rule 3: The program contains a definition for a method called main that has type void and takes no parameters.
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
        self.init_method = true;

        // Rule 1: No identifier is declared twice in the same scope
        self.visit_identifier(&method_decl.name);

        let method_type = self.string_to_type(method_decl.type_name.as_str());
        let method_name_str = method_decl.name.name.as_str();

        // write function to global scope
        self.write_to_table(method_decl.name.name.as_str(), Entry::Method( MethodEntry {
            name: method_name_str.to_string(),
            return_type: method_type.clone(),
            is_const: false,
            param_list: vec![],
            param_count: 0,
        }));

        // create new scope
        let method_table = MethodTable {
            entries: HashMap::new(),
            method_return_type: method_type.clone(),
            parent: self.current_scope.take(),
        };
        
        self.current_scope = Some(Box::new(method_table));
        // add method args to scope
        for arg in &method_decl.args {
            self.visit_method_arg_decl(arg.as_ref());
        }
        self.visit_block(&method_decl.body);
    }

    fn visit_block(&mut self, block: &AST::Block) {
        
    }

    fn visit_var_decl(&mut self, var_decl: &AST::VarDecl) {
        self.init_type = self.string_to_type(var_decl.type_name.as_str());
        let var_name = var_decl.name.as_ref().name.as_str();

        // Rule 1: No identifier is declared twice in the same scope
        self.visit_identifier(var_decl.name.as_ref());
        
        // Rule 22: Declarations of const locations must have an initializer
        if var_decl.is_const && var_decl.initializer.as_ref().is_none() {
            self.push_error(&format!("Error: Const location {} must have an initializer.", var_decl.name.as_ref().name));
            return;
        }

        if var_decl.is_array {
            if let Some(array_len_node) = var_decl.array_len.as_ref() {
                self.visit_int_constant(array_len_node);
                // Rule 6: If present, the ⟨int literal⟩ in an array declaration must be greater than 0.
                if array_len_node.value.parse::<i64>().unwrap() <= 0 {
                    self.push_error(&format!("Error: Array initializer length must be greater than 0."));
                }
                // Rule 5: Array initializers have either a declared length or an initializer list, but not both.
                // TODO: check if it's supposed to be .as_ref().as_ref() or only one .as_ref()
                if var_decl.initializer.as_ref().as_ref().is_some() {
                    self.push_error(&format!("Error: Array initializers have either a declared length or an initializer list, but not both."));
                }
            } else {
                // Rule 5: Array initializers have either a declared length or an initializer list, but not both.
                if var_decl.initializer.as_ref().as_ref().is_none() {
                    self.push_error(&format!("Error: Array initializers have either a declared length or an initializer list, but not both."));
                }

                match var_decl.initializer.as_ref().as_ref().unwrap() {
                    AST::ASTNode::ArrayLiteral(array_literal) => {
                        self.visit_array_literal(array_literal);
                    },
                    _ => {
                        self.push_error(&format!("Error: expected an array list as initializer for variable of type array."));
                    }
                }
            } 
            // add to symbol table
            let array_type = match self.init_type {
                Type::Int => Type::IntArray,
                Type::Bool => Type::BoolArray,
                _ => panic!("invalid type"),
            };

            self.write_to_table(var_name, Entry::Array( ArrayEntry {
                name: var_name.to_string(),
                var_type: array_type,
                is_const: var_decl.is_const,
            }));
        } 
        
        else {
            if var_decl.initializer.as_ref().as_ref().is_some() {
                self.visit_literal(var_decl.initializer.as_ref().as_ref().unwrap());
            }
            self.write_to_table(var_name, Entry::Var( VarEntry {
                name: var_name.to_string(),
                var_type: self.init_type.clone(),
                is_const: var_decl.is_const,
            }));
        }
    }

    fn visit_method_arg_decl(&mut self, method_arg_decl: &AST::MethodArgDecl) {
        let arg_name = method_arg_decl.name.as_ref().name.as_str();
        let arg_type = self.string_to_type(method_arg_decl.type_name.as_str());
        // Rule 1: No identifier is declared twice in the same scope
        self.visit_identifier(method_arg_decl.name.as_ref());
        self.write_to_table(arg_name, Entry::Var( VarEntry {
            name: arg_name.to_string(),
            var_type: arg_type,
            is_const: false,
        }));
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
        for literal in &array_literal.array_values {
            self.visit_literal(literal);
        }
    }

    fn visit_identifier(&mut self, identifier: &AST::Identifier) {
        let id_name = identifier.name.as_str();
        match identifier.status {
            // init variable
            0 => {
                // Rule 1: No identifier is declared twice in the same scope
                self.check_declared(id_name);
            },
            // read variable
            1 => {
                // Rule 2: No identifier is used before it is declared
                _ = self.find_var(id_name, self.current_scope.clone());
            },
            // write variable
            2 => {
                // Rule 2: No identifier is used before it is declared
                match self.find_var(id_name, self.current_scope.clone()) {
                    Ok(id_entry) => {
                        // Rule 23: const locations may not be assigned to
                        if id_entry.get_is_const() {
                            self.push_error(&format!("Error: Identifier {} is a const location and may not be assigned to.", id_name));
                        }
                    },
                    Err(()) => (),
                }
            },
            _ => {
                self.push_error(&format!("Error: Identifier status {} is invalid: not one of 0, 1, 2.", identifier.status));
            }
        }
    }

    fn visit_int_constant(&mut self, int_constant: &AST::IntConstant) {
        // Rule 25: All integer literals must be in the 64 bit integer range: −9223372036854775808 ≤ x ≤ 9223372036854775807

        // hex numbers
        if int_constant.value.contains("x") {
            match i64::from_str_radix(&int_constant.value[2..], 16) {
                Ok(_) => (),
                Err(_) => {
                    self.push_error(&format!("Error: Integer literal {} is out of 64 bit range.", int_constant.value));
                }
            }
        } else {
            match int_constant.value.parse::<i64>() {
                Ok(_) => (),
                Err(_) => {
                    self.push_error(&format!("Error: Integer literal {} is out of 64 bit range.", int_constant.value));
                }
            }
        }
        if self.checking_type {
            if self.init_type != Type::Int {
                self.push_error(&format!("Error: expected {:?} as type for initializer variable not int", self.init_type));
            }
        }
    }

    fn visit_string_constant(&mut self, string_constant: &AST::StringConstant) {
        
    }

    fn visit_bool_constant(&mut self, bool_constant: &AST::BoolConstant) {
        if self.checking_type {
            if self.init_type != Type::Bool {
                self.push_error(&format!("Error: expected {:?} as type for initializer variable not bool", self.init_type));
            }
        }
    }

    fn visit_char_constant(&mut self, char_constant: &AST::CharConstant) {
        if self.checking_type {
            if self.init_type != Type::Int {
                self.push_error(&format!("Error: expected {:?} as type for initializer variable not int", self.init_type));
            }
        }
    }
}

pub fn interpret_file(input: &std::path::PathBuf, debug: bool) -> Result<(), Vec<String>> {
    let _input = std::fs::read_to_string(input).expect("Filename is incorrect.");
    match parse_file(input) {
        Ok(ast) => {
            let mut interpreter = Interpreter {
                global_scope: GlobalTable {
                    entries: HashMap::new(),
                },
                current_scope: None,
                errors: vec![],
                correct: true,
                checking_type: false,
                init_method: false,
                init_type: Type::None,
            };
            ast.accept(&mut interpreter);
            if interpreter.correct {
                return Ok(());
            } else {
                return Err(interpreter.errors);
            }
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
                writeln!(writer, "{}", error).unwrap();
            }
        }
    }
}