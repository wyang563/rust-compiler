use super::AST::{self};
use super::visitor::Visitor;

pub struct ParserPrinter {
    indent: usize,
}

impl Visitor for ParserPrinter {
    fn visit_program(&mut self, program: &AST::Program) {

    }

    fn visit_field_decl(&mut self, field_decl: &AST::FieldDecl) {

    }

    fn visit_method_decl(&mut self, method_decl: &AST::MethodDecl) {

    }

    fn visit_block(&mut self, block: &AST::Block) {

    }

    fn visit_var_decl(&mut self, var_decl: &AST::VarDecl) {

    }

    fn visit_method_arg_decl(&mut self, method_arg_decl: &AST::MethodArgDecl) {

    }

    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement) {

    }

    fn visit_for_statement(&mut self, for_statement: &AST::ForStatement) {

    }

    fn visit_while_statement(&mut self, while_statement: &AST::WhileStatement) {
        
    }

    fn visit_return_statement(&mut self, return_statement: &AST::ReturnStatement) {

    }

    fn visit_statement_control(&mut self, statement_control: &AST::StatementControl) {
        
    }

    fn visit_assignment(&mut self, assignment: &AST::Assignment) {
        
    }

    fn visit_method_call(&mut self, method_call: &AST::MethodCall) {
        
    }

    fn visit_len_call(&mut self, len_call: &AST::LenCall) {
        
    }

    fn visit_unary_expression(&mut self, unary_expression: &AST::UnaryExpression) {
        
    }

    fn visit_binary_expression(&mut self, binary_expression: &AST::BinaryExpression) {
        
    }

    fn visit_index_expression(&mut self, index_expression: &AST::IndexExpression) {
        
    }

    fn visit_array_literal(&mut self, array_literal: &AST::ArrayLiteral) {
        
    }

    fn visit_identifier(&mut self, identifier: &AST::Identifier) {
        
    }

    fn visit_int_constant(&mut self, int_constant: &AST::IntConstant) {
        
    }

    fn visit_string_constant(&mut self, string_constant: &AST::StringConstant) {
        
    }

    fn visit_bool_constant(&mut self, bool_constant: &AST::BoolConstant) {
        
    }

    fn visit_char_constant(&mut self, char_constant: &AST::CharConstant) {
        
    }
}



