use kala_repr::Slot;

use super::instruction::{InstructionImmediate, InstructionUpdate, InstructionJump, InstructionBranch, InstructionLoad, InstructionStore, InstructionRegister, Instruction, Opcode};

pub struct Interpreter {
    stack: Vec<Slot>,

    ra: usize, // return address after return
    fp: usize, // current frame base
    sp: usize, // current stack top
    pc: usize, // program counter
}

impl Interpreter {
    pub fn read_register(&self, register: u8) -> Slot {
        self.stack[self.fp + register as usize]
    }

    pub fn write_register(&mut self, register: u8, value: Slot) {
        self.stack[self.fp + register as usize] = value;
    }

    pub fn op(&mut self, op: Instruction) {
        match op.untyped.operation {
            Opcode::AddI => self.op_addi(op.immediate),
            Opcode::MulI => self.op_muli(op.immediate),
            Opcode::ModI => self.op_modi(op.immediate),
            Opcode::SltI => self.op_slti(op.immediate),
            Opcode::SltIU => self.op_sltiu(op.immediate),
            Opcode::AndI => self.op_andi(op.immediate),
            Opcode::OrI => self.op_ori(op.immediate),
            Opcode::XorI => self.op_xori(op.immediate),
            Opcode::SllI => self.op_slli(op.immediate),
            Opcode::SrlI => self.op_srli(op.immediate),
            Opcode::SraI => self.op_srai(op.immediate),
            Opcode::LuI => self.op_lui(op.immediate),

            Opcode::Add => self.op_add(op.register),
            Opcode::Sub => self.op_sub(op.register),
            Opcode::Mul => self.op_mul(op.register),
            Opcode::Div => self.op_div(op.register),
            Opcode::Mod => self.op_mod(op.register),
            Opcode::Pow => self.op_pow(op.register),
            Opcode::Slt => self.op_slt(op.register),
            Opcode::SltU => self.op_sltu(op.register),
            Opcode::And => self.op_and(op.register),
            Opcode::Or => self.op_or(op.register),
            Opcode::Xor => self.op_xor(op.register),
            Opcode::Sll => self.op_sll(op.register),
            Opcode::Srl => self.op_srl(op.register),
            Opcode::Sra => self.op_sra(op.register),

            Opcode::Jal => self.op_jal(op.jump),
            Opcode::Jalr => self.op_jalr(op.immediate),
            Opcode::Beq => self.op_beq(op.branch),
            Opcode::Bne => self.op_bne(op.branch),
            Opcode::Blt => self.op_blt(op.branch),
            Opcode::Bge => self.op_bge(op.branch),
            Opcode::Throw => self.op_throw(op),
            Opcode::Catch => self.op_catch(op),

            Opcode::Load => self.op_load(op.load),
            Opcode::Store => self.op_store(op.store),
            Opcode::AllocI => self.op_alloci(op.immediate),
            Opcode::Alloc => self.op_alloc(op.register),

            Opcode::CallI => self.op_calli(op.immediate),
            Opcode::Call => self.op_call(op.register),
            Opcode::Return => self.op_return(op.immediate),
            Opcode::Unwind => self.op_unwind(op.immediate),


            Opcode::AwaitCall => unimplemented!("AwaitCall"),
            Opcode::AwaitCallI => unimplemented!("AwaitCallI"),
            Opcode::Eval => unimplemented!("Eval"),


            _ => unimplemented!("Opcode {:?}", op.untyped.operation),
        }
    }

    fn op_immediate(&mut self, op: InstructionImmediate, f: fn(Slot, Slot) -> Slot) {
        let a = self.read_register(op.source_0);
        let b = Slot::from_existing_type(a, op.immediate);
        let c = f(a, b);
        self.write_register(op.destination, c);
    }

