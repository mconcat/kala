use std::borrow::BorrowMut;

use kala_ast::ast::{self, FunctionExpression, NodeF, ParameterElement};
use kala_context::environment_record::EnvironmentRecord;
use crate::context::{InterpreterContext, CompletionSignal};
use crate::declare::{*, self};
use crate::value::JSValue;
use crate::lexical::{InterpreterF as F, Identifier};
use crate::lexical;
/*
pub fn eval_statement(ctx: &mut InterpreterContext, stmt: &mut lexical::Statement) {
    match &mut stmt {
        ast::Statement::VariableDeclaration(stmt) => eval_variable_declaration(ctx, stmt),
        ast::Statement::FunctionDeclaration(stmt) => eval_function_declaration(ctx, stmt),

        ast::Statement::Block(stmt) => eval_block(ctx, stmt),

        ast::Statement::If(stmt) => eval_if(ctx, stmt),
        ast::Statement::For(stmt) => eval_for(ctx, stmt),
        ast::Statement::ForOf(stmt) => eval_for_of(ctx, stmt),
        ast::Statement::While(stmt) => eval_while(ctx, stmt),
        ast::Statement::Switch(stmt) => unimplemented!(), // eval_switch(ctx, stmt),

        ast::Statement::Try(stmt) =>  unimplemented!(),// eval_try(ctx, stmt),

        ast::Statement::Break(stmt) => eval_break(ctx, stmt),
        ast::Statement::Continue(stmt) => eval_continue(ctx, stmt),
        ast::Statement::Return(stmt) => eval_return(ctx, stmt),
        ast::Statement::Throw(stmt) => unimplemented!(), // eval_throw(ctx, stmt),

        ast::Statement::Expression(stmt) => { eval_expression(ctx, &mut stmt.expression); },
    }
}
*/
pub fn eval_variable_declaration(ctx: &mut InterpreterContext, stmt: &mut ast::VariableDeclaration<F>) {
    for decl in &mut stmt.declarators {
        if let Some(expr) = &mut decl.init {
            let value = eval_expression(ctx, expr);
            if value.is_none() {
                unimplemented!("TODO: handle error")
            }
            declare_binding(ctx, &stmt.kind, &decl.binding, &value.unwrap());
        } else {
            declare_binding(ctx, &stmt.kind, &decl.binding, &JSValue::undefined());
        }
    }
}

pub fn eval_function_declaration(ctx: &mut InterpreterContext, stmt: &mut ast::FunctionDeclaration<F>) {
    let function_env = EnvironmentRecord::new(); 
    // TODO: add captured variables to function_env
    let value = JSValue::function(function_env, stmt.function.clone());
    declare_binding_identifier(ctx, &ast::DeclarationKind::Const, &stmt.function.function.name.as_ref().unwrap(), &value);
}

pub fn eval_block(ctx: &mut InterpreterContext, block: &mut ast::BlockStatement<F>) {
    ctx.enter_scope();
    for stmt in block.body.iter_mut() {
        F::eval_statement(ctx, stmt);
    }
    ctx.exit_scope();
}

pub fn eval_if(ctx: &mut InterpreterContext, stmt: &mut ast::IfStatement<F>) {
    let cond = eval_expression(ctx, &mut stmt.test);
    if cond.is_none() {
        unimplemented!("TODO: handle error")
    }
    let cond = cond.unwrap();
    ctx.enter_scope();
    if cond.is_truthy() {
        F::eval_statement(ctx, &mut stmt.consequent);
    } else if let Some(alt) = &mut stmt.alternate {
        F::eval_statement(ctx, alt);
    }
    ctx.exit_scope();
}

pub fn eval_for(ctx: &mut InterpreterContext, stmt: &mut ast::ForStatement<F>) {
    if let Some(init) = &mut stmt.init {
        unimplemented!("for init");
    }
    ctx.enter_for_scope();
    loop {
        ctx.loop_scope();
        if let Some(cond) = &mut stmt.test {
            let cond = F::eval_expression(ctx, cond);
            if cond.is_none() {
                ctx.exit_for_scope();
                return; // TODO: return error
            }
            if !cond.unwrap().is_truthy() {
                break;
            }
        }

        F::eval_statement(ctx, &mut stmt.body);

        if let Some(completion) = ctx.completion_signal() {
            match completion {
                CompletionSignal::Break => {
                    ctx.clear_completion_signal();
                    break
                },
                CompletionSignal::Continue => {
                    ctx.clear_completion_signal();
                    continue
                },
                CompletionSignal::Return => {
                    return
                },
                CompletionSignal::ReturnValue(_) => {
                    return
                }
                CompletionSignal::Throw(_) => {
                    return // will be handled inside try-catch clause
                },
            }        
        }

        if let Some(update) = &mut stmt.update {
            eval_expression(ctx, update);
        }
    }
    ctx.exit_for_scope();
}

