use std::mem::uninitialized;

use jessie_ast::{Expr, DataLiteral, Array, Record, PropDef, AssignOp, CondExpr, BinaryExpr, BinaryOp, UnaryOp, CallExpr, CallPostOp, Function, CaptureDeclaration, Variable, PropertyAccess, LValue, LValueCallPostOp};
use kala_repr::Slot;
use utils::{VectorMapPool, VectorMap, Map};

use crate::interpreter::Evaluation;


fn eval_expr(interpreter: &mut Interpreter, expr: Expr) -> Evaluation {
    match expr {
        Expr::DataLiteral(lit) => eval_literal(interpreter, lit),
        Expr::Array(array) => eval_array(interpreter, array),
        Expr::Record(obj) => eval_record(interpreter, obj),
        Expr::ArrowFunc(func) => eval_function(interpreter, func),
        Expr::FunctionExpr(func) => eval_function(interpreter, func),
        Expr::Assignment(assignment) => eval_assignment(interpreter, assignment),
        Expr::CondExpr(cond) => eval_cond(interpreter, cond),
        Expr::BinaryExpr(binary) => eval_binary(interpreter, binary),
        Expr::UnaryExpr(unary) => eval_unary(interpreter, unary),
        Expr::CallExpr(call) => eval_call(interpreter, call),
        Expr::ParenedExpr(parened) => eval_expr(interpreter, *parened),
        Expr::Variable(variable) => eval_variable(interpreter, variable),
        Expr::Spread(spread) => unreachable!("Spread should be handled by eval_array"),
    }
}

fn eval_literal(interpreter: &mut Interpreter, lit: DataLiteral) -> Evaluation {
    match lit {
        DataLiteral::Null => Evaluation::Value(Slot::new_null()),
        DataLiteral::False => Evaluation::Value(Slot::new_false()),
        DataLiteral::True => Evaluation::Value(Slot::new_true()),
        DataLiteral::Integer(s) => Evaluation::Value(Slot::new_integer(s.parse::<i64>().unwrap())),
        DataLiteral::Decimal(s) => unimplemented!("decimal literal"),
        DataLiteral::Undefined => Ok(Slot::new_undefined()),
        DataLiteral::String(s) => Evaluation::Value(Slot::new_string(s)),
        DataLiteral::Bigint(s) => unimplemented!("bigint literal"),
    }
}

fn eval_array(interpreter: &mut Interpreter, array: Array) -> Result<Slot, String> {
    let mut slots = Vec::with_capacity(array.0.len());
    for expr in array.0 {
        match expr {
            Expr::Spread(spread) => {
                unimplemented!("spread in array literal");
            }
            _ => slots.push(eval_expr(interpreter, expr)?),
        }
    }
    Ok(Slot::new_array(slots))
}

fn eval_record(interpreter: &mut Interpreter, obj: Record) -> Result<Slot, String> {
    let mut slots = VectorMap::with_capacity(obj.0.len());
    for propdef in obj.0 {
        match propdef {
            PropDef::KeyValue(key, value) => {
                slots.insert(&key.dynamic_property, eval_expr(interpreter, value)?)?;
            }
            PropDef::Shorthand(key, var) => {
                slots.insert(&key.dynamic_property, eval_variable(interpreter, var)?)?;
            }
            PropDef::Spread(spread) => unimplemented!("spread in record literal")
        }
    }
    Ok(Slot::new_record(slots))
}

fn eval_function(interpreter: &mut Interpreter, func: Function) -> Result<Slot, String> {
    let mut captures = Vec::with_capacity(func.captures.len());
    for capture in func.captures {
        match capture {
            CaptureDeclaration::Local { name, variable } => {
                captures.push(variable.get());
            }
            CaptureDeclaration::Global { name } => todo!("global capture")
        }
    }


    let slot = Slot::new_function(
        func,
        captures,
    ); 

    Ok(slot)
}

fn assign(interpreter: &mut Interpreter, lhs: LValue, rhs: Slot) -> Evaluation {
    let lvalue = match lhs {
        LValue::Variable(var) => interpreter.frame.get_variable(var.get()),

        LValue::CallLValue(var) => {
            let mut lvalue = eval_expr(interpreter, var.expr)?;
            for op in var.post_op {
                match op {
                    LValueCallPostOp::Index(index) => {
                        lvalue = lvalue.get_element(index);
                    },
                    LValueCallPostOp::Member(field) => {
                        lvalue = lvalue.get_property(field);
                    },
                }
            }
            lvalue
        }
    };

    lvalue.set(rhs);
    Evaluation::Value(lvalue)
}

fn eval_assignment(interpreter: &mut Interpreter, assignment: Assignment) -> Result<Slot, String> {
    match assignment.0 {
        AssignOp::Assign => assign(interpreter, assignment.1, eval_expr(interpreter, assignment.2)?),
        _ => unimplemented!("assignment with operator")
    }
}

fn eval_cond(interpreter: &mut Interpreter, expr: CondExpr) -> Result<Slot, String> {
    let cond = eval_expr(interpreter, expr.0)?;
    if cond.is_truthy() {
        eval_expr(interpreter, expr.1)
    } else {
        eval_expr(interpreter, expr.2)
    }
}

