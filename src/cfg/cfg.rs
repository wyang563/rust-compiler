use super::super::parser::AST::{self};

pub struct BasicBlock {
    pub id: usize,
    pub statement: Vec<Box<AST::Statement>>,
    pub successors: Vec<Box<BasicBlock>>,
}

pub struct ControlFlowGraph {
    pub blocks: Vec<Box<BasicBlock>>,
}

impl ControlFlowGraph {
    pub fn constructCfg(ast: AST::Program) {

    }
}

