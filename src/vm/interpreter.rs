#[path = "../gen/nessie.ast.rs"]
mod ast;

#[path="./runtime.rs"]
mod runtime;

#[path="./mock.rs"]
mod mock;

struct<V: > ExecutionContext {
    lexical_env: Environment,
    variable_env: Environment,
}

struct Environment {
    
}

use ast::statement::*;

fn eval_statement<Context: runtime::Context>(ctx: &Context, stmt: &Statement) {
    match stmt {
        Statement::BlockStatement(stmt) => eval_block(ctx, stmt),
        Statement::BreakStatement(stmt) => eval_break_statement(ctx, stmt),
        Statement::ContinueStatement(stmt) => eval_continue_statement(stmt),
        Statement::ExpressionStatement(stmt) => eval_expression(stmt.expression),
        Statement::ForStatement(stmt) => eval_for_statement(stmt),
        Statement::ForOfStatement(stmt) => eval_for_of_statement(stmt),
        Statement::FunctionDeclaration(stmt) => eval_function_declaration(stmt),
        Statement::IfStatement(stmt) => eval_if_statement(stmt),
        Statement::ReturnStatement(stmt) => eval_return_statement(stmt),
        Statement::SwitchStatement(stmt) => eval_switch_statement(stmt),
        Statement::ThrowStatement(stmt) => eval_throw_statement(stmt),
        Statement::TryStatement(stmt) => eval_try_statement(stmt),
        Statement::VariableDeclaration(stmt) => eval_variable_declaration(stmt),
        Statement::WhileStatement(stmt) => eval_while_statement(stmt),
    }
}

// https://tc39.es/ecma262/#sec-block-runtime-semantics-evaluation
fn eval_block<Context: runtime::Context>(ctx: &Context, stmt: &ast::BlockStatement) {
    ctx.enter_scope();

    for stmt in &stmt.statements {
        eval_statement(ctx, stmt);
    }

    ctx.exit_scope();
}

fn eval_break_statement<Context: runtime::Context>(ctx: &Context, stmt: &ast::BreakStatement) {
    ctx.break_loop(); // TODO: support label
}

fn eval_continue_statement<Context: runtime::Context>(ctx: &Context, stmt: &ast::ContinueStatement) {
    ctx.continue_loop(); // TODO: support label
}

fn eval_for_statement<Context: runtime::Context>(stmt: &ast::ForStatement) {
    unimplemented!()
}

fn eval_expression<Context: runtime::Context>(ctx: &Context, expr: &ast::Statement) {
    match expr {
        ast::Literal(expr) => eval_literal(ctx, expr),
        ast::Array(expr) => eval_array(ctx, expr),
        ast::Assignment(expr) => eval_assignment(ctx, expr),
        ast::Binary(expr) => eval_binary(ctx, expr),
        ast::Call(expr) => eval_call(ctx, expr),
        ast::Conditional(expr) => eval_conditional(ctx, expr),
        ast::Function(expr) => eval_function(ctx, expr),
        ast::Identifier(expr) => eval_identifier(ctx, expr),
        ast::Logical(expr) => eval_logical(ctx, expr),
        ast::Member(expr) => eval_member(ctx, expr),
        ast::Object(expr) => eval_object(ctx, expr),
        ast::Unary(expr) => eval_unary(ctx, expr),
        ast::Update(expr) => eval_update(ctx, expr),
        ast::ArrowFunction(expr) => eval_arrow_function(ctx, expr),
    }
}

use ast::literal::*;

#[inline]
fn eval_literal<Context: runtime::Context>(ctx: &Context, literal: &Literal) -> runtime::Value {
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
fn eval_binary<Context: runtime::Context>(ctx: &Context, expr: &ast::BinaryExpression) -> runtime::Value {
    match expr.operator {
        ast::binary_expression::Operator::Add => eval_expression(ctx, expr.left).clone().as_numeric().add(eval_expression(ctx, &expr.right).as_numeric()),
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