pub fn eval_for_of(ctx: &mut InterpreterContext, stmt: &mut ast::ForOfStatement<F>) {
    unimplemented!()
    /*
    let iterable = eval_expression(ctx, &mut stmt.decl.init.expect("for-of must have init"));
   
    ctx.enter_scope();
    for item in iterable.iter() {
        declare_binding(ctx, stmt.kind, &stmt.decl.binding, item);
        F::eval_statement(ctx, &mut stmt.body);

        if let Some(completion) = ctx.completion_signal() {
            match completion {
                CompletionSignal::Break => {
                    ctx.clear_completion_signal();
                    break
                },
                CompletionSignal::Continue => {
                    ctx.clear_completion_signal();
                    continue
                },
                CompletionSignal::Return => {
                    return
                },
                CompletionSignal::ReturnValue(val) => {
                    return
                }
                CompletionSignal::Throw(_) => {
                    return // will be handled inside try-catch clause
                },
            }        
        }
    }    
    ctx.exit_scope();
    */
}

pub fn eval_while(ctx: &mut InterpreterContext, stmt: &mut ast::WhileStatement<F>) {
    ctx.enter_scope();
    loop {
        let cond = F::eval_expression(ctx, &mut stmt.test).unwrap(); // TODO: handle error
        if !cond.is_truthy() {
            break;
        }

        F::eval_statement(ctx, &mut stmt.body);

        if let Some(completion) = ctx.completion_signal() {
            match completion {
                CompletionSignal::Break => {
                    ctx.clear_completion_signal();
                    break
                },
                CompletionSignal::Continue => {
                    ctx.clear_completion_signal();
                    continue
                },
                CompletionSignal::Return => {
                    return
                },
                CompletionSignal::ReturnValue(_) => {
                    return
                }
                CompletionSignal::Throw(_) => {
                    return // will be handled inside try-catch clause
                },
            }        
        }
    }
    ctx.exit_scope();
}

pub fn eval_switch(ctx: &mut InterpreterContext, stmt: &mut ast::SwitchStatement<F>) {
    unimplemented!()
    /* 
    let discriminant = eval_expression(ctx, &mut stmt.discriminant);
    ctx.enter_switch_scope();
    for case in stmt.cases.iter_mut() {
        if case.test.is_none() || discriminant == F::eval_expression(ctx, case.test.as_mut().unwrap()) {
            for stmt in case.consequent.iter_mut() {
                F::eval_statement(ctx, stmt);
            }
            break;
        }
    }
    ctx.exit_switch_scope();
    */
}

pub fn eval_break(ctx: &mut InterpreterContext, stmt: &mut ast::BreakStatement) {
    ctx.termination_break()
}

pub fn eval_continue(ctx: &mut InterpreterContext, stmt: &mut ast::ContinueStatement) {
    ctx.termination_continue()
}

pub fn eval_return(ctx: &mut InterpreterContext, stmt: &mut ast::ReturnStatement<F>) {
    let val = stmt.argument.as_mut().map(|x| eval_expression(ctx, x).unwrap()); // TODO: handle error
    ctx.termination_return(&val)
}

pub fn eval_throw(ctx: &mut InterpreterContext, stmt: &mut ast::ThrowStatement<F>) {
    let val = eval_expression(ctx, &mut stmt.argument).unwrap(); // TODO: handle error
    ctx.termination_throw(&val)
}

pub fn eval_expression(ctx: &mut InterpreterContext, expr: &mut <F as NodeF>::Expression) -> Option<JSValue> {
    match &mut expr.expression {
        ast::Expression::Literal(lit) => Some(eval_literal(ctx, lit)),
        ast::Expression::Array(arr) => eval_array(ctx, arr),
        ast::Expression::Object(obj) => eval_object(ctx, obj),
        ast::Expression::Variable(ident) => eval_variable(ctx, &mut ident.name),
        ast::Expression::Binary(bin) => eval_binary(ctx, bin),
        ast::Expression::Unary(unary) => eval_unary(ctx, unary),
        ast::Expression::Conditional(cond) => eval_conditional(ctx, cond),
        ast::Expression::Logical(logical) => eval_logical(ctx, logical),
        ast::Expression::Call(call) => eval_call(ctx, call),
        ast::Expression::Update(update) => eval_update(ctx, update),
        ast::Expression::Member(index) => eval_member(ctx, index),
        ast::Expression::Assignment(assign) => eval_assignment(ctx, assign),
        ast::Expression::Function(func) => eval_function(ctx, func),
        ast::Expression::ArrowFunction(func) => eval_arrow_function(ctx, func),
        ast::Expression::Parenthesized(paren) => eval_expression(ctx, &mut paren.expression),
    }
}

