use std::mem::MaybeUninit;

use jessie_ast::*;
use utils::{FxMap, FxMapPool};

use crate::{vm::opcode::Opcode, slot::{Slot, SlotTag, number::{bound_i32, promote_i64, promote_i32}}, memory::memory::Memory, types::number::Number};

// Interpreter exeuctes a single script. 
pub struct Interpreter {
    stack: Vec<Slot>,
    code: Vec<Opcode>,
    pc: usize,
    fp: usize,

    memory: Memory,
    // map_pool: FxMapPool<Slot>, // meaningless for now as we don't deallocate
}

impl Interpreter {
    pub fn const_read_bytes_from_code<const count: usize>(&mut self) -> &[u8; count] {
        let offset = self.pc;
        self.pc += count;
        if self.pc >= self.code.len() {
            panic!("out of bounds");
        }
        unsafe{&*(self.code.as_ptr().add(offset) as *const [u8; count])}
    }

    pub fn read_bytes_from_code(&mut self, count: usize) -> &[u8] {
        let offset = self.pc;
        self.pc += count;
        if self.pc >= self.code.len() {
            panic!("out of bounds");
        }
        unsafe{std::mem::transmute(&self.code[offset..self.pc])}
    }

    pub fn const_read_opcodes_from_code<const count: usize>(&mut self) -> &[Opcode; count] {
        let offset = self.pc;
        self.pc += count;
        if self.pc >= self.code.len() {
            panic!("out of bounds");
        }
        unsafe{&*(self.code.as_ptr().add(offset) as *const [Opcode; count])}
    }

    pub fn read_opcodes_from_code(&mut self, count: usize) -> &[Opcode] {
        let offset = self.pc;
        self.pc += count;
        if self.pc >= self.code.len() {
            panic!("out of bounds");
        }
        unsafe{std::mem::transmute(&self.code[offset..self.pc])}
    }

    pub fn const_push_slots<const count: usize>(&mut self, slots: &[Slot; count]) {
        self.stack.extend_from_slice(slots);
    }

    // Should not push to the stack before consuming all the slots.
    pub fn const_pop_slots_from_stack<const count: usize>(&mut self) -> &[Slot; count] {
        if count > self.stack.len() {
            panic!("out of bounds");
        }
        let offset = self.stack.len() - count;
        unsafe{self.stack.set_len(offset)};
        unsafe{&*(self.stack.as_ptr().add(offset) as *const [Slot; count])}
    }

    pub fn pop_slots_from_stack(&mut self, count: usize) -> &[Slot] {
        if count > self.stack.len() {
            panic!("out of bounds");
        }
        let offset = self.stack.len() - count;
        unsafe{self.stack.set_len(offset)};
        unsafe{std::mem::transmute(&self.stack[offset..self.stack.len()])}
    }

    unsafe fn op_binary_number(&mut self, a: Slot, b: Slot, i32op: fn(i32, i32) -> i64, numop: fn(&Number, &Number) -> (Number, bool)) -> Slot {
        let a_ptr = a.get_number_pointer();
        let b_ptr = b.get_number_pointer();
        if a_ptr.is_null() && b_ptr.is_null() {
            let res = i32op(a.value, b.value);
            if bound_i32(res) {
                return Slot::new_i32(res as i32);
            }
            let res_ptr = self.memory.allocate_number(promote_i64(res));
            return Slot::new_number(res_ptr);
        }

        let a = if a_ptr.is_null() {
            promote_i32(a.value)
        } else {
            *a_ptr
        };

        let b = if b_ptr.is_null() {
            promote_i32(b.value)
        } else {
            *b_ptr
        };

        let (res, overflow) = numop(a, b);
        if !overflow {
            return Slot::new_number(self.memory.allocate_number(res));
        }

        if res.0 < 0 {
            // overflow
            unimplemented!("Infinity")
        } else {
            // underflow
            unimplemented!("-Infinity")
        }
    }