    pub fn op_addi(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::add);
    }

    pub fn op_muli(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::mul);
    }

    pub fn op_modi(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::rem);
    }

    pub fn op_slti(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::slt);
    }

    pub fn op_sltiu(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::sltu);   
    }

    pub fn op_andi(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::bit_and);
    }

    pub fn op_ori(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::bit_or);
    }

    pub fn op_xori(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::bit_xor);
    }

    pub fn op_slli(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::shl);
    }

    pub fn op_srli(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::shr);
    }

    pub fn op_srai(&mut self, op: InstructionImmediate) {
        self.op_immediate(op, Slot::sar);
    }

    pub fn op_lui(&mut self, op: InstructionImmediate) {
    
    }

    fn op_register(&mut self, op: InstructionRegister, f: fn(Slot, Slot) -> Slot) {
        let a = self.read_register(op.source_0);
        let b = self.read_register(op.source_1);
        let c = f(a, b);
        self.write_register(op.destination, c);
    }

    pub fn op_add(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::add);
    }

    pub fn op_mul(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::mul);
    }

    pub fn op_mod(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::rem);
    }

    pub fn op_slt(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::slt);
    }

    pub fn op_sltu(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::sltu);
    }

    pub fn op_and(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::bit_and);
    }

    pub fn op_or(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::bit_or);
    }

    pub fn op_xor(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::bit_xor);
    }

    pub fn op_sll(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::shl);
    }

    pub fn op_srl(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::shr);
    }

    pub fn op_sra(&mut self, op: InstructionRegister) {
        self.op_register(op, Slot::sar);
    }

    pub fn op_jal(&mut self, op: InstructionJump) {
        let jmp = op.immediate as isize * 4;
        self.write_register(op.destination, self.pc + 4);
        self.pc += jmp;
    }

    pub fn op_jalr(&mut self, op: InstructionImmediate) {
        let jmp = (self.read_register(op.source_0) + op.immediate) * 4;
        self.write_register(op.destination, self.pc + 4);
        self.pc += jmp;
    }

    pub fn op_beq(&mut self, op: InstructionBranch) {
        let a = self.read_register(op.source_0);
        let b = self.read_register(op.source_1);
        // XXX: since we can have multiple representation for a single value because of
        // - inlined value
        // - multiple levels of indirection when stored in the local variable
        // so having simple equality check is not enough
        // we may need to put type check in conditional branches too(right now the hints are not used in conditional branches)
        if a == b {
            let jmp = op.immediate as isize * 4;
            self.pc += jmp;
        }
    }

    pub fn op_bne(&mut self, op: InstructionBranch) {
        let a = self.read_register(op.source_0);
        let b = self.read_register(op.source_1);
        if a != b {
            let jmp = op.immediate as isize * 4;
            self.pc += jmp;
        }
    }

    pub fn op_blt(&mut self, op: InstructionBranch) {
        let a = self.read_register(op.source_0);
        let b = self.read_register(op.source_1);
        if a < b {
            let jmp = op.immediate as isize * 4;
            self.pc += jmp;
        }
    }

    pub fn op_bge(&mut self, op: InstructionBranch) {
        let a = self.read_register(op.source_0);
        let b = self.read_register(op.source_1);
        if a >= b {
            let jmp = op.immediate as isize * 4;
            self.pc += jmp;
        }
    }

    pub fn op_load(&mut self, op: InstructionImmediate) {
        let width = op.operation.memory_width();
        let base = self.read_register(op.base);
        let offset = op.offset as isize;
        let address = base + offset;
        let value = self.read_memory(address, width);
        self.write_register(op.destination, value);
    }

    pub fn op_store(&mut self, op: InstructionImmediate) {
        let width = op.operation.memory_width();
        let base = self.read_register(op.base);
        let offset = op.offset as isize;
        let address = base + offset;
        let value = self.read_register(op.source);
        self.write_memory(address, width, value);
    }

    pub fn op_alloc(&mut self, op: InstructionRegister) {
        let width = op.operation.memory_width();
        let base = self.read_register(op.destination);
        let offset = op.immediate as isize;
        let address = base + offset;
        self.write_register(op.destination, address);
        self.write_memory(address, width, Slot::Undefined);
    }

    pub fn op_alloci(&mut self, op: InstructionImmediate) {
        let width = op.operation.memory_width();
        let base = self.read_register(op.destination);
        let offset = op.immediate as isize;
        let address = base + offset;
        self.write_register(op.destination, address);
        self.write_memory(address, width, Slot::Undefined);
    }

    pub fn op_calli(&mut self, op: InstructionImmediate) {
        let arg_start = self.fp + op.source_0 as usize;
        let arg_count = op.source_1 as usize;
        let arg_end = self_start + arg_count;
        let args = self.stack[arg_start..arg_end];
        self.push_call_data(args);
        self.jump(op.immediate);
    }

    pub fn op_call(&mut self, op: InstructionRegister) {
        let arg_start = self.fp + op.source_0 as usize;
        let arg_count = op.source_1 as usize;
        let arg_end = self_start + arg_count;
        let args = self.stack[arg_start..arg_end];
        self.push_call_data_variadic(args);
        let closure = self.read_register(op.destination);
        self.jump_closure(closure);
    }

    pub fn op_return(&mut self, op: InstructionImmediate) {
        let return_start: self.fp + op.
    }
}