fn eval_binary(interpreter: &mut Interpreter, expr: BinaryExpr) -> Result<Slot, String> {
    match expr.0 {
        BinaryOp::Or => {
            let lhs = eval_expr(interpreter, expr.1)?;
            if lhs.is_truthy() {
                Ok(lhs)
            } else {
                eval_expr(interpreter, expr.2)
            }
        },
        BinaryOp::And => {
            let lhs = eval_expr(interpreter, expr.1)?;
            if lhs.is_truthy() {
                eval_expr(interpreter, expr.2)
            } else {
                Ok(lhs)
            }
        },
        BinaryOp::Coalesce => {
            let lhs = eval_expr(interpreter, expr.1)?;
            if lhs.is_nullish() {
                eval_expr(interpreter, expr.2)
            } else {
                Ok(lhs)
            }
        },

        BinaryOp::BitAnd => bit_and(interpreter, expr.1, expr.2),
        BinaryOp::BitOr => bit_or(interpreter, expr.1, expr.2),
        BinaryOp::BitXor => bit_xor(interpreter, expr.1, expr.2),

        BinaryOp::StrictEqual => strict_equal(interpreter, expr.1, expr.2),
        BinaryOp::StrictNotEqual => strict_not_equal(interpreter, expr.1, expr.2),

        BinaryOp::LessThan => less_than(interpreter, expr.1, expr.2),
        BinaryOp::LessThanEqual => less_than_equal(interpreter, expr.1, expr.2),
        BinaryOp::GreaterThan => greater_than(interpreter, expr.1, expr.2),
        BinaryOp::GreaterThanEqual => greater_than_equal(interpreter, expr.1, expr.2),

        BinaryOp::BitLeftShift => bit_left_shift(interpreter, expr.1, expr.2),
        BinaryOp::BitRightShift => bit_right_shift(interpreter, expr.1, expr.2),
        BinaryOp::BitUnsignedRightShift => bit_unsigned_right_shift(interpreter, expr.1, expr.2),

        BinaryOp::Add => add(interpreter, expr.1, expr.2),
        BinaryOp::Sub => sub(interpreter, expr.1, expr.2),
        BinaryOp::Mul => mul(interpreter, expr.1, expr.2),
        BinaryOp::Div => div(interpreter, expr.1, expr.2),
        BinaryOp::Mod => modulo(interpreter, expr.1, expr.2),
        BinaryOp::Pow => pow(interpreter, expr.1, expr.2),
    }
}

fn eval_unary(interpreter: &mut Interpreter, expr: UnaryExpr) -> Result<Slot, String> {
    match expr.0 {
        UnaryOp::Not => {
            let operand = eval_expr(interpreter, expr.1)?;
            Ok(Slot::new_boolean(!operand.is_truthy()))
        }
        UnaryOp::BitNot => {
            unimplemented!("bitwise not")
        }
        UnaryOp::Neg => {
            let operand = eval_expr(interpreter, expr.1)?;
            Ok(Slot::new_integer(-operand.to_integer()))
        }
        UnaryOp::Pos => {
            let operand = eval_expr(interpreter, expr.1)?;
            Ok(Slot::new_integer(operand.to_integer()))
        }
        UnaryOp::TypeOf => {
            let operand = eval_expr(interpreter, expr.1)?;
            Ok(Slot::new_string(operand.typeof()))
        }
    }
}

fn eval_call(interpreter: &mut Interpreter, expr: CallExpr) -> Result<Slot, String> {
    let mut callee = eval_expr(interpreter, expr.0)?;
    for op in expr.post_ops {
        match op {
            CallPostOp::Index(index) => {
                callee = callee.get_index(eval_expr(interpreter, index)?);
            }
            CallPostOp::Member(member) => {
                callee = callee.get_property(member);
            }
            CallPostOp::Call(args) => {
                callee = call(interpreter, callee, args)?;
            }
        }
    }

    Ok(callee)
}

fn call(interpreter: &mut Interpreter, callee: Slot, args: Vec<Expr>) -> Result<Slot, String> {
    let args = args.into_iter().map(|arg| eval_expr(interpreter, arg)).collect::<Result<Vec<_>, _>>()?;
    let closure = callee.to_closure();
    let callee_frame = Frame {
        captures: closure.captures,
        arguments: args,
        locals: vec![Slot::new_uninitalized(); closure.locals.len()],
    };

    let caller_frame = std::mem::replace(&mut interpreter.frame, callee_frame);
    let result = eval_block(interpreter, closure.body)?;
    std::mem::replace(&mut interpreter.frame, caller_frame);

    Ok(result)
}

fn eval_variable(interpreter: &mut Interpreter, variable: Variable) -> Result<Slot, String> {
    let index = variable.declaration_index;
    if index < interpreter.frame.locals.len() {
        Ok(interpreter.frame.locals[index])
    } else {
        Err(format!("Variable {} not found", variable.name))
    }
}