#[inline]
pub fn eval_literal(ctx: &mut InterpreterContext, lit: &mut ast::Literal) -> JSValue {
    match lit {
        ast::Literal::Undefined => JSValue::Undefined,
        ast::Literal::Boolean(b) => JSValue::boolean(b.value),
        ast::Literal::Number(n) => JSValue::number(n.value as i32), // TODO
        ast::Literal::String(s) => JSValue::string(s.value.clone()),
    }
}

#[inline]
pub fn eval_array(ctx: &mut InterpreterContext, arr: &mut ast::ArrayExpression<F>) -> Option<JSValue> {
    let mut elements = Vec::new();
    for element in arr.elements.iter_mut() {
        match element {
            ast::ParameterElement::Parameter(param) => {
                elements.push(F::eval_expression(ctx, param)?);
            }
            _ => unimplemented!(),
        }
    }
    Some(JSValue::array(elements))
}

#[inline]
pub fn eval_object(ctx: &mut InterpreterContext, obj: &mut ast::ObjectExpression<F>) -> Option<JSValue> {
    let mut properties = Vec::new();
    for property in obj.properties.iter_mut() {
        match property {
            ast::ObjectElement::KeyValue(key, value) => properties.push((key.clone(), F::eval_expression(ctx, value)?)),
            ast::ObjectElement::Shorthand(key) => properties.push((key.clone(), eval_variable(ctx, key)?)),
            _ => unimplemented!()
        }
    };

    Some(JSValue::object(properties))
}

#[inline]
pub fn eval_variable(ctx: &mut InterpreterContext, ident: &mut lexical::Identifier) -> Option<JSValue> {
    ctx.get_binding_value(&ident)
}

#[inline]
pub fn eval_binary(ctx: &mut InterpreterContext, bin: &mut ast::BinaryExpression<F>) -> Option<JSValue> {
    // eval for all types for strict (non) equal operation
    match bin.operator {
        ast::BinaryOperator::StrictEqual => {
            let left = F::eval_expression(ctx, &mut bin.left)?;
            let right = F::eval_expression(ctx, &mut bin.right)?;
            return Some(JSValue::boolean(left.equal(&right)))
        },
        ast::BinaryOperator::StrictNotEqual => {
            let left = F::eval_expression(ctx, &mut bin.left)?;
            let right = F::eval_expression(ctx, &mut bin.right)?;
            return Some(JSValue::boolean(!left.equal(&right)))
        },
        _ => {},
    }

    // eval for only number type for anything else

    let mut operand = F::eval_expression(ctx, &mut bin.left)?;
    
    let mut left = operand.as_mut_number();
    if left.is_none() {
        // TODO: throw error
        return None
    }
    let mut left = left.unwrap();

    let mut argument = F::eval_expression(ctx, &mut bin.right)?;

    let right = argument.as_mut_number();
    if right.is_none() {
        // TODO: throw error
        return None
    }
    let mut right = right.unwrap();

    match bin.operator {
        // arithmetic
        ast::BinaryOperator::Add => left.add(right),
        ast::BinaryOperator::Sub => left.sub(right),
        ast::BinaryOperator::Mul => left.mul(right),
        ast::BinaryOperator::Div => left.div(right),
        ast::BinaryOperator::Pow => left.pow(right),
        ast::BinaryOperator::Mod => left.modulo(right),
        // bitwise operations
        ast::BinaryOperator::BitAnd => left.bit_and(right),
        ast::BinaryOperator::BitOr => left.bit_or(right),
        ast::BinaryOperator::BitXor => left.bit_xor(right),
        ast::BinaryOperator::LeftShift => left.left_shift(right),
        ast::BinaryOperator::RightShift => left.right_shift(right),
        ast::BinaryOperator::UnsignedRightShift => left.unsigned_right_shift(right),
        // comparisons
        ast::BinaryOperator::StrictEqual => unreachable!(),
        ast::BinaryOperator::StrictNotEqual => unreachable!(),
        ast::BinaryOperator::LessThan => return Some(JSValue::boolean(left.less_than(right))),
        ast::BinaryOperator::LessThanEqual => return Some(JSValue::boolean(left.less_than_equal(right))),
        ast::BinaryOperator::GreaterThan => return Some(JSValue::boolean(left.greater_than(right))),
        ast::BinaryOperator::GreaterThanEqual => return Some(JSValue::boolean(left.greater_than_equal(right))),
    };

    Some(operand)
}

