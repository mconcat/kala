use kala_ast::ast;
use crate::context::{InterpreterContext, CompletionSignal};


pub fn eval_statement(ctx: &mut InterpreterContext, stmt: &mut ast::Statement) {
    match stmt {
        ast::Statement::VariableDeclaration(stmt) => eval_variable_declaration(ctx, stmt),
        ast::Statement::FunctionDeclaration(stmt) => eval_function_declaration(ctx, stmt),

        ast::Statement::Block(stmt) => eval_block(ctx, stmt),

        ast::Statement::If(stmt) => eval_if(ctx, stmt),
        ast::Statement::For(stmt) => eval_for(ctx, stmt),
        ast::Statement::ForOf(stmt) => eval_for_of(ctx, stmt),
        ast::Statement::While(stmt) => eval_while(ctx, stmt),
        ast::Statement::Switch(stmt) => eval_switch(ctx, stmt),

        ast::Statement::Try(stmt) => eval_try(ctx, stmt),

        ast::Statement::Break(stmt) => eval_break(ctx, stmt),
        ast::Statement::Continue(stmt) => eval_continue(ctx, stmt),
        ast::Statement::Return(stmt) => eval_return(ctx, stmt),
        ast::Statement::Throw(stmt) => eval_throw(ctx, stmt),

        ast::Statement::Expression(stmt) => eval_expression(ctx, stmt),
    }
}

pub fn eval_variable_declaration(ctx: &mut InterpreterContext, stmt: &mut ast::VariableDeclaration) {
    for decl in stmt.declarators {
        let value = match decl.init {
            Some(expr) => eval_expression(ctx, expr),
            None => Value::Undefined,
        };
        declare_binding(ctx, &decl.binding, value);
    }
}

pub fn eval_block(ctx: &mut InterpreterContext, block: &mut ast::BlockStatement) {
    ctx.enter_scope();
    for stmt in block.statements.iter_mut() {
        eval_statement(ctx, stmt);
    }
    ctx.exit_scope();
}

pub fn eval_if(ctx: &mut InterpreterContext, stmt: &mut ast::IfStatement) {
    let cond = eval_expression(ctx, &mut stmt.condition);
    ctx.enter_scope();
    if cond.is_truthy() {
        eval_statement(ctx, &mut stmt.consequent);
    } else if let Some(alt) = &mut stmt.alternate {
        eval_statement(ctx, alt);
    }
    ctx.exit_scope();
}

pub fn eval_for(ctx: &mut InterpreterContext, stmt: &mut ast::ForStatement) {
    if let Some(init) = &mut stmt.init {
        eval_statement(ctx, init);
    }
    ctx.enter_for_scope();
    loop {
        ctx.loop_scope();
        if let Some(cond) = &mut stmt.condition {
            let cond = eval_expression(ctx, cond);
            if !cond.is_truthy() {
                break;
            }
        }

        eval_statement(ctx, &mut stmt.body);

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

pub fn eval_for_of(ctx: &mut InterpreterContext, stmt: &mut ast::ForOfStatement) {
    let iterable = eval_expression(ctx, &mut stmt.decl.init.expect("for-of must have init"));
   
    ctx.enter_scope();
    for item in iterable.iter() {
        declare_binding(ctx, stmt.kind, stmt.decl.binding, item);
        eval_statement(ctx, &mut stmt.body);

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
                CompletionSignal::Throw => {
                    return // will be handled inside try-catch clause
                },
            }        
        }
    }    
    ctx.exit_scope();
}

pub fn eval_while(ctx: &mut InterpreterContext, stmt: &mut ast::WhileStatement) {
    ctx.enter_scope();
    loop {
        let cond = eval_expression(ctx, &mut stmt.condition);
        if !cond.is_truthy() {
            break;
        }

        eval_statement(ctx, &mut stmt.body);

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
                CompletionSignal::Throw => {
                    return // will be handled inside try-catch clause
                },
            }        
        }
    }
    ctx.exit_scope();
}

pub fn eval_switch(ctx: &mut InterpreterContext, stmt: &mut ast::SwitchStatement) {
    let discriminant = eval_expression(ctx, &mut stmt.discriminant);
    ctx.enter_switch_scope();
    for case in stmt.cases.iter_mut() {
        if case.test.is_none() || discriminant == eval_expression(ctx, case.test.as_mut().unwrap()) {
            for stmt in case.consequent.iter_mut() {
                eval_statement(ctx, stmt);
            }
            break;
        }
    }
    ctx.exit_switch_scope();
}



