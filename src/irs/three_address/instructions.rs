
#[derive(Clone)]
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
    Load value (either from variable or constant) into variable v
    target: t
    var1: v or c (if loading a constant)
     */
    Move,

    /*
    Store value onto the operand stack (for function parameters)
    var: v or c (if storing a constant)
     */
    Push,

    /*
    Load value into array variable 
    target: t (target variable we're loading into)
    var1: v (array variable we're loading from)
    ind: i (index we're loading or storing at)
     */
    LoadArray,     
    
    /*
    Store value into array variable at a given index
    target: t (target array variable we're storing into) 
    var1: v (variable or constant we're storing)
    ind: i (index we're storing at)
     */
    StoreArray, 

    /*
    func_var: f (function we are calling) 
    p: param count (constant integer)
     */
    Call,
    Ret,
    Goto, // go to specific label in code

    // label representation
    Label,
}

pub enum Instruction {
    Binary(BinaryInstruction),
    Unary(UnaryInstruction),
    Push(PushInstruction),
    Flow(FlowInstruction),
    Array(ArrayInstruction),
    Call(Call),
    Ret(Ret),
}

pub trait InstructionTrait {
    fn get_type(&self) -> InstructionType;
}

/*
Add, Sub, Mul, Div, Mod, Eq, And, Or, Gt, Geq
*/
pub struct BinaryInstruction {
    target: String,
    var1: usize,
    var2: usize,
    instruction: InstructionType,
}

impl InstructionTrait for BinaryInstruction {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}

/*
Not, Neg, IntCast, LongCast, Move
*/
pub struct UnaryInstruction {
    target: usize,
    var: usize,
    instruction: InstructionType,
}

impl InstructionTrait for UnaryInstruction {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}

/*
Push
*/
pub struct PushInstruction {
    var: usize,
    instruction: InstructionType,
}

impl InstructionTrait for PushInstruction {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}

/*
Goto, Label
*/
pub struct FlowInstruction {
    var: String, // label we're going to or label we're representing 
    instruction: InstructionType,
}

/*
LoadArray, StoreArray
*/
pub struct ArrayInstruction {
    target: usize,
    var: usize,
    ind: usize,
    instruction: InstructionType,
}

impl InstructionTrait for ArrayInstruction {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}

pub struct Call {
    func_var: usize,
    p: usize,
    instruction: InstructionType,
}

impl InstructionTrait for Call {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}

pub struct Ret {
    instruction: InstructionType,
}

impl InstructionTrait for Ret {
    fn get_type(&self) -> InstructionType {
        self.instruction.clone()
    }
}