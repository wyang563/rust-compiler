use super::super::parser::AST;
use super::super::parser::visitor::Visitor;
use super::cfg_blocks::{BasicBlock, Block, ConditionBlock, DeclBlock, ImportDeclBlock, MethodDeclBlock, NoOp, LineType};

pub struct ControlFlowGraph {
    pub nodes: Vec<Block>,
    pub start_block: usize,
    pub end_block: usize, // this varies by true and false branch end block

    // flags
    pub prev_cond_block: usize,
    pub prev_loop_block: usize, // for break/continue statements 
    pub line_type: LineType, // line type ie whether we're in a true or false branch, or none
}

impl ControlFlowGraph {
    
}

impl Visitor for ControlFlowGraph {
    fn visit_program(&mut self, program: &AST::Program) {
        // initial global scope decls        
        let global_import_decl_block = ImportDeclBlock {
            import_decls: program.imports.clone(),
            next_block: None,
        };
        let global_decl_block = DeclBlock {
            decls: program.fields.clone(),
            next_block: None, // placeholder no op value initially
        };
        let global_method_decl_block = MethodDeclBlock {
            method_decls: program.methods.clone(),
            next_block: None, // placeholder no op value initially
        };

        self.add_block(Block::ImportDecl(global_import_decl_block));
        self.add_block(Block::Decl(global_decl_block));
        self.add_block(Block::MethodDecl(global_method_decl_block));

        // start execution from main function
        for method_decl in &program.methods {
            if method_decl.name.name == "main" {
                method_decl.body.accept(self);
            }
        }
    }

    fn visit_block(&mut self, block: &AST::Block) {
        // field decls
        let method_field_decls = block.fields.clone();
        let method_field_block = DeclBlock {
            decls: method_field_decls,
            next_block: None, // placeholder no op value initially
        };
        self.add_block(Block::Decl(method_field_block));
        let mut block_statements: Vec<Box<AST::ASTNode>> = vec![];

        // TODO: this logic will be modified as we need to draw back edges to previous nodes
        for statement in &block.statements {
            match statement.as_ref() {
                AST::ASTNode::IfStatement(if_statement) => {
                    self.add_block(Block::Basic( BasicBlock {
                        statements: block_statements.clone(),
                        next_block: None,
                    }));
                    if_statement.accept(self);
                    block_statements = vec![];
                },
                AST::ASTNode::WhileStatement(while_statement) => {
                    self.add_block(Block::Basic( BasicBlock {
                        statements: block_statements.clone(),
                        next_block: None,
                    }));
                    while_statement.accept(self);
                    block_statements = vec![];
                },
                AST::ASTNode::ForStatement(for_statement) => {
                    self.add_block(Block::Basic( BasicBlock {
                        statements: block_statements.clone(),
                        next_block: None,
                    }));
                    for_statement.accept(self);
                    block_statements = vec![];
                },
                AST::ASTNode::StatementControl(statement_control) => statement_control.accept(self),
                _ => {
                    block_statements.push(statement.clone());
                },
            }
        }
    }
    
    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement) {
        // create condition block
        if_statement.condition.accept(self);
        
    }

    fn visit_while_statement(&mut self, while_statement: &AST::WhileStatement) {
        
    }

    fn visit_for_statement(&mut self, for_statement: &AST::ForStatement) {
        
    }

    fn visit_return_statement(&mut self, _return_statement: &AST::ReturnStatement) {
        
    }

    fn visit_statement_control(&mut self, _statement_control: &AST::StatementControl) {
        
    }

    fn visit_expression(&mut self, expression: &AST::ASTNode) {
        match expression {
            AST::ASTNode::BinaryExpression(binary_expression) => {
                binary_expression.accept(self);
            },
            _ => {
                // create condition expression
                let condition_block = ConditionBlock {
                    cond_expr: Box::new(expression.clone()),
                    true_block: None,
                    false_block: None,
                };
                self.prev_cond_block = self.nodes.len();
                self.add_block(Block::Condition(condition_block));
            },
        }
        
    }
}

impl ControlFlowGraph {
    // helpers
    fn add_block(&mut self, block: Block) {
        self.nodes.push(block);
        let latest_node_ind = self.nodes.len() - 1;
        if self.nodes.len() > 0 {
            self.nodes[self.end_block].set_next_block(latest_node_ind);
        }
        self.end_block = latest_node_ind;
    }

    pub fn construct_cfg(&mut self, ast: AST::Program) -> ControlFlowGraph {
        // starting control flow graph
        let mut cfg = ControlFlowGraph { // place holder blocks for initialization
            nodes: vec![],
            start_block: 0,
            end_block: 0,
            prev_cond_block: 0,
            prev_loop_block: 0,
            line_type: LineType::None,
        };
        ast.accept(&mut cfg);
        return cfg;
    }
}

