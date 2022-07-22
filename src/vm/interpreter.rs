#[path = "../gen/nessie.ast.rs"]
mod ast;

#[path="./runtime.rs"]
mod runtime;

#[path="./mock.rs"]
mod mock;

use std::collections::HashSet;

// The eval_ functions in this module evaluate the static semantics of the
// Jessie AST nodes, and provide additional typing/binding information. The
// runtime contexts are provided by the runtime module, which is responsible for
// evaluating the runtime semantics of the program.

use ast::statement::*;

fn eval_statement(ctx: &impl runtime::Context, stmt: &Statement) {
    match stmt {
        Statement::VariableDeclaration(stmt) => eval_variable_declaration(ctx, stmt),
        Statement::FunctionDeclaration(stmt) => eval_function_declaration(ctx, stmt),

        Statement::BlockStatement(stmt) => eval_block(ctx, stmt),

        Statement::IfStatement(stmt) => eval_if_statement(ctx, stmt),
        Statement::ForStatement(stmt) => eval_for_statement(ctx, stmt),
        Statement::ForOfStatement(stmt) => eval_for_of_statement(ctx, stmt),
        Statement::WhileStatement(stmt) => eval_while_statement(ctx, stmt),
        Statement::SwitchStatement(stmt) => eval_switch_statement(ctx, stmt),
    
        Statement::TryStatement(stmt) => eval_try_statement(ctx, stmt),


        Statement::BreakStatement(stmt) => eval_break_statement(ctx, stmt),
        Statement::ContinueStatement(stmt) => eval_continue_statement(ctx, stmt), 
        Statement::ReturnStatement(stmt) => eval_return_statement(ctx, stmt),
        Statement::ThrowStatement(stmt) => eval_throw_statement(ctx, stmt),

        Statement::ExpressionStatement(stmt) => eval_expression(ctx, stmt.expression),
    }
}

fn early_error_variable_declaration(stmt: &ast::VariableDeclaration) {
    for decl in stmt.declarators.iter() {
        match decl.declarator {
            ast::variable_declarator::Declarator::Normal(decl) => {
                if decl.identifier == "let" || decl.identifier == "const" {
                    panic!("early error: variable declaration cannot be `let` or `const`");
                }
            },
            ast::variable_declarator::Declarator::Binding(decl) => {
                unimplemented!("asdf")
            },
        }
    }
}

#[inline]
fn eval_variable_declaration<Context: runtime::Context>(ctx: &Context, stmt: &ast::VariableDeclaration) {
    if ctx.check_early_error() {
        early_error_variable_declaration(stmt);
    }

    for decl in stmt.declarators.iter() {
        match decl.declarator {
            ast::variable_declarator::Declarator::Normal(decl) => {
                // RS: Evaluation
                let binding = ctx.resolve_binding(decl.identifier);
                let value = match decl.initializer {
                    Some(expr) => {
                        let value = eval_expression(ctx, expr);
                        if value.is_closure() {
                            ctx.set_closure_name(value, decl.identifier);
                        }
                        value
                    }
                    None => runtime::Value::Undefined,
                };
                ctx.initialize_binding(stmt.kind, binding, value)
            },
            ast::variable_declarator::Declarator::Binding(decl) => {
                unimplemented!("binding variable declarators")
            }
        }
    }
}

#[inline]
fn early_error_function_declaration(stmt: &ast::FunctionDeclaration) {
    let decl = stmt.function?;
    let mut unique_parameter_set = HashSet::with_capacity(decl.parameters.len());
    for param in decl.parameters.iter() {
        if param.identifier == "eval" || param.identifier == "arguments" {
            panic!("early error: function declaration cannot have `eval` or `arguments` as parameters");
        }

        if !unique_parameter_set.insert(param.identifier) {
            panic!("early error: function declaration cannot have duplicate parameters");
        }

        if declared_names(decl.body).contains(&param.identifier) {
            panic!("early error: function declaration cannot have a parameter that is also declared in the body");
        }

        // call to 'super' is not allowed anywhere, will be checked in identifier access
    }
}

#[inline]
fn eval_function_declaration<Context: runtime::Context>(ctx: &Context, stmt: &ast::FunctionDeclaration) {
    if ctx.check_early_error() {
        early_error_function_declaration(stmt);
    }

    
}

