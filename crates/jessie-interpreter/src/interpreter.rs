use jessie_ast::*;

// Interpreter exeuctes a single script. 
pub struct Interpreter {
    stack: Vec<Value>,
    ops: Vec<Opcode>,
}

impl Interpreter {
    pub fn eval(&self, expr: Expr) {
        
    }
}