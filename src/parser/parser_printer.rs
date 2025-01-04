use std::result;

use super::AST::{self};
use super::visitor::Visitor;

pub struct ParserPrinter {
    indent: usize,
    new_line: bool,
    in_for_loop_def: bool,
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
        self.return_string = result;
    }

    fn visit_for_statement(&mut self, for_statement: &AST::ForStatement) {
        self.in_for_loop_def = true;
        let mut result = self.tab_incr("for (");
        for_statement.increment_var.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(" = ");
        for_statement.start_expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr("; ");
        for_statement.end_expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr("; ");
        for_statement.update_expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(") ");
        self.in_for_loop_def = false;
        for_statement.block.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_while_statement(&mut self, while_statement: &AST::WhileStatement) {
        let mut result = self.tab_incr("while (");
        while_statement.condition.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(") ");
        while_statement.block.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_return_statement(&mut self, return_statement: &AST::ReturnStatement) {
        let mut result = self.tab_incr("return");
        if let Some(expr) = return_statement.expr.as_ref() {
            result += &self.tab_incr(" ");
            expr.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        result += &self.tab_incr(";\n");
        self.return_string = result;
    }

    fn visit_statement_control(&mut self, statement_control: &AST::StatementControl) {
        let mut result = self.tab_incr(&statement_control.op);
        result += &self.tab_incr(";\n");
        self.return_string = result;
    }

    fn visit_assignment(&mut self, assignment: &AST::Assignment) {
        assignment.assign_var.accept(self);
        let mut result = self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(" ");
        result += &self.tab_incr(&assignment.assign_op);
        if let Some(expr) = assignment.expr.as_ref() {
            result += &self.tab_incr(" ");
            expr.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
        }
        if !self.in_for_loop_def {
            result += &self.tab_incr(";\n");
        }
        self.return_string = result;
    }

    fn visit_method_call(&mut self, method_call: &AST::MethodCall) {
        method_call.name.accept(self);
        let mut result = self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr("(");
        for (i, arg) in method_call.args.iter().enumerate() {
            arg.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
            if i < method_call.args.len() - 1 {
                result += &self.tab_incr(", ");
            }
        }
        result += &self.tab_incr(")");
        self.return_string = result;
    }

    fn visit_len_call(&mut self, len_call: &AST::LenCall) {
        let mut result = self.tab_incr("len(");
        len_call.id.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(")");
        self.return_string = result;
    }

    fn visit_unary_expression(&mut self, unary_expression: &AST::UnaryExpression) {
        let mut result = self.tab_incr(&unary_expression.op);
        unary_expression.expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_binary_expression(&mut self, binary_expression: &AST::BinaryExpression) {
        binary_expression.left_expr.accept(self);
        let mut result = self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr(" ");
        result += &self.tab_incr(&binary_expression.op);
        result += &self.tab_incr(" ");
        binary_expression.right_expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        self.return_string = result;
    }

    fn visit_index_expression(&mut self, index_expression: &AST::IndexExpression) {
        index_expression.id.accept(self);
        let mut result = self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr("[");
        index_expression.idx_expr.accept(self);
        result += &self.tab_incr(&self.return_string.clone());
        result += &self.tab_incr("]");
        self.return_string = result;
    }

    fn visit_array_literal(&mut self, array_literal: &AST::ArrayLiteral) {
        let mut result = self.tab_incr("{");
        for (i, val) in array_literal.array_values.iter().enumerate() {
            val.accept(self);
            result += &self.tab_incr(&self.return_string.clone());
            if i < array_literal.array_values.len() - 1 {
                result += &self.tab_incr(", ");
            }
        }
        result += &self.tab_incr("}");
        self.return_string = result;
    }

    fn visit_identifier(&mut self, identifier: &AST::Identifier) {
        self.return_string = identifier.name.clone();
    }

    fn visit_int_constant(&mut self, int_constant: &AST::IntConstant) {
        let mut result = self.tab_incr("");
        if int_constant.is_neg {
            result += &self.tab_incr("-");
        }
        result += &self.tab_incr(&int_constant.value.to_string());
        self.return_string = result;
    }

    fn visit_string_constant(&mut self, string_constant: &AST::StringConstant) {
        self.return_string = string_constant.value.clone();
    }

    fn visit_bool_constant(&mut self, bool_constant: &AST::BoolConstant) {
        self.return_string = bool_constant.value.to_string();
    }

    fn visit_char_constant(&mut self, char_constant: &AST::CharConstant) {
        self.return_string = char_constant.value.clone();
    }
}



