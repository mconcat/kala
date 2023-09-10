use jessie_ast::{Expr, DataLiteral, Array, Record, PropDef, AssignOp, CondExpr, BinaryExpr, BinaryOp, UnaryOp, CallExpr, CallPostOp, Function, CaptureDeclaration, LValue, UnaryExpr, Assignment, VariableCell, ParameterDeclaration, DeclarationIndex, LValueCallPostOp, VariableIndex};
use kala_repr::{slot::Slot, function::Variable};

use crate::{interpreter::{Evaluation, Interpreter}, operation::{strict_equal, strict_not_equal, less_than, less_than_or_equal, greater_than, greater_than_or_equal, add, sub, mul, div, modulo, pow}};


pub fn eval_expr(interpreter: &mut Interpreter, expr: &Expr) -> Evaluation {
    println!("eval_expr: {:?}", expr);
    match expr {
        Expr::DataLiteral(lit) => eval_literal(lit),
        Expr::Array(array) => eval_array(interpreter, array),
        Expr::Record(obj) => eval_record(interpreter, obj),
        Expr::Function(func) => eval_function(interpreter, func),
        Expr::Assignment(assignment) => eval_assignment(interpreter, assignment),
        Expr::CondExpr(cond) => eval_cond(interpreter, cond),
        Expr::BinaryExpr(binary) => eval_binary(interpreter, binary),
        Expr::UnaryExpr(unary) => eval_unary(interpreter, unary),
        Expr::CallExpr(call) => eval_call(interpreter, call),
        Expr::ParenedExpr(parened) => eval_expr(interpreter, &*parened),
        Expr::Variable(index) => eval_variable(interpreter, index.get()),
        Expr::Spread(spread) => unreachable!("Spread should be handled by eval_array"),
    }
}

