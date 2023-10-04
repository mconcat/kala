use jessie_ast::Expr;
use kala_repr::slot::Slot;

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
    Evaluation::Value(Slot::op_strict_equal(&x, &y))
}

pub fn strict_not_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_strict_not_equal(&x, &y))
}

pub fn less_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_less_than(&x, &y))
}

pub fn less_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_less_than_or_equal(&x, &y))
}

pub fn greater_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_greater_than(&x, &y))
}

pub fn greater_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_greater_than_or_equal(&x, &y))
}

pub fn add(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    let res = Evaluation::Value(Slot::op_add(x, y));
    res
}

pub fn sub(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_sub(x, y))
}

pub fn mul(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_mul(x, y))
}

pub fn div(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_div(x, y))
}

pub fn modulo(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_modulo(x, y))
}

pub fn pow(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Evaluation::Value(Slot::op_pow(x, y))
}

