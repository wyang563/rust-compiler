
pub enum InstructionType {
    /*
    Performs arithmetic 
    target: t (identifier or index expression)
    var1: v1 (identifier, index expression, or constant) 
    var2: v2 (identifier, index expression, or constant)
    op: arithmetic operation
    */
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    /*
    Performs binary logical ops 
    target: t (identifier or index expression)
    var1: v1 (identifier, index expression, or constant)
    var2: v2 (identifier, index expression, or constant)
    op: binary logical operation
     */
    Eq,
    And,
    Or,
    Gt,
    Geq,
    
    /*
    Performs unary logical ops 
    target: t (identifier or index expression)
    var1: v1 (identifier, index expression, or constant)
    op: unary logical operation
     */
    Not,
    Neg,

    /*
    Performs type casting
    target: t (identifier or index expression)
    var1: v1 (identifier, index expression, or constant) 
    op: cast operation
     */
    IntCast,
    LongCast,

    /*
    Loads value into variable
    target: t (identifier or index expression)
    var1: v (constant or variable value)
     */
    Load,     
    Move,

    /*
    Stores variable value onto operand stack
    store_var: s (variable or constant to store into)
     */
    Store,

    /*
    func_var: f (function we are calling) 
    p: param count (constant integer)
     */
    Call,
    Return,
    Goto, // go to specific tag in code
}

pub struct Instruction {

}