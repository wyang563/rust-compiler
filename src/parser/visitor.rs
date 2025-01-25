use super::AST::{self};

pub trait Visitor {
    fn visit_program(&mut self, program: &AST::Program);
    fn visit_import_decl(&mut self, import_decl: &AST::ImportDecl);
    fn visit_field_decl(&mut self, field_decl: &AST::FieldDecl);
    fn visit_method_decl(&mut self, method_decl: &AST::MethodDecl);
    fn visit_block(&mut self, block: &AST::Block);
    fn visit_var_decl(&mut self, var_decl: &AST::VarDecl);
    fn visit_method_arg_decl(&mut self, method_arg_decl: &AST::MethodArgDecl);
    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement);
    fn visit_for_statement(&mut self, for_statement: &AST::ForStatement);
    fn visit_while_statement(&mut self, while_statement: &AST::WhileStatement);
    fn visit_return_statement(&mut self, return_statement: &AST::ReturnStatement);
    fn visit_statement_control(&mut self, statement_control: &AST::StatementControl);
    fn visit_assignment(&mut self, assignment: &AST::Assignment);
    fn visit_method_call(&mut self, method_call: &AST::MethodCall);
    fn visit_len_call(&mut self, len_call: &AST::LenCall);
    fn visit_unary_expression(&mut self, unary_expression: &AST::UnaryExpression);
    fn visit_binary_expression(&mut self, binary_expression: &AST::BinaryExpression);
    fn visit_index_expression(&mut self, index_expression: &AST::IndexExpression);
    fn visit_array_literal(&mut self, array_literal: &AST::ArrayLiteral);
    fn visit_identifier(&mut self, identifier: &AST::Identifier);
    fn visit_int_constant(&mut self, int_constant: &AST::IntConstant);
    fn visit_string_constant(&mut self, string_constant: &AST::StringConstant);
    fn visit_bool_constant(&mut self, bool_constant: &AST::BoolConstant);
    fn visit_char_constant(&mut self, char_constant: &AST::CharConstant);

    // optional methods
    fn visit_expression(&mut self, _expr: &AST::ASTNode) {}
    fn visit_location(&mut self, _location: &AST::ASTNode) {}
    fn visit_literal(&mut self, _literal: &AST::ASTNode) {}

    fn visit_ast_node(&mut self, ast_node: &AST::ASTNode) {
        match ast_node {
            AST::ASTNode::Program(program) => self.visit_program(program),
            AST::ASTNode::ImportDecl(import_decl) => self.visit_import_decl(import_decl),
            AST::ASTNode::FieldDecl(field_decl) => self.visit_field_decl(field_decl),
            AST::ASTNode::MethodDecl(method_decl) => self.visit_method_decl(method_decl),
            AST::ASTNode::Block(block) => self.visit_block(block),
            AST::ASTNode::VarDecl(var_decl) => self.visit_var_decl(var_decl),
            AST::ASTNode::MethodArgDecl(method_arg_decl) => self.visit_method_arg_decl(method_arg_decl),
            AST::ASTNode::IfStatement(if_statement) => self.visit_if_statement(if_statement),
            AST::ASTNode::ForStatement(for_statement) => self.visit_for_statement(for_statement),
            AST::ASTNode::WhileStatement(while_statement) => self.visit_while_statement(while_statement),
            AST::ASTNode::ReturnStatement(return_statement) => self.visit_return_statement(return_statement),
            AST::ASTNode::StatementControl(statement_control) => self.visit_statement_control(statement_control),
            AST::ASTNode::Assignment(assignment) => self.visit_assignment(assignment),
            AST::ASTNode::MethodCall(method_call) => self.visit_method_call(method_call),
            AST::ASTNode::LenCall(len_call) => self.visit_len_call(len_call),
            AST::ASTNode::UnaryExpression(unary_expression) => self.visit_unary_expression(unary_expression),
            AST::ASTNode::BinaryExpression(binary_expression) => self.visit_binary_expression(binary_expression),
            AST::ASTNode::IndexExpression(index_expression) => self.visit_index_expression(index_expression),
            AST::ASTNode::ArrayLiteral(array_literal) => self.visit_array_literal(array_literal),
            AST::ASTNode::Identifier(identifier) => self.visit_identifier(identifier),
            AST::ASTNode::IntConstant(int_constant) => self.visit_int_constant(int_constant),
            AST::ASTNode::StringConstant(string_constant) => self.visit_string_constant(string_constant),
            AST::ASTNode::BoolConstant(bool_constant) => self.visit_bool_constant(bool_constant),
            AST::ASTNode::CharConstant(char_constant) => self.visit_char_constant(char_constant),
        }
    }
}