pub fn eval_expression(ctx: &mut InterpreterContext, expr: &mut ast::Expression) -> JSValue {
    match expr {
        ast::Expression::Literal(lit) => eval_literal(ctx, lit),
        ast::Expression::Array(arr) => eval_array(ctx, arr),
        ast::Expression::Object(obj) => eval_object(ctx, obj),
        ast::Expression::Variable(ident) => eval_variable(ctx, ident),
        ast::Expression::Binary(bin) => eval_binary(ctx, bin),
        ast::Expression::Unary(unary) => eval_unary(ctx, unary),
        ast::Expression::Conditional(cond) => eval_conditional(ctx, cond),
        ast::Expression::Logical(logical) => eval_logical(ctx, logical),
        ast::Expression::Call(call) => eval_call(ctx, call),
        ast::Expression::Update(update) => eval_update(ctx, update),
        ast::Expression::Member(index) => eval_index(ctx, index),
        ast::Expression::Assignment(assign) => eval_assignment(ctx, assign),
        ast::Expression::Function(func) => eval_function(ctx, func),
        ast::Expression::ArrowFunction(func) => eval_arrow_function(ctx, func),
        ast::Expression::Parenthesized(paren) => eval_parenthesized(ctx, paren),
    }
}

#[inline]
pub fn eval_literal(ctx: &mut InterpreterContext, lit: &mut ast::Literal) -> JSValue {
    match lit {
        ast::Literal::Undefined => JSValue::Undefined,
        ast::Literal::Null => JSValue::Null,
        ast::Literal::Boolean(b) => JSValue::Boolean(*b),
        ast::Literal::Number(n) => JSValue::Number(*n),
        ast::Literal::String(s) => JSValue::String(s.clone()),
        ast::Literal::Bigint(_) => unimplemented!(),
    }
}

#[inline]
pub fn eval_variable(ctx: &mut InterpreterContext, ident: &mut ast::Identifier) -> JSValue {
    ctx.get_binding_value(&ident.name)
}

#[inline]
pub fn eval_binary(ctx: &mut InterpreterContext, bin: &mut ast::BinaryExpression) -> JSValue {
    let left = eval_expression(ctx, &mut bin.left);
    let right = eval_expression(ctx, &mut bin.right);
    match bin.op {
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
        ast::BinaryOperator::StrictEqual => left.strict_equal(right),
        ast::BinaryOperator::StrictNotEqual => left.strict_not_equal(right),
        ast::BinaryOperator::LessThan => left.less_than(right),
        ast::BinaryOperator::LessThanEqual => left.less_than_equal(right),
        ast::BinaryOperator::GreaterThan => left.greater_than(right),
        ast::BinaryOperator::GreaterThanEqual => left.greater_than_equal(right),
    }
}

pub fn eval_unary(ctx: &mut InterpreterContext, unary: &mut ast::UnaryExpression) -> JSValue {
    let operand = eval_expression(ctx, &mut unary.operand);
    match unary.op {
        ast::UnaryOperator::Minus => operand.negate(),
        ast::UnaryOperator::Plus => operand.positive(),
        ast::UnaryOperator::Tilde => operand.bit_not(),
        ast::UnaryOperator::TypeOf => operand.type_of(),
        ast::UnaryOperator::Void => JSValue::Undefined,
        ast::UnaryOperator::Bang => operand.not(),
    }
}

pub fn eval_call(ctx: &mut InterpreterContext, call: &mut ast::CallExpression) -> JSValue {
    let callee = eval_expression(ctx, &mut call.callee);
    let mut args = Vec::new();
    for arg in call.arguments.iter_mut() {
        args.push(eval_expression(ctx, arg));
    }
    callee.call(ctx, args)
}

pub fn eval_index(ctx: &mut InterpreterContext, index: &mut ast::IndexExpression) -> JSValue {
    let base = eval_expression(ctx, &mut index.base);
    let property = eval_expression(ctx, &mut index.property);
    base.get(ctx, property)
}

pub fn eval_assign(ctx: &mut InterpreterContext, assign: &mut ast::AssignExpression) -> JSValue {
    let value = eval_expression(ctx, &mut assign.value);
    match &mut assign.left {
        ast::AssignLeft::Identifier(ident) => {
            ctx.set_binding_value(&ident.name, value.clone());
        }
        ast::AssignLeft::Index(index) => {
            let base = eval_expression(ctx, &mut index.base);
            let property = eval_expression(ctx, &mut index.property);
            base.set(ctx, property, value.clone());
        }
    }
    value
}