// https://tc39.es/ecma262/#sec-block-runtime-semantics-evaluation
//
// In Jessie, there are no var declarations, so we only need to scan the
// function declarations inside the block and hoist them. For variables,
// we can just evaluate the statements in the block linearly, and add the 
// declaration to the context when encountered.
//
// ctx.block_scope() declares a new scope for the block, and restores the previous
// scope when the block is finished. TODO: inline closure 
// Equivalent to NewDeclarativeEnvironment
fn eval_block(ctx: &impl runtime::Context, stmt: &ast::BlockStatement) {
    ctx.block_scope(|| {
        for stmt in &stmt.statements {
            eval_statement(ctx, stmt);
            
        }
    })
}

// https://tc39.es/ecma262/#sec-break-statement-runtime-semantics-evaluation
// 
// No labeled break implementation.
//
// Break statement invokes a termination signal that propagates over the ast
// and handled by the nearest enclosing loop.
fn eval_break_statement(ctx: &impl runtime::Context, stmt: &ast::BreakStatement) {
    // break_loop is a signal that the nearest enclosing loop should break.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.terminator_break();
}

// https://tc39.es/ecma262/#sec-continue-statement-runtime-semantics-evaluation
//
// No labeled continue implementation.
//
// Continue statement invokes a termination signal that propagates over the ast
// and handled by the nearest enclosing loop.
fn eval_continue_statement(ctx: &impl runtime::Context, stmt: &ast::ContinueStatement) {
    // continue_loop is a signal that the nearest enclosing loop should continue.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.terminator_continue();
}

// https://262.ecma-international.org/9.0/#sec-for-statement
// 
fn eval_for_statement(ctx: &impl runtime::Context, stmt: &ast::ForStatement) {
    ctx.scope(|| {
        match stmt.init {
            Some(init) => eval_expression(ctx, init),
            None => (),
        }
        loop {
            match stmt.test {
                Some(test) => {
                    let test_val = eval_expression(ctx, test);
                    if !test_val.truthy() {
                        break;
                    }
                }
                None => (),
            }

            for stmt in &stmt.body.statements {
                eval_statement(ctx, stmt);
                // When any of the internal statement had set completion signal,
                // for statement handles them appropriately.
                if ctx.completion_signal().is_some() {
                    break
                }
            }

            match ctx.termination_signal() {
                None => (),
                Some(runtime::CompletionSignal::Continue) => continue,
                Some(runtime::CompletionSignal::Break) => break,
                Some(runtime::CompletionSignal::Return(val)) => return,
                Some(runtime::CompletionSignal::Throw(val)) => return,
            }

            // TODO: binding

            match stmt.update {
                Some(update) => eval_expression(ctx, &update),
                None => (),
            }
        }
    }|)
}

fn eval_for_of_statement(ctx: &impl Context, stmt: &ast::ForOfStatement) {
    ctx.scope(|| {
        let iterable = eval_expression(ctx, &stmt.iterable);
        let iterator = iterable.iterator();
        let mut iterator = iterator.unwrap();
        loop {
            let next = iterator.next();
            if next.is_none() {
                break;
            }
            let next = next.unwrap();
            let next = next.value();
            let next = runtime::Value::from_js_value(next);
            ctx.set_variable(stmt.left_identifier.name, next);
            eval_statement(ctx, &stmt.body);
            match ctx.completion_signal() {
                None => (),
                Some(runtime::CompletionSignal::Continue) => continue,
                Some(runtime::CompletionSignal::Break) => break,
                Some(runtime::CompletionSignal::Return(val)) => return,
                Some(runtime::CompletionSignal::Throw(val)) => return,
            }
        }
    }|)
}

use ast::expression::*;

fn eval_expression<Context: runtime::Context>(ctx: &Context, expr: &ast::Expression) -> &Context::V {
    match expr {
        Expression::Literal(expr) => eval_literal(ctx, expr),
        Expression::Array(expr) => eval_array(ctx, expr),
        Expression::Object(expr) => eval_object(ctx, expr),
        Expression::Function(expr) => eval_function(ctx, expr),
        Expression::ArrowFunction(expr) => eval_arrow_function(ctx, expr),
        
        Expression::Binary(expr) => eval_binary(ctx, expr),
        Expression::Unary(expr) => eval_unary(ctx, expr),
        Expression::Conditional(expr) => eval_conditional(ctx, expr),
        Expression::Logical(expr) => eval_logical(ctx, expr),
        Expression::Update(expr) => eval_update(ctx, expr),
        
        Expression::Variable(expr) => eval_variable(ctx, expr),
        Expression::Assignment(expr) => eval_assignment(ctx, expr),
        Expression::Member(expr) => eval_member(ctx, expr),
        
        Expression::Call(expr) => eval_call(ctx, expr),
    }
}

