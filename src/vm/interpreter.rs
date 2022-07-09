#[path = "../gen/nessie.ast.rs"]
mod ast;

struct ExecutionContext {
    lexical_env: Environment,
    variable_env: Environment,
}

struct Environment {
    
}

fn eval_statement(stmt: &ast::Statement) {
    match stmt {
        ast::BlockStatement(stmt) => eval_block_statement(stmt),
        ast::BreakStatement(stmt) => eval_break_statement(stmt),
        ast::ContinueStatement(stmt) => eval_continue_statement(stmt),
        ast::ExpressionStatement(stmt) => eval_expression_statement(stmt),
        ast::ForStatement(stmt) => eval_for_statement(stmt),
        ast::ForOfStatement(stmt) => eval_for_of_statement(stmt),
        ast::FunctionDeclaration(stmt) => eval_function_declaration(stmt),
        ast::IfStatement(stmt) => eval_if_statement(stmt),
        ast::ReturnStatement(stmt) => eval_return_statement(stmt),
        ast::SwitchStatement(stmt) => eval_switch_statement(stmt),
        ast::ThrowStatement(stmt) => eval_throw_statement(stmt),
        ast::TryStatement(stmt) => eval_try_statement(stmt),
        ast::VariableDeclaration(stmt) => eval_variable_declaration(stmt),
        ast::WhileStatement(stmt) => eval_while_statement(stmt),
    }
}

// https://tc39.es/ecma262/#sec-block-runtime-semantics-evaluation
fn eval_block_statement(stmt: &ast::BlockStatement) {
    // create a new scope to the execution environment

    for stmt in &stmt.statements {
        eval_statement(stmt);
    }
}

fn eval_expression(expr: &ast::Statement) {
    match expr {
        ast::Literal(expr) => eval_literal(expr),
        ast::Array(expr) => eval_array(expr),
        ast::Assignment(expr) => eval_assignment(expr),
        ast::Binary(expr) => eval_binary(expr),
        ast::Call(expr) => eval_call(expr),
        ast::Conditional(expr) => eval_conditional(expr),
        ast::Function(expr) => eval_function(expr),
        ast::Identifier(expr) => eval_identifier(expr),
        ast::Logical(expr) => eval_logical(expr),
        ast::Member(expr) => eval_member(expr),
        ast::Object(expr) => eval_object(expr),
        ast::Unary(expr) => eval_unary(expr),
        ast::Update(expr) => eval_update(expr),
        ast::ArrowFunction(expr) => eval_arrow_function(expr),
    }
}

fn eval_literal(literal: &ast::Literal) {
    match literal {
        ast::Boolean(literal) => eval_boolean(literal),
        ast::Null(literal) => eval_null(literal),
        ast::Number(literal) => eval_number(literal),
        ast::String(literal) => eval_string(literal),
        ast::Bigint(literal) => eval_bigint(literal),
    }
}

