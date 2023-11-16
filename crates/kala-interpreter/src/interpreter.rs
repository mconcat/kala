
use std::{rc::Rc, cell::{OnceCell, Cell}};

use jessie_ast::{VariableIndex, DeclarationIndex, PropertyAccess};
use kala_repr::{slot::Slot, function::{Frame}};


/* 
#[derive(Debug, PartialEq, Clone)]
pub struct BlockFlag(pub u32);

impl BlockFlag {
    pub fn new() -> Self {
        BlockFlag(0)
    }

    // returns true except for the global scope
    // return is allowed
    pub fn within_function(&self) -> bool {
        self.0 & 0b0001 == 1
    }

    // for or while
    // continue and break are allowed
    pub fn within_loop(&self) -> bool {
        self.0 & 0b0010 == 1
    }

    // try block
    // if true, throw will be caught by the catch block
    // if false, throw will invoke rust side panic
    pub fn within_try(&self) -> bool {
        self.0 & 0b0100 == 1
    }

    // switch block
    // break is obligatory at the end of each case per jessie spec
    pub fn within_switch(&self) -> bool {
        self.0 & 0b1000 == 1
    }

    pub fn set_within_function(&mut self) {
        self.0 |= 0b0001;
    }

    pub fn set_within_loop(&mut self) {
        self.0 |= 0b0010;
    }

    pub fn set_within_try(&mut self) {
        self.0 |= 0b0100;
    }

    pub fn set_within_switch(&mut self) {
        self.0 |= 0b1000;
    }

    pub fn clear_within_function(&mut self) {
        self.0 &= 0b1110;
    }

    pub fn clear_within_loop(&mut self) {
        self.0 &= 0b1101;
    }

    pub fn clear_within_try(&mut self) {
        self.0 &= 0b1011;
    }

    pub fn clear_within_switch(&mut self) {
        self.0 &= 0b0111;
    }
}
*/
pub struct Interpreter {
    pub(crate) builtins: Rc<OnceCell<Vec<Cell<Slot>>>>,
   // pub(crate) stack: &'a mut Stack,
    pub(crate) current_frame: Frame,
}
/*
impl Drop for Interpreter {
    fn drop(&mut self) {
        println!("Interpreter dropped");
        if self.frame.len() > 1 {
            panic!("Frame leak");
        }

        drop(self.stack.drain(..));
        drop(self.frame.drain(..));
    }
}
*/
impl Interpreter {
    pub fn new(builtins: Rc<OnceCell<Vec<Cell<Slot>>>>, current_frame: Frame) -> Self {
        Interpreter {
            builtins,
            current_frame,
        }
    }

    pub fn empty() -> Self {
        Interpreter {
            builtins: Rc::new(OnceCell::new()),
            current_frame: Frame::empty(),
        }
    }

    pub fn fetch_variable(&mut self, index: VariableIndex) -> Option<&mut Slot> {
        let mut var = match index.declaration_index {
            DeclarationIndex::Capture(index) => self.current_frame.get_capture(index as usize),
            DeclarationIndex::Local(index) => self.current_frame.get_local(index as usize),
            DeclarationIndex::Parameter(index) => self.current_frame.get_argument(index as usize),
            DeclarationIndex::Builtin(index) => unsafe{&mut*self.builtins.get().unwrap().get(index as usize).unwrap().as_ptr()},
        };

        for property in index.property_access {
            var = match property {
                PropertyAccess::Element(elem) => var.get_element(elem.try_into().unwrap())?,
                PropertyAccess::Property(prop) => var.get_property(&prop.dynamic_property)?,
            }
        };

        Some(var)
    }
/* 
    fn get_local_initial_value(&mut self, index: u32) -> Evaluation {
        let decl = self.get_frame().get_local(index as usize);

        let decl = self.get_frame().get_local(index as usize)?.clone();
        let initial_value_expr = decl.get_initial_value();
        match initial_value_expr {
            None => Evaluation::Value(Slot::new_undefined()),
            Some(initial_value_expr) => eval_expr(self, initial_value_expr)
        }
    }
    */
   /* 
    pub fn initialize_local(&mut self, index: u32) -> Completion {
        let decls = &self.frame.last().unwrap().1;

        let initial_expr = decls[index as usize].get_initial_value().as_ref().unwrap().clone(); // we dont really need to clone here but we do it for the sake of the borrow checker TODO fix this
        let initial_value_eval = eval_expr(self, &initial_expr);
        let slot = &mut*self.get_frame().get_local(index as usize)?;

        *slot = Into::<Completion>::into(initial_value_eval)?;
        Completion::Normal
    }*/
}