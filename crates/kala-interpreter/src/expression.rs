use core::panic;
use std::mem::replace;

use jessie_ast::{Expr, DataLiteral, Array, Record, PropDef, AssignOp, CondExpr, BinaryExpr, BinaryOp, UnaryOp, CallExpr, CallPostOp, Function, CaptureDeclaration, LValue, UnaryExpr, Assignment, DeclarationIndex, LValueCallPostOp, VariableIndex};
use kala_repr::{slot::Slot, object::Property, completion::Completion};

use crate::{interpreter::Interpreter, operation::{strict_equal, strict_not_equal, less_than, less_than_or_equal, greater_than, greater_than_or_equal, add, sub, mul, div, modulo, pow}, statement::eval_block};


pub fn eval_expr(interpreter: &mut Interpreter, expr: &Expr) -> Completion {
    println!("eval_expr: {:?}", expr);
    match expr {
        Expr::DataLiteral(lit) => eval_literal(lit),
        Expr::Array(array) => eval_array(interpreter, array),
        Expr::Record(obj) => eval_record(interpreter, obj),
        Expr::Function(func) => eval_function(interpreter, (**func).clone()),
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

fn eval_literal(lit: &DataLiteral) -> Completion {
    match lit {
        DataLiteral::Null => Completion::Value(Slot::new_null()),
        DataLiteral::False => Completion::Value(Slot::new_false()),
        DataLiteral::True => Completion::Value(Slot::new_true()),
        DataLiteral::Integer(s) => Completion::Value(Slot::new_integer(*s)),
        DataLiteral::Decimal(i, f) => Completion::Value(Slot::new_number(*i, *f)),
        DataLiteral::Undefined => Completion::Value(Slot::new_undefined()),
        DataLiteral::String(s) => Completion::Value(Slot::new_string(s.clone())),
        DataLiteral::Bigint(sign, abs) => unimplemented!("bigint literal"),
    }
}

fn eval_array(interpreter: &mut Interpreter, array: &Array) -> Completion {
    let mut slots = Vec::with_capacity(array.0.len());
    for expr in &array.0 {
        match expr {
            Expr::Spread(spread) => {
                unimplemented!("spread in array literal");
            }
            _ => slots.push(eval_expr(interpreter, expr)?),
        }
    }
    Completion::Value(Slot::new_array(slots))
} 

fn eval_record(interpreter: &mut Interpreter, obj: &Record) -> Completion {
    let mut props = Vec::with_capacity(obj.0.len());
    for propdef in &obj.0 {
        match propdef {
            PropDef::KeyValue(key, value) => {
                props.push(Property{
                    key: key.dynamic_property.clone(),
                    value: eval_expr(interpreter, &value)?,
                });
            }
            PropDef::Shorthand(key, index) => {
                props.push(Property{
                    key: key.dynamic_property.clone(),
                    value: eval_variable(interpreter, index.get())?,
                });
            }
            PropDef::Spread(spread) => unimplemented!("spread in record literal")
        }
    }
    Completion::Value(Slot::new_object(props))
}
/* 
// invoked from the caller side before entering the function
fn enter_function(interpreter: &mut Interpreter, arguments: Vec<Slot>, captures: Vec<Slot>) {
    interpreter.stack.push_slots(arguments);
    interpreter.stack.push_slots(captures);
    interpreter.stack.enter_function_frame()
}

fn exit_function(interpreter: &mut Interpreter, arguments_len: usize, captures_len: usize) {
    interpreter.stack.exit_function_frame();
    interpreter.stack.pop_slots(arguments_len + captures_len);
}
*/

fn eval_function(interpreter: &mut Interpreter, func: Function) -> Completion {
    let captures: Vec<Slot> = func.declarations.captures.into_iter().map(|capture| {
        let variable = interpreter.fetch_variable(capture.variable.get());
        if variable.is_none() {
            panic!("should have variable");
        }
        variable.unwrap().clone()
    }
    ).collect();

    //let mut local_initializers: Vec<Option<Box<dyn FnOnce(&mut Frame) -> Completion>>> = Vec::with_capacity(func.locals.len());

    let statements = func.statements.clone();

    let builtins = interpreter.builtins.clone();

    let function = Slot::new_function(func.name.get_name().cloned(), Box::new(move |frame, arguments| {
        println!("function call: {:?}", frame);
        // push arguments
        frame.slots.extend(arguments);

        // enter function frame with captures and locals
        let recovery = frame.enter_function_frame(captures.clone(), func.declarations.locals.len());
        let frame_value = std::mem::take(frame);

        // hoist(pre-declare) function declarations
        // TODO
        // 

        let mut function_interpreter = Interpreter {
            builtins: builtins.clone(),
            current_frame: frame_value, 
        };
        let result = eval_block(&mut function_interpreter, &statements);

        let _ = replace(frame, function_interpreter.current_frame);

        // exit function frame, arguments still remain
        frame.exit_function_frame(recovery);

        result
    }));

    Completion::Value(function)
}

fn lvalue<'a>(interpreter: &'a mut Interpreter, lvalue: &LValue) -> Option<Slot> {
    match lvalue {
        LValue::Variable(index) => interpreter.fetch_variable(index.get()).cloned().into(),
        LValue::CallLValue(expr) => {
            let res_eval = eval_expr(interpreter, &expr.expr);
            let mut res = match res_eval {
                Completion::Value(slot) => slot,
                _ => return None,
            };
            for op in &expr.post_ops {
                res = match op {
                    LValueCallPostOp::Index(index_expr) => {
                        let index_eval = eval_expr(interpreter, &index_expr);
                        let index = match index_eval {
                            Completion::Value(slot) => slot,
                            _ => return None,
                        };
                        let res_eval = res.get_element(index.unwrap_integer().0.try_into().unwrap());
                        match res_eval {
                            Some(slot) => slot.clone(),
                            _ => return None,
                        }
                    } 
                    LValueCallPostOp::Member(field) => {
                        let res_eval = res.get_property(field);
                        match res_eval {
                            Some(slot) => slot.clone(),
                            _ => return None,
                        }
                    },
                }
            } 
            Some(res)
        }
    }
}

fn assign(interpreter: &mut Interpreter, lhs: &LValue, rhs: Slot) -> Completion {
    let mut lvalue = match lhs {
        LValue::Variable(var) => {
            match var.get().declaration_index {
                DeclarationIndex::Local(index) => interpreter.current_frame.get_local(index as usize),
                DeclarationIndex::Capture(index) => interpreter.current_frame.get_capture(index as usize),
                DeclarationIndex::Parameter(index) => interpreter.current_frame.get_argument(index as usize),
                DeclarationIndex::Builtin(_) => panic!("cannot assign to builtin")
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

    *lvalue = rhs;
    Completion::Value(lvalue.clone())
}

fn eval_assignment(interpreter: &mut Interpreter, assignment: &Assignment) -> Completion {
    let rhs = eval_expr(interpreter, &assignment.2)?;

    match assignment.0 {
        AssignOp::Assign => assign(interpreter, &assignment.1, rhs),
        _ => unimplemented!("assignment with operator")
    }
}

fn eval_cond(interpreter: &mut Interpreter, expr: &CondExpr) -> Completion {
    let cond = eval_expr(interpreter, &expr.0)?;
    if cond.is_truthy() {
        eval_expr(interpreter, &expr.1)
    } else {
        eval_expr(interpreter, &expr.2)
    }
}

fn eval_binary(interpreter: &mut Interpreter, expr: &BinaryExpr) -> Completion {
    match expr.0 {
        BinaryOp::Or => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_truthy() {
                Completion::Value(lhs)
            } else {
                eval_expr(interpreter, &expr.2)
            }
        },
        BinaryOp::And => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_truthy() {
                eval_expr(interpreter, &expr.2)
            } else {
                Completion::Value(lhs)
            }
        },
        BinaryOp::Coalesce => {
            let lhs = eval_expr(interpreter, &expr.1)?;
            if lhs.is_nullish() {
                eval_expr(interpreter, &expr.2)
            } else {
                Completion::Value(lhs)
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

fn eval_unary(interpreter: &mut Interpreter, expr: &UnaryExpr) -> Completion {
    let mut res = Slot::new_uninitialized();
    for op in &expr.op {
        res = match op {
            UnaryOp::Not => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                Completion::Value(Slot::new_boolean(!operand.is_truthy()))
            }
            UnaryOp::BitNot => {
                unimplemented!("bitwise not")
            }
            UnaryOp::Neg => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                Completion::Value(operand.op_neg())
            }
            UnaryOp::Pos => {
                let operand = eval_expr(interpreter, &expr.expr)?;
                unimplemented!("pos")
            }
            UnaryOp::TypeOf => {
                unimplemented!("typeof")
                /* 
                let operand = eval_expr(interpreter, &expr.expr)?;
                Completion::Value(operand.type_of().into())
                */
            }
        }?;
    }
    Completion::Value(res)
}

fn eval_call(interpreter: &mut Interpreter, expr: &CallExpr) -> Completion {
    println!("eval_call: {:?}", expr);

    let mut callee = eval_expr(interpreter, &expr.expr)?;
    for op in &expr.post_ops {
        match op {
            CallPostOp::Index(index) => {
                let index_slot = eval_expr(interpreter, &index)?;
                let index = index_slot.unwrap_integer().unwrap(); // TODO: non-smi array index
                callee = callee.get_element(index.try_into().unwrap()).cloned()?;
            }
            CallPostOp::Member(member) => { 
                callee = callee.get_property(member).cloned()?;
            }
            CallPostOp::Call(args) => {
                callee = call(interpreter, callee, args)?;
            }
        }
    }

    Completion::Value(callee)
}

fn call(interpreter: &mut Interpreter, callee: Slot, args: &Vec<Expr>) -> Completion {
    let argument_completions = args.into_iter().map(|arg| eval_expr(interpreter, &arg));

    let mut arguments = Vec::with_capacity(argument_completions.len());

    for arg_completion in argument_completions {
        arguments.push(arg_completion?)
    }
    let closure = callee.as_function()?;

    let result = (*closure.function)(&mut interpreter.current_frame, arguments);

    match result {
        Completion::Return(slot) => Completion::Value(slot),
        Completion::ReturnEmpty => Completion::Value(Slot::new_undefined()),
        Completion::Throw(_) => result,
        _ => Completion::Value(Slot::new_undefined()),
    }
}

fn eval_variable(interpreter: &mut Interpreter, index: VariableIndex) -> Completion {
    Completion::Value(interpreter.fetch_variable(index).map(|slot| slot.clone())?)
}