use ast::literal::*;

#[inline]
fn eval_literal(ctx: &impl Context, literal: &Literal) -> impl runtime::Value {
    match literal {
        Literal::Undefined(_) => ctx.new_undefined(),
        Literal::Null(_) => ctx.new_null(),
        Literal::Boolean(literal) => runtime::MockBoolean::new(literal.value),
        Literal::Number(literal) => eval_number(literal),
        Literal::String(literal) => eval_string(literal),
        Literal::Bigint(literal) => eval_bigint(literal),
    }
}

#[inline]
fn eval_number<Context: runtime::Context>(ctx: &Context, literal: i64) -> runtime::Value {
    // TODO: sanity check on 2^53
    runtime::MockNumeric::new(literal)
}

#[inline]
fn eval_string<Context: runtime::Context>(ctx: &Context, literal: &str) -> runtime::Value {
    runtime::MockString::new(literal)
}

#[inline]
fn eval_bigint<Context: runtime::Context>(ctx: &Context, literal: &str) -> runtime::Value {
    unimplemented!(); // TODO: parse bigint
    // ctx.new_bigint(parsed_bigint)
}

#[inline]
fn eval_array<Context: runtime::Context>(ctx: &Context, arr: &ast::ArrayExpression) -> runtime::Value {
   ctx.new_array(arr.elements.iter().map(|e| eval_expression(ctx, e)).collect())
}

#[inline]
fn eval_assignment<Context: runtime::Context>(ctx: &Context, expr: &ast::AssignmentExpression) -> runtime::Value {
    match expr.operator {
        ast::assignment_expression::Operator::Assign => eval_assign(ctx, expr.left, expr.right),
        ast::assignment_expression::Operator::Add => eval_lval(ctx, expr.left).as_numeric().add(eval_expression(ctx, &expr.right).as_numeric()),
    }
}



#[inline]
fn eval_call<Context: runtime::Context>(ctx: &Context, expr: &ast::CallExpression) -> runtime::Value {
    eval_expression(ctx, expr.callee).as_closure().call(ctx, expr.arguments.iter().map(|e| eval_expression(ctx, e)).collect()))   
}

#[inline]
fn eval_conditional<Context: runtime::Context>(ctx: &Context, expr: &ast::ConditionalExpression) -> runtime::Value {
    if eval_expression(ctx, &expr.test).as_boolean().value {
        eval_expression(ctx, &expr.consequent)
    } else {
        eval_expression(ctx, &expr.alternate)
    }
}

#[inline]
fn eval_function<Context: runtime::Context>(ctx: &Context, expr: &ast::FunctionExpression) -> runtime::Value {
}

#[inline]
fn eval_identifier<Context: runtime::Context>(ctx: &Context, expr: &ast::IdentifierExpression) -> runtime::Value {
    ctx.get_variable(expr.name.as_str())
}

#[inline]
fn eval_binary<Context: runtime::Context>(ctx: &Context, expr: &ast::BinaryExpression) -> Context::V {
    let left = eval_expression(ctx, &expr.left);
    let right = eval_expression(ctx, &expr.right);
    match expr.operator {
        ast::binary_expression::ArithmeticOperator(op) => ctx.op_arithmetic(op, left, right),
        ast::binary_expression::ComparisonOperator(op) => ctx.new_boolean(ctx.op_comparison(op, left, right)),
    }
}

#[inline]
fn eval_unary<Context: runtime::Context>(ctx: &Context, expr: &ast::UnaryExpression) -> &Context::V {
    ctx.op_unary(expr.operator, eval_expression(ctx, &expr.argument))
}

#[inline]
fn eval_logical<Context: runtime::Context>(ctx: &Context, expr: &ast::LogicalExpression) -> &Context::V {
    ctx.op_logical(expr.operator, || { expr.left }, || { expr.right })
}

#[inline]
fn eval_update<Context: runtime::Context>(ctx: &Context, expr: &ast::UpdateExpression) -> &Context::V {
    ctx.op_update(expr.operator, eval_expression(ctx, &expr.argument))
}

#[inline]
fn eval_variable<Context: runtime::Context>(ctx: &Context, expr: &ast::VariableExpression) -> &Context::V {
    ctx.get_variable(expr.name.as_str())
}