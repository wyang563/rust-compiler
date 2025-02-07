use super::super::parser::AST;

// blocks in CFG
#[derive(Debug)]
pub enum Block {
    Basic(BasicBlock),
    Condition(ConditionBlock),
    NoOp(NoOp),
    Decl(DeclBlock),
}

impl Block {
    pub fn get_next_block(&self) -> Option<usize> {
        match self {
            Block::Basic(block) => block.next_block,
            Block::Condition(block) => block.true_block,
            Block::NoOp(block) => block.next_block,
            Block::Decl(block) => block.next_block,
        }
    }

    pub fn set_next_block(&mut self, ind: usize) {
        match self {
            Block::Basic(block) => block.next_block = Some(ind),
            Block::Condition(_) => (),
            Block::NoOp(block) => block.next_block = Some(ind),
            Block::Decl(block) => block.next_block = Some(ind),
        }
    }

    pub fn set_branch_block(&mut self, ind: usize, branch_type: bool) {
        match self {
            Block::Condition(block) => {
                if branch_type {
                    block.true_block = Some(ind);
                } else {
                    block.false_block = Some(ind);
                }
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct BasicBlock {
    pub statements: Vec<Box<AST::ASTNode>>,
    pub next_block: Option<usize>,
}

#[derive(Debug)]
pub struct ConditionBlock {
    pub cond_expr: Box<AST::ASTNode>,
    pub true_block: Option<usize>,
    pub false_block: Option<usize>,
}

#[derive(Debug)]
pub struct NoOp {
    pub next_block: Option<usize>,
}

#[derive(Debug)]
pub struct DeclBlock {
    pub decls: Vec<Box<AST::FieldDecl>>,
    pub next_block: Option<usize>,
}
