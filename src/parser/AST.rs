

pub enum ASTNode {
    Program(Program),

}

// Top level declarations

pub struct Program {
    imports: Vec<String>,
    fields: Vec<FieldDecl>,
    methods: Vec<MethodDecl>,
}

pub struct FieldDecl {
    is_const: bool,
    type_name: String,
    vars: Vec<VarDecl>,
}

pub struct MethodDecl{
    type_name: String,
    name: Identifier,
    args: Vec<MethodArgDecl>,
    body: Block,
}

pub struct Block {
    fields: Vec<FieldDecl>,
    statements: Vec<ASTNode>, // statements of type specified by the grammar   
}

// Declarations
pub struct VarDecl {
    name: Identifier,
    array_len: IntConstant,
    initializer: ASTNode, // either a literal or an array literal
}

pub struct MethodArgDecl {
    type_name: String,
    name: Identifier,
}

// Statements 
pub struct AssignStatement {
    location: ASTNode, // either an identifier or an index expression
    assign_expr: ASTNode, // either Assign or Increment terminal
}

pub struct IfStatement {
    condition: ASTNode, // any expression type specified by the grammar
    then_block: Block,
    else_block: Block,
}

pub struct ForStatement {
    loop_expr: Identifier,
    start_expr: ASTNode,
    end_expr: ASTNode,
    update_expr: ASTNode, // either ForUpdate or MethodCall
    block: Block,
}

pub struct WhileStatement {
    condition: ASTNode, // any expression type specified by the grammar
    block: Block,
}

pub struct ReturnStatement {
    expr: ASTNode, // any expression type specified by the grammar
}

pub struct StatementControl {
    op: String, // either Break or Continue
}

// Assignments
pub struct Assignment {
    assign_op: String, 
    expr: ASTNode, // any expression type specified by the grammar
}

pub struct Increment {
    increment_op: String,
}

pub struct ForUpdate {
    location: ASTNode, // either an identifier or an index expression
    assign_expr: ASTNode, // either Assign or Increment terminal
}

// Expressions

pub struct MethodCall {
    name: Identifier,
    args: Vec<ASTNode>,
}

pub struct UnaryExpression {
    op: String,
    expr: ASTNode,
}

pub struct BinaryExpression {
    op: String,
    left_expr: ASTNode,
    right_expr: ASTNode
}

/*
For location case <id> '[' <expr> ']'
*/
pub struct IndexExpression {
    id: Identifier, 
    idx_expr: ASTNode,
}

pub struct LengthExpression {
    id: Identifier,
}

// Base Constants and Identifiers
pub struct ArrayLiteral {
    array_values: Vec<ASTNode>,
}

pub struct Identifier {
    name: String, 
}

/*
Stores both decimal and hex numbers
*/
pub struct IntConstant {
    value: i32,
}

pub struct StringConstant {
    value: String,
}

pub struct BoolConstant {
    value: bool,
}

pub struct CharConstant {
    value: char,
}

