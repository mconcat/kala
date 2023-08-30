use jessie_ast::{Expr, DataLiteral, Array, Record, PropDef, AssignOp, CondExpr, BinaryExpr, BinaryOp, UnaryOp, CallExpr, CallPostOp, Function, CaptureDeclaration, Variable, PropertyAccess, LValue, LValueCallPostOp, UnaryExpr, Assignment};
use kala_repr::slot::Slot;
use utils::{VectorMapPool, VectorMap, Map, SharedString};

use crate::{interpreter::{Evaluation, Interpreter, Frame}, operation::{strict_equal, strict_not_equal, less_than, less_than_or_equal, greater_than, greater_than_or_equal, add, sub, mul, div, modulo, pow}, statement::eval_block};


pub fn eval_expr(interpreter: &mut Interpreter, expr: &Expr) -> Evaluation {
    match expr {
        Expr::DataLiteral(lit) => eval_literal(interpreter, lit),
        Expr::Array(array) => eval_array(interpreter, array),
        Expr::Record(obj) => eval_record(interpreter, obj),
        Expr::Function(func) => eval_function(interpreter, func),
        Expr::Assignment(assignment) => eval_assignment(interpreter, assignment),
        Expr::CondExpr(cond) => eval_cond(interpreter, cond),
        Expr::BinaryExpr(binary) => eval_binary(interpreter, binary),
        Expr::UnaryExpr(unary) => eval_unary(interpreter, unary),
        Expr::CallExpr(call) => eval_call(interpreter, call),
        Expr::ParenedExpr(parened) => eval_expr(interpreter, &*parened),
        Expr::Variable(variable) => eval_variable(interpreter, &variable.get()),
        Expr::Spread(spread) => unreachable!("Spread should be handled by eval_array"),
    }
}

fn eval_literal(interpreter: &mut Interpreter, lit: &DataLiteral) -> Evaluation {
    match lit {
        DataLiteral::Null => Evaluation::Value(Slot::new_null()),
        DataLiteral::False => Evaluation::Value(Slot::new_false()),
        DataLiteral::True => Evaluation::Value(Slot::new_true()),
        DataLiteral::Integer(s) => Evaluation::Value(Slot::new_number_from_parts(*s, 0)),
        DataLiteral::Decimal(i, f) => Evaluation::Value(Slot::new_number_from_parts(i, f)),
        DataLiteral::Undefined => Evaluation::Value(Slot::new_undefined()),
        DataLiteral::String(s) => Evaluation::Value(Slot::new_string(*s)),
        DataLiteral::Bigint(sign, abs) => unimplemented!("bigint literal"),
    }
}

fn eval_array(interpreter: &mut Interpreter, array: &Array) -> Evaluation {
    let mut slots = Vec::with_capacity(array.0.len());
    for expr in &array.0 {
        match expr {
            Expr::Spread(spread) => {
                unimplemented!("spread in array literal");
            }
            _ => slots.push(eval_expr(interpreter, expr)?),
        }
    }
    Evaluation::Value(Slot::new_array(slots))
} 

fn eval_record(interpreter: &mut Interpreter, obj: &Record) -> Evaluation {
    let mut slots = VectorMap::with_capacity(obj.0.len());
    for propdef in obj.0 {
        match propdef {
            PropDef::KeyValue(key, value) => {
                slots.insert(&key.dynamic_property, eval_expr(interpreter, &value)?)?;
            }
            PropDef::Shorthand(key, var) => {
                slots.insert(&key.dynamic_property, eval_variable(interpreter, &var.get())?).into()?;
            }
            PropDef::Spread(spread) => unimplemented!("spread in record literal")
        }
    }
    Ok(Slot::new_object(slots))
}

