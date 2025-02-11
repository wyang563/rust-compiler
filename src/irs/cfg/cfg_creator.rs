use super::super::super::parser::AST;
use super::super::super::parser::visitor::Visitor;
use super::cfg_blocks::{BasicBlock, Block, ConditionBlock, DeclBlock, NoOp};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProgramGraph {
    pub import_decl: Vec<Box<AST::ImportDecl>>,
    pub global_field_decls: Vec<Box<AST::FieldDecl>>,
    pub method_graphs: HashMap<String, Box<ControlFlowGraph>>,
}

#[derive(Debug)]
pub struct ControlFlowGraph {
    pub nodes: Vec<Block>,
    pub start_block: usize,
    pub end_block: usize, // this varies by true and false branch end block

    // flags
    pub prev_loop_blocks: Vec<usize>, // stack of loop blocks
}

impl ControlFlowGraph {
}

impl Visitor for ControlFlowGraph { 
    fn visit_method_decl(&mut self, method_decl: &AST::MethodDecl) {
        self.visit_block(&method_decl.body);
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
                    self.visit_if_statement(if_statement);
                    block_statements = vec![];
                },
                AST::ASTNode::WhileStatement(while_statement) => {
                    self.add_block(Block::Basic( BasicBlock {
                        statements: block_statements.clone(),
                        next_block: None,
                    }));
                    self.visit_while_statement(while_statement);
                    block_statements = vec![];
                },
                AST::ASTNode::ForStatement(for_statement) => {
                    self.add_block(Block::Basic( BasicBlock {
                        statements: block_statements.clone(),
                        next_block: None,
                    }));
                    self.visit_for_statement(for_statement);
                    block_statements = vec![];
                },
                AST::ASTNode::StatementControl(statement_control) => self.visit_statement_control(statement_control),
                _ => {
                    block_statements.push(statement.clone());
                },
            }
        }
        self.add_block(Block::Basic( BasicBlock {
            statements: block_statements.clone(),
            next_block: None,
        }));
    }
    
    fn visit_if_statement(&mut self, if_statement: &AST::IfStatement) {
        self.visit_expression(if_statement.condition.as_ref());
        let cond_block_ind = self.nodes.len() - 1;
        let then_block_start_ind = self.nodes.len(); 
        self.visit_block(if_statement.then_block.as_ref());
        let then_block_end_ind = self.nodes.len() - 1; 
        self.nodes[cond_block_ind].set_branch_block(then_block_start_ind, true);        

        // set then branch pointers
        self.nodes[cond_block_ind].set_branch_block(then_block_start_ind, true);

        let mut else_block_end_ind: usize = 0;
        if let Some(else_block) = if_statement.else_block.as_ref() {
            let else_block_start_ind = self.nodes.len();
            self.visit_block(else_block);
            else_block_end_ind = self.nodes.len() - 1;
            self.nodes[cond_block_ind].set_branch_block(else_block_start_ind, false);
        }
        // add merge block for true/false branch
        self.add_block(Block::NoOp(NoOp {
            next_block: None,
        }));
        
        let end_ind = self.nodes.len() - 1;
        self.nodes[then_block_end_ind].set_next_block(end_ind);
        if if_statement.else_block.is_some() {
            self.nodes[else_block_end_ind].set_next_block(end_ind);
        } 
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
            // AST::ASTNode::BinaryExpression(binary_expression) => {
            //     self.visit_binary_expression(binary_expression);
            // },
            _ => {
                // create condition expression
                let condition_block = ConditionBlock {
                    cond_expr: Box::new(expression.clone()),
                    true_block: None,
                    false_block: None,
                };
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
}

pub fn construct_program_graph(ast: AST::Program) -> ProgramGraph {
    let mut program_graph = ProgramGraph {
        import_decl: vec![],
        global_field_decls: vec![],
        method_graphs: HashMap::new(),
    };
    program_graph.import_decl = ast.imports.clone();
    program_graph.global_field_decls = ast.fields.clone();

    // create cfgs for all methods
    for method_decl in ast.methods {
        let method_name = method_decl.name.name.clone();
        let mut cfg = ControlFlowGraph {
            nodes: vec![],
            start_block: 0,
            end_block: 0,
            prev_loop_blocks: vec![],
        };
        cfg.visit_method_decl(method_decl.as_ref());
        program_graph.method_graphs.insert(method_name, Box::new(cfg));
    }
    return program_graph;
}