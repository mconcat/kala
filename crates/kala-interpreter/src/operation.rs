use jessie_ast::Expr;

use crate::{interpreter::{Interpreter, Evaluation}, expression::eval_expr};
/* 
pub fn bit_and(interpreter: &mut Interpreter, x: Expr, y: Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
}
*/

pub fn strict_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.strict_equal(&y))
}

pub fn strict_not_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.strict_not_equal(&y))
}

pub fn less_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.less_than(y).into())
}

pub fn less_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.less_than_or_equal(y).into())
}

pub fn greater_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.greater_than(y).into())
}

pub fn greater_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(x.greater_than_or_equal(y).into())
}

pub fn add(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    let res = Evaluation::Value(x+y);
    res
}

pub fn sub(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value((x-y).into())
}

pub fn mul(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value((x*y).into())
}

pub fn div(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    unimplemented!("div")
    /* 
    if y.is_zero() {
        Evaluation::Value(Slot::new_undefined())
    } else {
        Evaluation::Value((x/y).into())
    }*/
}

pub fn modulo(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
   
    unimplemented!("mod")
   // Evaluation::Value((x).into())
}

pub fn pow(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;

    unimplemented!("pow")

    //Evaluation::Value(x.pow(y).into())
}

