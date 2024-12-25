

pub enum ASTNode {
    Program(Box<Program>),
    FieldDecl(Box<FieldDecl>),
    MethodDecl(Box<MethodDecl>),
    Block(Box<Block>),
    VarDecl(Box<VarDecl>),
    MethodArgDecl(Box<MethodArgDecl>),
    AssignStatement(Box<AssignStatement>),
    IfStatement(Box<IfStatement>),
    ForStatement(Box<ForStatement>),
    WhileStatement(Box<WhileStatement>),
    ReturnStatement(Box<ReturnStatement>),
    StatementControl(Box<StatementControl>),
    Assignment(Box<Assignment>),
    Increment(Box<Increment>),
    ForUpdate(Box<ForUpdate>),
    MethodCall(Box<MethodCall>),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    IndexExpression(Box<IndexExpression>),
    LengthExpression(Box<LengthExpression>),
    ArrayLiteral(Box<ArrayLiteral>),
    Identifier(Box<Identifier>),
    IntConstant(Box<IntConstant>),
    StringConstant(Box<StringConstant>),
    BoolConstant(Box<BoolConstant>),
    CharConstant(Box<CharConstant>),
}

// Top level declarations

pub struct Program {
    pub imports: Vec<Box<Identifier>>,
    pub fields: Vec<Box<FieldDecl>>,
    pub methods: Vec<Box<MethodDecl>>,
}

pub struct FieldDecl {
    pub is_const: bool,
    pub type_name: String,
    pub vars: Vec<Box<VarDecl>>,
}

pub struct MethodDecl{
    pub type_name: String,
    pub name: Identifier,
    pub args: Vec<Box<MethodArgDecl>>,
    pub body: Box<Block>,
}

pub struct Block {
    pub fields: Vec<Box<FieldDecl>>,
    pub statements: Vec<Box<ASTNode>>, // statements of type specified by the grammar   
}

// Declarations
pub struct VarDecl {
    pub name: Box<Identifier>,
    pub array_len: Box<Option<IntConstant>>,
    pub initializer: Box<Option<ASTNode>>, // either a literal or an array literal
}

pub struct MethodArgDecl {
    pub type_name: String,
    pub name: Box<Identifier>,
}

// Statements 
pub struct AssignStatement {
    pub location: Box<ASTNode>, // either an identifier or an index expression
    pub assign_expr: Box<ASTNode>, // either Assign or Increment terminal
}

pub struct IfStatement {
    pub condition: Box<ASTNode>, // any expression type specified by the grammar
    pub then_block: Box<Block>,
    pub else_block: Box<Block>,
}

pub struct ForStatement {
    pub loop_expr: Box<Identifier>,
    pub start_expr: Box<ASTNode>,
    pub end_expr: Box<ASTNode>,
    pub update_expr: Box<ASTNode>, // either ForUpdate or MethodCall
    pub block: Box<Block>,
}

pub struct WhileStatement {
    pub condition: Box<ASTNode>, // any expression type specified by the grammar
    pub block: Box<Block>,
}

pub struct ReturnStatement {
    pub expr: Box<ASTNode>, // any expression type specified by the grammar
}

pub struct StatementControl {
    pub op: String, // either Break or Continue
}

// Assignments
pub struct Assignment {
    pub assign_op: String, 
    pub expr: Box<ASTNode>, // any expression type specified by the grammar
}

pub struct Increment {
    pub increment_op: String,
}

pub struct ForUpdate {
    pub location: Box<ASTNode>, // either an identifier or an index expression
    pub assign_expr: Box<ASTNode>, // either Assign or Increment terminal
}

// Expressions

pub struct MethodCall {
    pub name: Box<Identifier>,
    pub args: Vec<Box<ASTNode>>,
}

pub struct UnaryExpression {
    pub op: String,
    pub expr: Box<ASTNode>,
}

pub struct BinaryExpression {
    pub op: String,
    pub left_expr: Box<ASTNode>,
    pub right_expr: Box<ASTNode>
}

/*
For location case <id> '[' <expr> ']'
*/
pub struct IndexExpression {
    pub id: Box<Identifier>, 
    pub idx_expr: Box<ASTNode>,
}

pub struct LengthExpression {
    pub id: Box<Identifier>,
}

// Base Constants and Identifiers
pub struct ArrayLiteral {
    pub array_values: Vec<Box<ASTNode>>,
}

pub struct Identifier {
    pub name: String, 
}

/*
Stores both decimal and hex numbers
*/
pub struct IntConstant {
    pub value: i64,
}

impl IntConstant {
    pub fn new(value: &str) -> IntConstant {
        let mut sign = 1;
        if value.starts_with("-") {
            sign = -1;
        }
        let unsigned_val = &value[1..value.len()];
        if unsigned_val.starts_with("0x") {
            IntConstant {
                value: sign * i64::from_str_radix(&unsigned_val[2..unsigned_val.len()], 16).unwrap(),
            }
        } else {
            IntConstant {
                value: sign * unsigned_val.parse::<i64>().unwrap(),
            }
        }
    }
}

pub struct StringConstant {
    pub value: String,
}

pub struct BoolConstant {
    pub value: bool,
}

pub struct CharConstant {
    pub value: char,
}