    unsafe fn op_binary_bigint<'a>(&mut self, a: Slot, b: Slot, i32op: fn(i32, i32) -> i64, numop: fn(i32, &'a [u64], i32, &'a [u64]) -> (i32, &'a [u64])) -> Slot {
        let a_ptr = a.get_bigint_pointer();
        let b_ptr = b.get_bigint_pointer();
        if a_ptr.is_null() && b_ptr.is_null() {
            let res = i32op(a.value, b.value);
            if bound_i32(res) {
                return Slot::new_b32(res as i32);
            }
            let res_ptr = self.memory.allocate_bigint(&[res.abs()]);
            return Slot::new_bigint(res.signum() as i32, res_ptr);
        }

        let a_abs = if a_ptr.is_null() {
            &[a.value.abs() as u64][..]
        } else {
            &*a_ptr
        };

        let b_abs = if b_ptr.is_null() {
            &[b.value.abs() as u64][..]
        } else {
            &*b_ptr
        };

        let (sign, abs) = numop(a.value.signum(), a_abs, b.value.signum(), b_abs);

        let ptr = self.memory.allocate_bigint(abs);
        Slot::new_bigint(sign * abs.len() as i32, ptr)
    }

    unsafe fn op_comp_number(&mut self, a: Slot, b: Slot, i32op: fn(i32, i32) -> bool, numop: fn(i128, i128) -> bool) -> Slot {
        let a_ptr = a.get_number_pointer();
        let b_ptr = b.get_number_pointer();
        if a_ptr.is_null() && b_ptr.is_null() {
            return Slot::new_bool(i32op(a.value, b.value));
        }

        let a = if a_ptr.is_null() {
            promote_i32(a.value)
        } else {
            *a_ptr
        };

        let b = if b_ptr.is_null() {
            promote_i32(b.value)
        } else {
            *b_ptr
        };

        Slot::new_bool(numop(a, b))
    }

    unsafe fn op_comp_bigint(&mut self, a: Slot, b: Slot, i32op: fn(i32, i32) -> bool, numop: fn(i32, &[u64], i32, &[u64]) -> bool) -> Slot {
        let a_ptr = a.get_bigint_pointer();
        let b_ptr = b.get_bigint_pointer();
        if a_ptr.is_null() && b_ptr.is_null() {
            return Slot::new_bool(i32op(a.value, b.value));
        }

        let a_abs = if a_ptr.is_null() {
            &[a.value.abs() as u64][..]
        } else {
            &*a_ptr
        };

        let b_abs = if b_ptr.is_null() {
            &[b.value.abs() as u64][..]
        } else {
            &*b_ptr
        };

        Slot::new_bool(numop(a.value.signum(), a_abs, b.value.signum(), b_abs))
    }

    unsafe fn op_add_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_number(a, b, |a, b| (a as i64 + b as i64), Number::overflowing_add)
    }

    unsafe fn op_add_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_bigint(a, b, |a, b| (a as i64 + b as i64), |a, b, c, d| (a + c, b))
    }

    unsafe fn op_sub_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_number(a, b, |a, b| (a as i64 - b as i64), |a, b| a.overflowing_sub(b))
    }

    unsafe fn op_sub_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_bigint(a, b, |a, b| (a as i64 - b as i64), |a, b, c, d| (a - c, b))
    }

    unsafe fn op_mul_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_number(a, b, |a, b| (a as i64 * b as i64), |a, b| a.overflowing_mul(b))
    }

    unsafe fn op_mul_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_binary_bigint(a, b, |a, b| (a as i64 * b as i64), |a, b, c, d| (a * c, b))
    }

    unsafe fn op_div_number(&mut self, a: Slot, b: Slot) -> Slot {
        unimplemented!("tricky")
    }
    
    unsafe fn op_div_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        unimplemented!("tricky")
    }

    unsafe fn op_mod_number(&mut self, a: Slot, b: Slot) -> Slot {
        unimplemented!("tricky")
    }

    unsafe fn op_mod_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        unimplemented!("tricky")
    }

    // TODO: revisit the entier comparison functions later

    unsafe fn op_less_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_number(a, b, |a, b| a < b, |a, b| a < b)
    }

    unsafe fn op_less_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        // TODO: copilot wrote, b and d might be little endian order,
        // but the comparison operator < might be in big endian order.
        // check both later
        self.op_comp_bigint(a, b, |a, b| a < b, |a, b, c, d| a < c || (a == c && b < d))
    }

    unsafe fn op_less_equal_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_number(a, b, |a, b| a <= b, |a, b| a <= b)
    }

    unsafe fn op_less_equal_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_bigint(a, b, |a, b| a <= b, |a, b, c, d| a < c || (a == c && b <= d))
    }

    unsafe fn op_greater_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_number(a, b, |a, b| a > b, |a, b| a > b)
    }

    unsafe fn op_greater_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_bigint(a, b, |a, b| a > b, |a, b, c, d| a > c || (a == c && b > d))
    }

    unsafe fn op_greater_equal_number(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_number(a, b, |a, b| a >= b, |a, b| a >= b)
    }

    unsafe fn op_greater_equal_bigint(&mut self, a: Slot, b: Slot) -> Slot {
        self.op_comp_bigint(a, b, |a, b| a >= b, |a, b, c, d| a > c || (a == c && b >= d))
    }

    unsafe fn coerce_number_to_bigint(&mut self, a: Slot) -> Slot {
        let a_ptr = a.get_number_pointer();
        if a_ptr.is_null() {
            return Slot::new_bigint(a.value as i64)
        } 
        
        let a_value = &*a_ptr;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&a_ptr[..8]);
        Slot::new_bigint(i64::from_le_bytes(buf))
    }


    pub fn eval_opcode(&mut self) {
        let op = self.code[self.pc];
        self.pc += 1;
        match op {
            Opcode::True => self.stack.push(Slot::new_true()),
            Opcode::False => self.stack.push(Slot::new_false()),
            Opcode::Undefined => self.stack.push(Slot::new_undefined()),
            Opcode::Null => self.stack.push(Slot::new_null()),
            Opcode::Integer1 => {
                let bytes = self.const_read_bytes_from_code::<1>();
                self.stack.push(Slot::new_i32(i8::from_le_bytes(*bytes) as i32));
            },
            Opcode::Integer2 => {
                let bytes = self.const_read_bytes_from_code::<2>();
                self.stack.push(Slot::new_i32(i16::from_le_bytes(*bytes) as i32));
            },
            Opcode::Integer4 => {
                let bytes = self.const_read_bytes_from_code::<4>();
                self.stack.push(Slot::new_i32(i32::from_le_bytes(*bytes)));
            },
            Opcode::Number => {
                unimplemented!("Number")
            },
            Opcode::Bigint1 => {
                let bytes = self.const_read_bytes_from_code::<1>();
                self.stack.push(Slot::new_b32(self.code[self.pc+1] as i32));
            },
            Opcode::Bigint2 => {
                let mut bytes = self.const_read_bytes_from_code::<2>();
                self.stack.push(Slot::new_b32(i16::from_le_bytes(*bytes) as i32));
            },
            Opcode::Bigint4 => {
                let mut bytes = self.const_read_bytes_from_code::<4>(); 
                self.stack.push(Slot::new_b32(i32::from_le_bytes(*bytes)));
            },
            Opcode::Bigint => {
                unimplemented!("Bigint")
            },
            Opcode::String1 => {
                let len_bytes = self.const_read_bytes_from_code::<1>();
                let len = len_bytes[0] as usize;
                let string_bytes = self.read_bytes_from_code(len);
                let string_ptr = self.memory.allocate_bytes::<str>(string_bytes);
                self.stack.push(Slot::new_string(string_ptr));
            },
            Opcode::Array1 => {
                let len_bytes = self.const_read_bytes_from_code::<1>();
                let len = len_bytes[0] as usize;
                // XXX is it word length of byte length, brain not working rn check it later
                let slots = self.pop_slots_from_stack(len);
                let array = self.memory.allocate_slots(slots);
                self.stack.push(Slot::new_reference(array));
            },
            Opcode::Object => {
                let len_bytes = self.const_read_bytes_from_code::<1>();
                let len = len_bytes[0] as usize;
                let pairs = self.pop_slots_from_stack(len*2);
                let object = self.map_pool.get();
                for i in (0..len) {
                    let key_slot = pairs[i*2];
                    let key = key_slot.to_string();
                    let value = pairs[i*2+1];
                    
                }
            },
            Opcode::Function => {
                
            },
            Opcode::Add => {   
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_add_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_add_bigint(a, b) });
                    },
                    (SlotTag::String, SlotTag::String) => {
                        // DIVERGENCE: ecmascript allows string concatenation
                        self.throw_type_error("cannot add strings");
                    },
                    (SlotTag::Number, SlotTag::Bigint) | (SlotTag::Bigint, SlotTag::Number) => {
                        self.throw_type_error("cannot add number and bigint");
                    },
                    _ => {
                        self.throw_type_error("cannot add");
                    }
                }
            },
            Opcode::Sub => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_sub_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_sub_bigint(a, b) });
                    },
                    (SlotTag::Number, SlotTag::Bigint) | (SlotTag::Bigint, SlotTag::Number) => {
                        self.throw_type_error("cannot sub number and bigint");
                    },
                    _ => {
                        self.throw_type_error("cannot sub");
                    }
                }
            },
            Opcode::Mul => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_mul_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_mul_bigint(a, b) });
                    },
                    (SlotTag::Number, SlotTag::Bigint) | (SlotTag::Bigint, SlotTag::Number) => {
                        self.throw_type_error("cannot mul number and bigint");
                    },
                    _ => {
                        self.throw_type_error("cannot mul");
                    }
                }
            },
            Opcode::Div => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_div_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_div_bigint(a, b) });
                    },
                    (SlotTag::Number, SlotTag::Bigint) | (SlotTag::Bigint, SlotTag::Number) => {
                        self.throw_type_error("cannot div number and bigint");
                    },
                    _ => {
                        self.throw_type_error("cannot div");
                    }
                }
            },
            Opcode::Mod => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_mod_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_mod_bigint(a, b) });
                    },
                    (SlotTag::Number, SlotTag::Bigint) | (SlotTag::Bigint, SlotTag::Number) => {
                        self.throw_type_error("cannot mod number and bigint");
                    },
                    _ => {
                        self.throw_type_error("cannot mod");
                    }
                }
            },
            Opcode::Pow => {
                unimplemented!()
            },
            Opcode::LessThan => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                match (a.tag(), b.tag()) {
                    (SlotTag::Number, SlotTag::Number) => {
                        self.stack.push(unsafe { self.op_less_than_number(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_less_than_bigint(a, b) });
                    },
                    (SlotTag::Number, SlotTag::Bigint) => {
                        self.stack.push(unsafe { self.op_less_than_bigint(a, b) });
                    },
                    (SlotTag::Bigint, SlotTag::Number) => {
                        
                    },
                    _ => {
                        self.throw_type_error("cannot less than");
                    }
                }
            },
            Opcode::LessThanEqual => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                self.stack.push(a <= b);
            },
            Opcode::GreaterThan => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                self.stack.push(a > b);
            },
            Opcode::GreaterThanEqual => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                self.stack.push(a >= b);
            },
            Opcode::StrictEqual => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                self.stack.push(a == b);
            },
            Opcode::StrictNotEqual => {
                let [b, a] = self.const_pop_slots_from_stack::<2>();
                self.stack.push(a != b);
            },
            Opcode::Not => {
                let a = self.stack.pop().unwrap();
                self.stack.push(!a);
            },
            Opcode::Call => {
            },
            Opcode::Branch1 => {
                let offset = self.const_read_bytes_from_code::<1>()[0] as usize;
                self.pc += offset;
                return
            },
            Opcode::Branch2 => {
                let offset = i16::from_le_bytes(*self.const_read_bytes_from_code::<2>());
                self.pc += offset;
                return
            },
            Opcode::Branch4 => {
                let offset = i32::from_le_bytes(*self.const_read_bytes_from_code::<4>());
                self.pc += offset;
                return
            },
            Opcode::BranchIf1 => {
                let a = self.stack.pop().unwrap();
                if a.is_truthy() {
                    let offset = self.const_read_bytes_from_code::<1>()[0] as usize;
                    self.pc += offset;
                    return
                }
            },
            Opcode::BranchIf2 => {
                let a = self.stack.pop().unwrap();
                if a.is_truthy() {
                    let offset = i16::from_le_bytes(*self.const_read_bytes_from_code::<2>());
                    self.pc += offset;
                    return
                }
            },
            Opcode::BranchIf4 => {
                let a = self.stack.pop().unwrap();
                if a.is_truthy() {
                    let offset = i32::from_le_bytes(*self.const_read_bytes_from_code::<4>());
                    self.pc += offset;
                    return
                }
            },
            Opcode::BranchElse1 => {
                let a = self.stack.pop().unwrap();
                if a.is_falsy() {
                    let offset = i32::from_le_bytes(*self.const_read_bytes_from_code::<4>());
                    self.pc += offset;
                    return
                } 
            },
            Opcode::BranchElse2 => {
                let a = self.stack.pop().unwrap();
                if a.is_falsy() {
                    let offset = i32::from_le_bytes(*self.const_read_bytes_from_code::<4>());
                    self.pc += offset;
                    return
                }
            },
            Opcode::BranchElse4 => {
                let a = self.stack.pop().unwrap();
                if a.is_falsy() {
                    let offset = i32::from_le_bytes(*self.const_read_bytes_from_code::<4>());
                    self.pc += offset;
                    return
                }
            },
            Opcode::Return => {

            },
            Opcode::Dup => {
                let a = self.stack.last().unwrap();
                self.stack.push(a);
            },
            Opcode::DupAt => {
                let a = self.stack[self.stack.len() - 1 - self.code[self.pc] as usize];
                self.stack.push(a);
            },
            Opcode::Pop => {
                self.stack.pop();
            },
            Opcode::Swap => {
                std::mem::swap(&mut self.stack[self.stack.len() - 1], &mut self.stack[self.stack.len() - 2]);
            },
            Opcode::ConstLocal1 => {
                let index = self.code[self.pc+1] as usize;
                self.stack.push(self.locals[index]);
                self.pc += 1;
            },
            Opcode::LetLocal1 => {
                
            },
            Opcode::GetLocal1 => {

            }
        }
    }
}