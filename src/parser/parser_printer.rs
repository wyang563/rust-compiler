use std::result;

use super::AST::{self};
use super::visitor::Visitor;

pub struct ParserPrinter {
    indent: usize,
    new_line: bool,
    return_string: String,
}

impl ParserPrinter {
    fn tab_incr(&mut self, add_str: &str) -> String {
        let mut final_str = String::new();
        let mut tab = String::new();
        if self.new_line {
            for _ in 0..self.indent {
                tab.push_str("  ");
            }    
        }
        final_str += &tab;
        final_str.push_str(add_str);
        if add_str.ends_with('\n') {
            self.new_line = true;
        } else {
            self.new_line = false;
        }
        return final_str;
    }
}

impl Visitor for ParserPrinter {
    fn visit_program(&mut self, program: &AST::Program) {
        let mut result = self.tab_incr("");
        for import_decl in &program.imports {
            import_decl.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        for field_decl in &program.fields {
            field_decl.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        for method_decl in &program.methods {
            method_decl.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        self.return_string = result;
    }

    fn visit_import_decl(&mut self, import_decl: &AST::ImportDecl) {
        let mut result = self.tab_incr("import ");
        result += &self.tab_incr(import_decl.import_id.as_str());
        result += &self.tab_incr(";\n");
        self.return_string = result;
    }

    fn visit_field_decl(&mut self, field_decl: &AST::FieldDecl) {
        let mut result = self.tab_incr("");
        if field_decl.is_const {
            result += &self.tab_incr("const ");
        }
        result += &self.tab_incr(&field_decl.type_name);
        result += &self.tab_incr(" ");
        for (i, var_decl) in field_decl.vars.iter().enumerate() {
            var_decl.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
            if i < field_decl.vars.len() - 1 {
                result += &self.tab_incr(", ");
            }
        }
        result += &self.tab_incr(";\n");
        self.return_string = result;
    }

    fn visit_method_decl(&mut self, method_decl: &AST::MethodDecl) {
        let mut result = self.tab_incr(&method_decl.type_name);
        result += &self.tab_incr(" ");
        method_decl.name.accept(self);
        result += &self.tab_incr(" ");
        result += &self.tab_incr(&self.return_string.clone());
        for (i, arg) in method_decl.args.iter().enumerate() {
            arg.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
            if i < method_decl.args.len() - 1 {
                result += &self.tab_incr(", ");
            }
        }
        method_decl.body.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_block(&mut self, block: &AST::Block) {
        let mut result = self.tab_incr("{\n");
        self.indent += 1;
        for field_decl in &block.fields {
            field_decl.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        for statement in &block.statements {
            statement.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        self.indent -= 1;
        result += &self.tab_incr("}\n");
        self.return_string = result;
    }

    fn visit_var_decl(&mut self, var_decl: &AST::VarDecl) {
        var_decl.name.accept(self);
        let mut result = self.tab_incr(&self.return_string.clone());
        if var_decl.is_array {
            result += &self.tab_incr("[");
            if let Some(array_len_val) = var_decl.array_len.as_ref() {
                array_len_val.accept(self);
                result += &self.tab_incr(&self.return_string.clone());
            }
            result += &self.tab_incr("]");
        }
        if let Some(init_node) = var_decl.initializer.as_ref() {
            result += &self.tab_incr(" = ");
            init_node.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        self.return_string = result;
    }

    fn visit_method_arg_decl(&mut self, method_arg_decl: &AST::MethodArgDecl) {
        let mut result = self.tab_incr(&method_arg_decl.type_name);
        result += &self.tab_incr(" ");
        method_arg_decl.name.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement) {
        let mut result = self.tab_incr("if (");
        if_statement.condition.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(") ");
        if_statement.then_block.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        if let Some(else_block) = if_statement.else_block.as_ref() {
            result += &self.tab_incr("else ");
            else_block.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
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



