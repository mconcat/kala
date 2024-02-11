use core::panic;
use std::{borrow::Borrow, mem::{replace, self}, rc::Rc};

use jessie_ast::{Array, AssignOp, Assignment, BinaryExpr, BinaryOp, CallExpr, CallLValue, CallPostOp, CondExpr, DataLiteral, Expr, ExprOrBlock, Function, LValue, LValueCallPostOp, PropDef, Record, UnaryExpr, UnaryOp, Variable, VariableIndex};
use kala_repr::{slot::Slot, object::Property, completion::Completion, function::Frame};

use crate::{interpreter::Interpreter, operation::{strict_equal, strict_not_equal, less_than, less_than_or_equal, greater_than, greater_than_or_equal, add, sub, mul, div, modulo, pow}, statement::eval_block};


pub fn eval_expr(interpreter: &mut Interpreter, expr: &Expr) -> Completion {
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
        Expr::Variable(var) => eval_variable(interpreter, var.as_ref().clone()),
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
    for expr in array.0.iter() {
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
    for propdef in obj.0.iter() {
        match propdef {
            PropDef::KeyValue(key, value) => {
                props.push(Property::data(
                    key.name.clone(),
                    eval_expr(interpreter, &value)?,
                ));
            }
            PropDef::Shorthand(key, var) => {
                props.push(Property::data(
                    key.name.clone(),
                    eval_variable(interpreter, var.as_ref().clone())?,
                ));
            }
            PropDef::Getter(func) => {
                let found = props.as_mut_slice().into_iter().find(|prop| Some(prop.key.clone()) == func.get_name());
                if let Some(prop) = found {
                    if prop.getter != Slot::UNINITIALIZED {
                        panic!("duplicate getter");
                    }
                    prop.getter = eval_function(interpreter, func)?;
                } else {
                    props.push(Property::getter(
                        func.get_name().unwrap(),
                        eval_function(interpreter, func)?,
                    ));
                }
            }
            PropDef::Setter(func) => {
                let found = props.as_mut_slice().into_iter().find(|prop| Some(prop.key.clone()) == func.get_name());
                if let Some(prop) = found {
                    if prop.setter != Slot::UNINITIALIZED {
                        panic!("duplicate setter");
                    }
                    prop.setter = eval_function(interpreter, func)?; 
                } else {
                    props.push(Property::setter(
                        func.get_name().unwrap(),
                        eval_function(interpreter, func)?,
                    ));
                }
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

fn eval_function(interpreter: &mut Interpreter, func: &Function) -> Completion {

    //let mut local_initializers: Vec<Option<Box<dyn FnOnce(&mut Frame) -> Completion>>> = Vec::with_capacity(func.locals.len());
    let builtins = interpreter.builtins.clone();

    let mut captures = Vec::with_capacity(func.scope.as_ref().unwrap().captures.len());

    for capture in func.scope.as_ref().unwrap().captures.iter() {
        let variable = interpreter.fetch_variable(capture.index());
        if variable.is_none() {
            panic!("should have variable");
        }
        captures.push(variable.unwrap().clone());
    }

    let func = func.clone();

    let function = Slot::new_function(func.get_name(), Rc::new(move |frame: &mut Frame, arguments| {
        // NOTE: because of how local values are always initialized after the function declarations(because of the hoisting), we are capturing the variables when the function is called, not when it is declared.
        // This would cause a problem when the function escapes the scope where it is defined, probably in a way that the variables being captured will be the ones in the scope where the function is called, not where it is defined.
        // To fix this problem, when a locally defined function escapes a scope, we need to capture the variables at that point, not when the function is called.
        // We don't need to do it recursively as the list of captured variables contains all the children captured variables, see jessie_parser/scope.rs
        // I think this behavior is equivalent to how javascript works.

        println!("function called: {:?} {:?}", func.get_name(), arguments);

        println!("captures: {:?}", captures);

        let mut frame_value = std::mem::take(frame);

        // push arguments
        frame_value.slots.extend(arguments);

        println!("pushed arguments: {:?}", frame_value.slots);

        // enter function frame with captures and locals
        let recovery = frame_value.enter_function_frame(captures.clone(), func.locals().len());

        println!("function frame: {:?}", frame_value);

        let mut function_interpreter = Interpreter {
            builtins: builtins.clone(),
            current_frame: frame_value, 
        };

        // hoist(pre-declare) function declarations
        for (function_var, local_function) in func.functions().iter() {
            // promote local variables to heap if it is captured by any of the local functions
            let function: &Function = &local_function.as_ref().try_borrow().unwrap();
            for local_function_capture in function.captures() {
                if let VariableIndex::Local(_, index) = local_function_capture.index() {
                    let captured_var = function_interpreter.current_frame.get_local(index as usize);

                    // promote local variable to heap
                    *captured_var = Slot::new_variable_slot();
                    println!("promoted local variable to heap: {:?}@{:?}", captured_var, index)
                }
            }

            let local_evaluated_function = eval_function(&mut function_interpreter, &local_function.as_ref().borrow())?;
            *function_interpreter.current_frame.get_local(function_var.index().unwrap_local() as usize) = local_evaluated_function;
        }

        let result = match &func.body {
            ExprOrBlock::Block(block) => eval_block(&mut function_interpreter, block),
            ExprOrBlock::Expr(expr) => eval_expr(&mut function_interpreter, expr),
        };

        let _ = replace(frame, function_interpreter.current_frame);

        // exit function frame, arguments still remain

        println!("exit 1: function frame: {:?}", frame);
        frame.exit_function_frame(recovery);
        println!("exit 2: function frame: {:?}", frame);
        result
    }));

    Completion::Value(function)
}

fn assign(interpreter: &mut Interpreter, lhs: &LValue, rhs: Slot) -> Completion {
    if let LValue::Variable(var) = lhs {
        let lvalue = match var.index() {
            VariableIndex::Local(_, index) => interpreter.current_frame.get_local(index as usize),
            VariableIndex::Captured(index) => interpreter.current_frame.get_capture(index as usize),
            VariableIndex::Parameter(index) => interpreter.current_frame.get_argument(index as usize),
            VariableIndex::Static(_) => todo!("static variable assignment"), 
        };
        lvalue.set(rhs);
        return Completion::Value(lvalue.clone())
    }



    let LValue::CallLValue(lvalue) = lhs else { unreachable!("invalid lvalue") };
    let mut obj = eval_expr(interpreter, &lvalue.expr)?;

    // This whole part is ugly
    // please please please refactor this
    enum ElementOrProperty<'a> {
        Element(&'a mut Slot),
        Property(&'a mut Property),
    }


    let mut prop = match lvalue.post_ops[0] {
        LValueCallPostOp::Index(ref index) => {
            let index = eval_expr(interpreter, index)?;
            ElementOrProperty::Element(
                obj.get_element(index.unwrap_integer().unwrap().try_into().unwrap())?,
            )
        },
        LValueCallPostOp::Member(ref member) => {
            ElementOrProperty::Property(obj.get_property(member)?)
        },
    };   

    let mut left = Slot::UNINITIALIZED;

    for op in &lvalue.post_ops[1..] {
        left = match prop {
            ElementOrProperty::Element(slot) => slot.clone(),
            ElementOrProperty::Property(prop) => prop.get(&mut interpreter.current_frame)?,
        };

        prop = match op {
            LValueCallPostOp::Index(ref index) => {
                let index = eval_expr(interpreter, index)?;
                ElementOrProperty::Element(
                    left.get_element(index.unwrap_integer().unwrap().try_into().unwrap())?,
                )
            },
            LValueCallPostOp::Member(ref member) => {
                ElementOrProperty::Property(left.get_property(member)?)
            },
        }
    };



    match prop {
        ElementOrProperty::Element(slot) => {
            slot.set(rhs);
            Completion::Value(slot.clone())
        },
        ElementOrProperty::Property(prop) => {
            prop.set(&mut interpreter.current_frame, rhs)
        }
    }
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
    for op in expr.op.iter() {
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
    for op in expr.post_ops.iter() {
        match op {
            CallPostOp::Index(index) => {
                let index_slot = eval_expr(interpreter, &index)?;
                let index = index_slot.unwrap_integer().unwrap(); // TODO: non-smi array index
                callee = callee.get_element(index.try_into().unwrap()).cloned()?;
            }
            CallPostOp::Member(member) => {
                println!("object: {:?}", callee);
                println!("member: {:?}", member);
                callee = callee.get_property(&member).unwrap().get(&mut interpreter.current_frame)?;

            }
            CallPostOp::Call(args) => {
                callee = call(interpreter, callee, args)?; // wtf use either Vec or Box<[]>
            }
        }
    }
    Completion::Value(callee)
}

fn call(interpreter: &mut Interpreter, callee: Slot, args: &Box<[Expr]>) -> Completion {
    let argument_completions = args.into_iter().map(|arg| eval_expr(interpreter, &arg));

    let mut arguments = Vec::with_capacity(argument_completions.len());

    for arg_completion in argument_completions {
        arguments.push(arg_completion?)
    }
    let result = callee.call(&mut interpreter.current_frame, &mut arguments);

    println!("call result: {:?}", result);

    match result {
        Completion::Return(slot) => Completion::Value(slot),
        Completion::ReturnEmpty => Completion::Value(Slot::new_undefined()),
        Completion::Throw(_) => result,
        _ => Completion::Value(Slot::new_undefined()),
    }
}

fn eval_variable(interpreter: &mut Interpreter, var: Variable) -> Completion {
    Completion::Value(interpreter.fetch_variable(var.index()).map(|slot| slot.clone())?)
}