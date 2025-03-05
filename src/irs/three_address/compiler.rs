use super::super::super::parser::visitor::{Visitor};
use super::instructions::{Instruction, ArrayInstruction, Call, InstructionType, PushInstruction, FlowInstruction, UnaryInstruction, BinaryInstruction, Ret};
use super::super::super::parser::AST;
use super::super::super::semantics::symbol_table::{Entry, ArrayEntry, VarEntry, MethodEntry, ImportEntry, GlobalTable, MethodTable};
use std::collections::HashMap;

pub struct ThreeAddressCode {
    var_entries: Vec<Entry>, // vector of entry variables that can be referenced
    global_instructions: Vec<Box<Instruction>>,
    func_instructions: HashMap<String, Vec<Box<Instruction>>>,
    scopes: Vec<Box<MethodTable>>,

    // flags
    is_global: bool,
}

impl Visitor for ThreeAddressCode {
    fn visit_program(&mut self, _program: &AST::Program) {
        self.is_global = true;
        for import in &_program.imports {
            import.accept(self);
        }

        for field in &_program.fields {
            field.accept(self);
        }

        self.is_global = false;
        for method in &_program.methods {
            method.accept(self);
        }
    }

    fn visit_import_decl(&mut self, _import_decl: &AST::ImportDecl) {
        
    }
    
    fn visit_field_decl(&mut self, _field_decl: &AST::FieldDecl) {
        for var_decl in &_field_decl.vars {
            var_decl.accept(self);
        }
    }

    fn visit_method_decl(&mut self, _method_decl: &AST::MethodDecl) {}

    fn visit_block(&mut self, _block: &AST::Block) {}

    fn visit_var_decl(&mut self, _var_decl: &AST::VarDecl) {
        
    }

    fn visit_method_arg_decl(&mut self, _method_arg_decl: &AST::MethodArgDecl) {}

    fn visit_if_statement(&mut self, _if_statement: &AST::IfStatement) {}

    fn visit_for_statement(&mut self, _for_statement: &AST::ForStatement) {}

    fn visit_while_statement(&mut self, _while_statement: &AST::WhileStatement) {}

    fn visit_return_statement(&mut self, _return_statement: &AST::ReturnStatement) {}

    fn visit_statement_control(&mut self, _statement_control: &AST::StatementControl) {}

    fn visit_assignment(&mut self, _assignment: &AST::Assignment) {}

    fn visit_expression(&mut self, _expr: &AST::ASTNode) {}

    fn visit_method_call(&mut self, _method_call: &AST::MethodCall) {}

    fn visit_len_call(&mut self, _len_call: &AST::LenCall) {}

    fn visit_int_cast(&mut self, _int_cast: &AST::IntCast) {}

    fn visit_long_cast(&mut self, _long_cast: &AST::LongCast) {}

    fn visit_unary_expression(&mut self, _unary_expression: &AST::UnaryExpression) {}

    fn visit_binary_expression(&mut self, _binary_expression: &AST::BinaryExpression) {}

    fn visit_index_expression(&mut self, _index_expression: &AST::IndexExpression) {}

    fn visit_array_literal(&mut self, _array_literal: &AST::ArrayLiteral) {}

    fn visit_location(&mut self, _location: &AST::ASTNode) {}

    fn visit_literal(&mut self, _literal: &AST::ASTNode) {}

    fn visit_identifier(&mut self, _identifier: &AST::Identifier) {}

    fn visit_int_constant(&mut self, _int_constant: &AST::IntConstant) {}

    fn visit_long_constant(&mut self, _long_constant: &AST::LongConstant) {}

    fn visit_string_constant(&mut self, _string_constant: &AST::StringConstant) {}

    fn visit_bool_constant(&mut self, _bool_constant: &AST::BoolConstant) {}

    fn visit_char_constant(&mut self, _char_constant: &AST::CharConstant) {}   
}

pub fn compile_three_address(ast: AST::Program) -> ThreeAddressCode {
    let mut tac = ThreeAddressCode {
        var_entries: Vec::new(),
        global_instructions: Vec::new(),
        func_instructions: HashMap::new(),
        scopes: Vec::new(),
        is_global: false,
    };

    tac.visit_program(&ast);
    return tac;
}