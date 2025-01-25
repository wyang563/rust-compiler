use super::super::parser::AST;
use super::super::parser::visitor::Visitor;

// blocks in CFG
pub enum Block {
    Basic(BasicBlock),
    Condition(ConditionBlock),
    NoOp(NoOp),
    Decl(DeclBlock),
    MethodDecl(MethodDeclBlock),
}

pub struct BasicBlock {
    pub statements: Vec<Box<AST::ASTNode>>,
    pub next_block: Box<Block>,
}

pub struct ConditionBlock {
    pub cond_expr: Box<AST::ASTNode>,
    pub true_block: Box<Block>,
    pub false_block: Box<Block>,
}

pub struct NoOp {

}

pub struct DeclBlock {
    
}

pub struct MethodDeclBlock {

}

pub struct ControlFlowGraph {
    pub start_block: Block,
    pub end_block: Block,

}

impl ControlFlowGraph {
    pub fn construct_cfg(ast: AST::Program) {

    }
}

