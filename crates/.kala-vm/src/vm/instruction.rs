#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub union Instruction {
    pub raw: u32,
    pub untyped: InstructionUntyped,
    pub immediate: InstructionImmediate,
    pub update: InstructionUpdate,
    pub jump: InstructionJump,
    pub branch: InstructionBranch,
    pub load: InstructionLoad,
    pub store: InstructionStore,
    pub register: InstructionRegister,
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Register(pub u8);

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionUntyped {
    pub operation: u8,
    _unknown_field_0: u8,
    _unknown_field_1: u8,
    _unknown_field_2: u8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionRegister {
    pub operation: Opcode,
    pub destination: Register,
    pub source_0: Register,
    pub source_1: Register,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionImmediate {
    pub operation: Opcode,
    pub destination: Register,
    pub source_0: Register,
    pub immediate: i8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionUpdate {
    pub operation: Opcode,
    pub destination: Register,
    pub immediate: [u8; 2],
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionJump {
    pub operation: Opcode,
    pub destination: Register,
    pub immediate: i16,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionBranch {
    pub operation: Opcode,
    pub source_0: Register,
    pub source_1: Register,
    pub immediate: i16,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionStore {
    pub operation: Opcode, // includes width information
    pub source: Register,
    pub base: Register,
    pub offset: i8,
}



#[repr(u8)]
pub enum Opcode {
    // For arithmetic opcodes, the high 2 bits are the type hint for the operands
    // 00 => Reference
    // 01 => Number
    // 10 => Constants
    // 11 => Bigint
    // But the interpreter itself should work agnostic of the semantic of the type tag, and it should not effect the behavior of the interpreter.

    AddI = 0b0000_0000,
    // SUBI,
    MulI = 0b0000_0010,
    // DIVI,
    ModI = 0b0000_0100,
    // POWI
    SltI = 0b0000_0110,
    SltIU = 0b0000_0111,
    AndI = 0b0000_1000,
    OrI = 0b0000_1001,
    XorI = 0b0000_1010,
    SllI = 0b0000_1011,
    SrlI = 0b0000_1100,
    SraI = 0b0000_1101,

    LuI = 0b0000_1111,

    Add = 0b0001_0000,
    Sub = 0b0001_0001,
    Mul = 0b0001_0010,
    Div = 0b0001_0011,
    Mod = 0b0001_0100,
    Pow = 0b0001_0101,
    Slt = 0b0001_0110,
    SltU = 0b0001_0111,
    And = 0b0001_1000,
    Or = 0b0001_1001,
    Xor = 0b0001_1010,
    Sll = 0b0001_1011,
    Srl = 0b0001_1100,
    Sra = 0b0001_1101,

    Jal = 0b0010_0000,
    Jalr = 0b0010_0001,
    Beq = 0b0010_0010,
    Bne = 0b0010_0011,
    Blt = 0b0010_0100,
    Bge = 0b0010_0101,
    Throw = 0b0010_0110,
    Catch = 0b0010_0111,

    // For load, store, and alloc, the high 2 bits are the width/alignment of the operation
    // 00 => 1 byte
    // 01 => 2 bytes
    // 10 => 4 bytes
    // 11 => 8 bytes

    LoadI = 0b0010_1000,
    StoreI = 0b0010_1001,
    AllocI = 0b0010_1010,
    // FREEI = 0b0011_1011,

    Load = 0b0010_1100,
    Store = 0b0010_1101,
    Alloc = 0b0010_1110,
    // FREE = 0b0011_1111,

    // JS specific opcodes

    GetPropertyI = 0b0011_0000, // known property
    SetPropertyI = 0b0011_0001, // known property
    CallI = 0b0011_0010, // known function
    AwaitCallI = 0b0011_0011, // known async function

    GetProperty = 0b0011_0100, // computed property
    SetProperty = 0b0011_0101, // computed property
    Call = 0b0011_0110, // closure function
    AwaitCall = 0b0011_0111, // closure async function
    
    Return = 0b0011_1000,
    Unwind = 0b0011_1001,
    //Eval = 

    CallI = 0b0011_1010, // saves program regieters, pushes the return address to the stack, and jumps to the function, caller side
    Call = 0b0011_1011,
    AwaitCallI = 0b0011_1100, // reserved
    AwaitCall = 0b0011_1101, // reserved
    Return = 0b0011_1100, // pops the return address from the stack, restores program registers, and jumps to the return address, callee side
    Unwind = 0b0011_1101, // pops the arguments from the stack, caller side


    Eval = 0b0011_0101, // reserved, probably will be not implemented
}

impl Opcode {
    pub fn type_hint(self) -> u8 {
        (self as u8) >> 6
    }

    pub fn memory_width(self) -> usize {
        1 << ((self as usize) >> 6)
    }
}