fn eval_literal(lit: &DataLiteral) -> Evaluation {
    match lit {
        DataLiteral::Null => Evaluation::Value(Slot::new_null()),
        DataLiteral::False => Evaluation::Value(Slot::new_false()),
        DataLiteral::True => Evaluation::Value(Slot::new_true()),
        DataLiteral::Integer(s) => Evaluation::Value(
            if let Ok(smi) = s.try_into() {
                Slot::new_integer(smi)
            } else {
                Slot::new_number_from_parts([*s, 0])
            }
        ),
        DataLiteral::Decimal(n) => Evaluation::Value(Slot::new_number_from_parts(n.clone())),
        DataLiteral::Undefined => Evaluation::Value(Slot::new_undefined()),
        DataLiteral::String(s) => Evaluation::Value(Slot::new_string(s.clone())),
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
    let mut names = Vec::with_capacity(obj.0.len());
    let mut slots = Vec::with_capacity(obj.0.len());
    for propdef in &obj.0 {
        match propdef {
            PropDef::KeyValue(key, value) => {
                names.push(key.dynamic_property.clone());
                slots.push(eval_expr(interpreter, &value)?);
            }
            PropDef::Shorthand(key, index) => {
                names.push(key.dynamic_property.clone());
                slots.push(eval_variable(interpreter, index.get())?);
            }
            PropDef::Spread(spread) => unimplemented!("spread in record literal")
        }
    }
    Evaluation::Value(Slot::new_object(names, slots))
}

fn eval_function(interpreter: &mut Interpreter, func: &Function) -> Evaluation {
    let mut captures: Vec<Variable> = Vec::with_capacity(func.captures.len());
    for capture in &func.captures {
        match capture {
            CaptureDeclaration::Local { name, variable } => {
                captures.push(variable.get())
            }
            CaptureDeclaration::Global { name } => todo!("global capture")
        }
    }

    /* 
    let mut parameters = Vec::with_capacity(func.parameters.len());
    for parameter in &func.parameters {
        match parameter {
            ParameterDeclaration::Optional { name, default } => {
                parameters.push()
            }
            ParameterDeclaration::Pattern { pattern } => {
                todo!("pattern parameter")
            }
            ParameterDeclaration::Variable { name } => {
                parameters.push(name.clone());
            }
        }
    }
    */

    let slot = Slot::new_function(
        func.name.get_name().cloned(),
        func,
        captures,
    );

    Evaluation::Value(slot)
}

fn lvalue(interpreter: &mut Interpreter, lvalue: &LValue) -> Result<Variable, Slot> {
    match lvalue {
        LValue::Variable(index) => interpreter.fetch_variable(index.get()).ok_or(Slot::new_undefined()/* TODO: error instead of undefined*/),
        LValue::CallLValue(expr) => {
            let mut res: Variable = eval_expr(interpreter, &expr.expr).into()?;
            for op in expr.post_ops {
                res = match op {
                    LValueCallPostOp::Index(index) => res.get_element(eval_expr(interpreter, &index).into()?.as_number())?,
                    LValueCallPostOp::Member(field) => res.get_property(field)?,
                }
            } 
            Ok(res)
        }
    }
}

fn assign(interpreter: &mut Interpreter, lhs: &LValue, rhs: Slot) -> Evaluation {
    let mut lvalue = match lhs {
        LValue::Variable(var) => {
            let mut frame = interpreter.get_frame();
            match var.get().declaration_index {
                DeclarationIndex::Local(index) => frame.get_local(index as usize),
                DeclarationIndex::Capture(index) => frame.get_capture(index as usize),
                DeclarationIndex::Parameter(index) => frame.get_argument(index as usize),
            }
        }

        LValue::CallLValue(var) => {
            unimplemented!("call lvalue")
            /* 
            let mut lvalue = eval_expr(interpreter, &var.expr)?;
            for op in var.post_ops {
                match op {
                    LValueCallPostOp::Index(index) => {
                        lvalue = lvalue.get_element(eval_expr(interpreter, &index)?.as_number())?;
                    },
                    LValueCallPostOp::Member(field) => {
                        lvalue = lvalue.get_property(field)?;
                    },
                }
            }
            lvalue
            */
        }
    };

    // *lvalue = rhs;
    Evaluation::Value(lvalue)
}

fn eval_assignment(interpreter: &mut Interpreter, assignment: &Assignment) -> Evaluation {
    let rhs = eval_expr(interpreter, &assignment.2)?;

    match assignment.0 {
        AssignOp::Assign => assign(interpreter, &assignment.1, rhs),
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
    let mut res = Slot::new_uninitalized();
    for op in &expr.op {
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
        }?;
    }
    Evaluation::Value(res)
}

fn eval_call(interpreter: &mut Interpreter, expr: &CallExpr) -> Evaluation {
    let mut callee = eval_expr(interpreter, &expr.expr)?;
    for op in &expr.post_ops {
        match op {
            CallPostOp::Index(index) => {
                let index_slot = eval_expr(interpreter, &index)?;
                let index = index_slot.as_smi().unwrap(); // TODO: non-smi array index
                callee = callee.get_element(index)?;
            }
            CallPostOp::Member(member) => {
                callee = callee.get_property(member.clone())?;
            }
            CallPostOp::Call(args) => {
                callee = call(interpreter, callee, args)?;
            }
        }
    }

    Evaluation::Value(callee)
}

fn call(interpreter: &mut Interpreter, callee: Slot, args: &Vec<Expr>) -> Evaluation {
    /*
    let args = args.into_iter().map(|arg| eval_expr(interpreter, &arg)).collect::<Result<Vec<_>, _>>()?;
    let closure = callee.to_closure();
    let callee_frame = Frame {
        captures: closure.captures,
        arguments: args,
        locals: closure
        
         vec![(LocalDeclaration Slot::new_uninitalized()); closure.locals.len()],
    };

    let caller_frame = std::mem::replace(&mut interpreter.frame, callee_frame);
    let result = eval_block(interpreter, closure.body)?;
    std::mem::replace(&mut interpreter.frame, caller_frame);

    Evaluation::Value(result)
    */
    unimplemented!("call")
}

fn eval_variable(interpreter: &mut Interpreter, index: VariableIndex) -> Evaluation {
    let frame = interpreter.get_frame();

    let variable = match index.get().declaration_index {
        DeclarationIndex::Local(index) => frame.get_local(index as usize),
        DeclarationIndex::Capture(index) => frame.get_capture(index as usize),
        DeclarationIndex::Parameter(index) => frame.get_argument(index as usize),
    };

    let slot = frame.get_variable(variable.get());

    match slot {
        Some(slot) => Evaluation::Value(slot),
        None => unimplemented!("variable not found")
        /* 
        None => {
            let slot = Slot::new_uninitalized();
            frame.set_variable(variable.get(), slot);
            Evaluation::Value(slot)
        }
        */
    }
}