pub fn eval_unary(ctx: &mut InterpreterContext, unary: &mut ast::UnaryExpression<F>) -> Option<JSValue> {
    unimplemented!()
    /*
    let mut operand = eval_expression(ctx, &mut unary.operand);
    match unary.operator {
        ast::UnaryOperator::Minus => operand.negate(),
        ast::UnaryOperator::Plus => operand.positive(),
        ast::UnaryOperator::Tilde => operand.bit_not(),
        ast::UnaryOperator::TypeOf => operand.type_of(),
        ast::UnaryOperator::Void => JSValue::Undefined,
        ast::UnaryOperator::Bang => operand.not(),
    }
    */
}



pub fn eval_assignment(ctx: &mut InterpreterContext, expr: &mut ast::AssignmentExpression<F>) -> Option<JSValue> {
    let value = eval_expression(ctx, &mut expr.right)?;
    match &mut expr.left {
        ast::LValue::Variable(ident) => {
            ctx.set_binding_value(&ident, &value);
        }
        ast::LValue::Member(index) => {
            let mut object = F::eval_expression(ctx, &mut index.object)?;
            let property = match &mut index.property {
                ast::Member::Property(ident) => ident.clone(), 
                ast::Member::Computed(expr) => {
                    let value = F::eval_expression(ctx, expr)?.to_string();
                    Identifier::new(value)
                }
            };
            object.set_property(&property, value.clone());
        }
    }
    Some(value)
}

pub fn eval_conditional(ctx: &mut InterpreterContext, expr: &mut ast::ConditionalExpression<F>) -> Option<JSValue> {
    let discriminant = F::eval_expression(ctx, &mut expr.test)?;
    if discriminant.is_truthy() {
        eval_expression(ctx, &mut expr.consequent)
    } else {
        eval_expression(ctx, &mut expr.alternate)
    }
}

pub fn eval_logical(ctx: &mut InterpreterContext, expr: &mut ast::LogicalExpression<F>) -> Option<JSValue> {
    match expr.operator {
        ast::LogicalOperator::And => {
            let left = F::eval_expression(ctx, &mut expr.left)?;
            if !left.is_truthy() {
                Some(left)
            } else {
                F::eval_expression(ctx, &mut expr.right)
            }
        },
        ast::LogicalOperator::Or => {
            let left = eval_expression(ctx, &mut expr.left)?;
            if left.is_truthy() {
                Some(left)
            } else {
                F::eval_expression(ctx, &mut expr.right)
            }
        },
        ast::LogicalOperator::Coalesce => {
            let left = eval_expression(ctx, &mut expr.left)?;
            if !left.is_undefined() {
                Some(left)
            } else {
                F::eval_expression(ctx, &mut expr.right)
            }
        },
    }
}

pub fn eval_call(ctx: &mut InterpreterContext, call: &mut ast::CallExpression<F>) -> Option<JSValue> {
    let mut callee = F::eval_expression(ctx, &mut call.callee)?;
    let mut args = Vec::new();
    for arg in call.arguments.iter_mut() {
        match arg {
            ParameterElement::Parameter(expr) => {
                args.push(F::eval_expression(ctx, expr)?);
            },
            _ => unimplemented!(),
        }
    }
    callee.call(ctx, args)
}

pub fn eval_update(ctx: &mut InterpreterContext, expr: &mut ast::UpdateExpression<F>) -> Option<JSValue> {
    unimplemented!()
}

pub fn eval_member(ctx: &mut InterpreterContext, expr: &mut ast::MemberExpression<F>) -> Option<JSValue> {
    let object = F::eval_expression(ctx, &mut expr.object)?;
    let property = match &mut expr.property {
        ast::Member::Property(ident) => ident.clone(),
        ast::Member::Computed(expr) => {
            let value = F::eval_expression(ctx, expr)?.to_string();
            Identifier::new(value)
        }
    };
    object.get_property(&property)
}

pub fn eval_function(ctx: &mut InterpreterContext, expr: &mut ast::FunctionExpression<F>) -> Option<JSValue> {
    unimplemented!()
}

pub fn eval_arrow_function(ctx: &mut InterpreterContext, expr: &mut ast::ArrowFunctionExpression<F>) -> Option<JSValue> {
    unimplemented!()
}