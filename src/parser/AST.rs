use super::visitor::Visitor;

pub enum ASTNode {
    Program(Program),
    ImportDecl(ImportDecl),
    FieldDecl(FieldDecl),
    MethodDecl(MethodDecl),
    Block(Block),
    VarDecl(VarDecl),
    MethodArgDecl(MethodArgDecl),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
    StatementControl(StatementControl),
    Assignment(Assignment),
    MethodCall(MethodCall),
    LenCall(LenCall),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    IndexExpression(IndexExpression),
    ArrayLiteral(ArrayLiteral),
    Identifier(Identifier),
    IntConstant(IntConstant),
    StringConstant(StringConstant),
    BoolConstant(BoolConstant),
    CharConstant(CharConstant),
}

impl ASTNode {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_ast_node(self);
    }
}

// Top level declarations

pub struct Program {
    pub imports: Vec<Box<Identifier>>,
    pub fields: Vec<Box<FieldDecl>>,
    pub methods: Vec<Box<MethodDecl>>,
}

impl Program {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}

pub struct ImportDecl {
    pub import_id: String,
}

impl ImportDecl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_import_decl(self);
    }
}

pub struct FieldDecl {
    pub is_const: bool,
    pub type_name: String,
    pub vars: Vec<Box<VarDecl>>,
}

impl FieldDecl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_field_decl(self);
    }
}

pub struct MethodDecl{
    pub type_name: String,
    pub name: Identifier,
    pub args: Vec<Box<MethodArgDecl>>,
    pub body: Box<Block>,
}

impl MethodDecl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_method_decl(self);
    }
}

pub struct Block {
    pub fields: Vec<Box<FieldDecl>>,
    pub statements: Vec<Box<ASTNode>>, // statements of type specified by the grammar   
}

impl Block {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_block(self);
    }
}

// Declarations
pub struct VarDecl {
    pub name: Box<Identifier>,
    pub is_array: bool,
    pub array_len: Box<Option<IntConstant>>,
    pub initializer: Box<Option<ASTNode>>, // either a literal or an array literal
}

impl VarDecl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_var_decl(self);
    }
}

pub struct MethodArgDecl {
    pub type_name: String,
    pub name: Box<Identifier>,
}

impl MethodArgDecl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_method_arg_decl(self);
    }
}

// Statements 
pub struct IfStatement {
    pub condition: Box<ASTNode>, // any expression type specified by the grammar
    pub then_block: Box<Block>,
    pub else_block: Box<Option<Block>>,
}

impl IfStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_if_statement(self);
    }
}

pub struct ForStatement {
    pub increment_var: Box<Identifier>,
    pub start_expr: Box<ASTNode>,
    pub end_expr: Box<ASTNode>,
    pub update_expr: Box<ASTNode>, // either ForUpdate or MethodCall
    pub block: Box<Block>,
}

impl ForStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_for_statement(self);
    }
}

pub struct WhileStatement {
    pub condition: Box<ASTNode>, // any expression type specified by the grammar
    pub block: Box<Block>,
}

impl WhileStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_while_statement(self);
    }
}

pub struct ReturnStatement {
    pub expr: Box<Option<ASTNode>>, // any expression type specified by the grammar
}

impl ReturnStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_return_statement(self);
    }
}

pub struct StatementControl {
    pub op: String, // either Break or Continue
}

impl StatementControl {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_statement_control(self);
    }
}

// Assignments
pub struct Assignment {
    pub assign_var: Box<ASTNode>, // either an identifier or an index expression
    pub assign_op: String, 
    pub expr: Box<Option<ASTNode>>, // any expression type specified by the grammar
}

impl Assignment {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_assignment(self);
    }
}

// Expressions
pub struct MethodCall {
    pub name: Box<Identifier>,
    pub args: Vec<Box<ASTNode>>,
}

impl MethodCall {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_method_call(self);
    }
}

pub struct LenCall {
    pub id: Box<Identifier>,
}

impl LenCall {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_len_call(self);
    }
}

pub struct UnaryExpression {
    pub op: String,
    pub expr: Box<ASTNode>,
}

impl UnaryExpression {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_unary_expression(self);
    }
}

pub struct BinaryExpression {
    pub op: String,
    pub left_expr: Box<ASTNode>,
    pub right_expr: Box<ASTNode>
}

impl BinaryExpression {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_binary_expression(self);
    }
}

/*
For location case <id> '[' <expr> ']'
*/
pub struct IndexExpression {
    pub id: Box<Identifier>, 
    pub idx_expr: Box<ASTNode>,
}

impl IndexExpression {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_index_expression(self);
    }
}

// Base Constants and Identifiers
pub struct ArrayLiteral {
    pub array_values: Vec<Box<ASTNode>>,
}

impl ArrayLiteral {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_array_literal(self);
    }
}

pub struct Identifier {
    pub name: String, 
}

impl Identifier {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_identifier(self);
    }
}

/*
Stores both decimal and hex numbers
*/
pub struct IntConstant {
    pub is_neg: bool,
    pub value: String,
}

impl IntConstant {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_int_constant(self);
    }
}

pub struct StringConstant {
    pub value: String,
}

impl StringConstant {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_string_constant(self);
    }
}

pub struct BoolConstant {
    pub value: bool,
}

impl BoolConstant {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_bool_constant(self);
    }
}

pub struct CharConstant {
    pub value: String,
}

impl CharConstant {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_char_constant(self);
    }
}