fn eval_function(interpreter: &mut Interpreter, func: &Function) -> Evaluation {
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

fn assign(interpreter: &mut Interpreter, lhs: &LValue, rhs: Slot) -> Evaluation {
    let lvalue = match lhs {
        LValue::Variable(var) => interpreter.frame.get_variable(var.get()),

        LValue::CallLValue(var) => {
            let mut lvalue = eval_expr(interpreter, &var.expr)?;
            for op in var.post_ops {
                match op {
                    LValueCallPostOp::Index(index) => {
                        lvalue = lvalue.get_element(eval_expr(interpreter, index)?.as_number())?;
                    },
                    LValueCallPostOp::Member(field) => {
                        lvalue = lvalue.get_property(field)?;
                    },
                }
            }
            lvalue
        }
    };

    lvalue.set(rhs);
    Evaluation::Value(lvalue)
}

fn eval_assignment(interpreter: &mut Interpreter, assignment: &Assignment) -> Evaluation {
    match assignment.0 {
        AssignOp::Assign => assign(interpreter, &assignment.1, eval_expr(interpreter, &assignment.2)?),
        _ => unimplemented!("assignment with operator")
    }
}

fn eval_cond(interpreter: &mut Interpreter, expr: &CondExpr) -> Evaluation {
    let cond = eval_expr(interpreter, &expr.0)?;
    if cond.is_truthy() {
        eval_expr(interpreter, &expr.1)
    } else {
        eval_expr(interpreter, &expr.2)
    }
}

fn eval_binary(interpreter: &mut Interpreter, expr: &BinaryExpr) -> Evaluation {
    match expr.0 {
        BinaryOp::Or => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_truthy() {
                Evaluation::Value(lhs)
            } else {
                eval_expr(interpreter, &expr.2)
            }
        },
        BinaryOp::And => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_truthy() {
                eval_expr(interpreter, &expr.2)
            } else {
                Evaluation::Value(lhs)
            }
        },
        BinaryOp::Coalesce => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_nullish() {
                eval_expr(interpreter, &expr.2)
            } else {
                Evaluation::Value(lhs)
            }
        },

        BinaryOp::BitAnd => unimplemented!("bitand"), // bit_and(interpreter, expr.1, expr.2),
        BinaryOp::BitOr => unimplemented!("bitor"), // bit_or(interpreter, expr.1, expr.2),
        BinaryOp::BitXor => unimplemented!("bitxor"), // bit_xor(interpreter, expr.1, expr.2),

        BinaryOp::StrictEqual => strict_equal(interpreter, &expr.1, &expr.2),
        BinaryOp::StrictNotEqual => strict_not_equal(interpreter, &expr.1,&expr.2),

        BinaryOp::LessThan => less_than(interpreter, &expr.1, &expr.2),
        BinaryOp::LessThanEqual => less_than_or_equal(interpreter, &expr.1, &expr.2),
        BinaryOp::GreaterThan => greater_than(interpreter, &expr.1, &expr.2),
        BinaryOp::GreaterThanEqual => greater_than_or_equal(interpreter, &expr.1, &expr.2),

        BinaryOp::BitLeftShift => unimplemented!("leftshift"), // bit_left_shift(interpreter, expr.1, expr.2),
        BinaryOp::BitRightShift => unimplemented!("rightshift"), // bit_right_shift(interpreter, expr.1, expr.2),
        BinaryOp::BitUnsignedRightShift => unimplemented!("urightshift"), // bit_unsigned_right_shift(interpreter, expr.1, expr.2),

        BinaryOp::Add => add(interpreter, &expr.1, &expr.2),
        BinaryOp::Sub => sub(interpreter, &expr.1, &expr.2),
        BinaryOp::Mul => mul(interpreter, &expr.1, &expr.2),
        BinaryOp::Div => div(interpreter, &expr.1, &expr.2),
        BinaryOp::Mod => modulo(interpreter, &expr.1, &expr.2),
        BinaryOp::Pow => pow(interpreter, &expr.1, &expr.2),
    }
}

fn eval_unary(interpreter: &mut Interpreter, expr: &UnaryExpr) -> Evaluation {
    let mut res = Evaluation::Value(Slot::new_uninitalized());
    for op in expr.op {
        res = match op {
            UnaryOp::Not => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                Evaluation::Value(Slot::new_boolean(!operand.is_truthy()))
            }
            UnaryOp::BitNot => {
                unimplemented!("bitwise not")
            }
            UnaryOp::Neg => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                Evaluation::Value(-operand)
            }
            UnaryOp::Pos => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                unimplemented!("pos")
            }
            UnaryOp::TypeOf => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                Evaluation::Value(operand.type_of().into())
            }
        };

        res?;
    }
    res
}

fn eval_call(interpreter: &mut Interpreter, expr: &CallExpr) -> Evaluation {
    let mut callee = eval_expr(interpreter, &expr.expr)?;
    for op in expr.post_ops {
        match op {
            CallPostOp::Index(index) => {
                callee = callee.get_element(eval_expr(interpreter, &index)?).into()?;
            }
            CallPostOp::Member(member) => {
                callee = callee.get_property(member).into()?;
            }
            CallPostOp::Call(args) => {
                callee = call(interpreter, callee, args)?;
            }
        }
    }

    Evaluation::Value(callee)
}

fn call(interpreter: &mut Interpreter, callee: Slot, args: Vec<Expr>) -> Evaluation {
    let args = args.into_iter().map(|arg| eval_expr(interpreter, &arg)).collect::<Result<Vec<_>, _>>()?;
    let closure = callee.to_closure();
    let callee_frame = Frame {
        captures: closure.captures,
        arguments: args,
        locals: vec![Slot::new_uninitalized(); closure.locals.len()],
    };

    let caller_frame = std::mem::replace(&mut interpreter.frame, callee_frame);
    let result = eval_block(interpreter, closure.body)?;
    std::mem::replace(&mut interpreter.frame, caller_frame);

    Evaluation::Value(result)
}

fn eval_variable(interpreter: &mut Interpreter, variable: &Variable) -> Evaluation {
    let index = variable.declaration_index;
    let frame = interpreter.get_frame();
    if index < frame.locals.len() {
        Evaluation::Value(frame.locals[index])
    } else {
        Evaluation::Throw(Slot::new_string(SharedString::from_string(format!("Variable {} not found", variable.name))))
    }
}