use super::super::parser::AST;

// blocks in CFG

pub enum Block {
    Basic(BasicBlock),
    Condition(ConditionBlock),
    NoOp(NoOp),
    Decl(DeclBlock),
    MethodDecl(MethodDeclBlock),
    ImportDecl(ImportDeclBlock),
}

pub enum LineType {
    None,
    True,
    False,
}

impl Block {
    pub fn get_next_block(&self) -> Option<usize> {
        match self {
            Block::Basic(block) => block.next_block,
            Block::Condition(block) => block.true_block,
            Block::NoOp(block) => block.next_block,
            Block::Decl(block) => block.next_block,
            Block::MethodDecl(block) => block.next_block,
            Block::ImportDecl(block) => block.next_block,
        }
    }

    pub fn set_next_block(&mut self, ind: usize) {
        match self {
            Block::Basic(block) => block.next_block = Some(ind),
            Block::Condition(_) => (),
            Block::NoOp(block) => block.next_block = Some(ind),
            Block::Decl(block) => block.next_block = Some(ind),
            Block::MethodDecl(block) => block.next_block = Some(ind),
            Block::ImportDecl(block) => block.next_block = Some(ind),
        }
    }
}

pub struct BasicBlock {
    pub statements: Vec<Box<AST::ASTNode>>,
    pub next_block: Option<usize>,
}

pub struct ConditionBlock {
    pub cond_expr: Box<AST::ASTNode>,
    pub true_block: Option<usize>,
    pub false_block: Option<usize>,
}

pub struct NoOp {
    pub next_block: Option<usize>,
}

pub struct DeclBlock {
    pub decls: Vec<Box<AST::FieldDecl>>,
    pub next_block: Option<usize>,
}

pub struct MethodDeclBlock {
    pub method_decls: Vec<Box<AST::MethodDecl>>,
    pub next_block: Option<usize>,
}

pub struct ImportDeclBlock {
    pub import_decls: Vec<Box<AST::ImportDecl>>,
    pub next_block: Option<usize>,
}