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
    Ok(x.strict_equal(&y).into())
}

pub fn strict_not_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.strict_not_equal(&y).into())
}

pub fn less_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.less_than(&y).into())
}

pub fn less_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.less_than_or_equal(&y).into())
}

pub fn greater_than(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.greater_than(&y).into())
}

pub fn greater_than_or_equal(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.greater_than_or_equal(&y).into())
}

pub fn add(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.add(&y).into())
}

pub fn sub(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.sub(&y).into())
}

pub fn mul(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    Ok(x.mul(&y).into())
}

pub fn div(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    if y.is_zero() {
        Ok(Expr::undefined().into())
    } else {
        Ok(x.div(&y).into())
    }
}

pub fn modulo(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;
    if y.is_zero() {
        Ok(Expr::undefined().into())
    } else {
        Ok(x.modulo(&y).into())
    }
}

pub fn pow(interpreter: &mut Interpreter, x: &Expr, y: &Expr) -> Evaluation {
    let x = eval_expr(interpreter, x)?;
    let y = eval_expr(interpreter, y)?;

    Ok(x.pow(&y).into())
}

