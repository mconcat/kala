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

fn eval_statement<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &Statement) {
    match stmt {
        Statement::VariableDeclaration(stmt) => eval_variable_declaration(ctx, stmt),
        // Function declarations are hoisted to the top of the lexical scope.
        // When the declaration statement is actually met, noop.
        Statement::FunctionDeclaration(stmt) => (),

        Statement::BlockStatement(stmt) => eval_block_statement(ctx, stmt),

        Statement::IfStatement(stmt) => eval_if_statement(ctx, stmt),
        // Statement::ForStatement(stmt) => eval_for_statement(ctx, stmt),
        // Statement::ForOfStatement(stmt) => eval_for_of_statement(ctx, stmt),
        Statement::WhileStatement(stmt) => eval_while_statement(ctx, stmt),
        // Statement::SwitchStatement(stmt) => eval_switch_statement(ctx, stmt),
    
        // Statement::TryStatement(stmt) => eval_try_statement(ctx, stmt),


        Statement::BreakStatement(stmt) => eval_break_statement(ctx, stmt),
        Statement::ContinueStatement(stmt) => eval_continue_statement(ctx, stmt), 
        Statement::ReturnStatement(stmt) => eval_return_statement(ctx, stmt),
        // Statement::ThrowStatement(stmt) => eval_throw_statement(ctx, stmt),

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
fn eval_variable_declaration<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &ast::VariableDeclaration) {
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



// hoist_function_declaration is called when a function declaration is
// encountered in the lexical scope, at the time of evaluating parent statements.
// It creates and adds the function objects to the current context.
#[inline]
fn hoist_function_declaration<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &ast::FunctionDeclaration) {
    if ctx.check_early_error() {
        early_error_function_declaration(stmt);
    }

    // capture variables
    // within the function body, the free variables(excluding those declared as parameters or variables) are captured.
    let captured = ctx.extract_free_variables(free_variable_function_declaration(HashSet::new(), stmt));

    let function_object = ctx.new_function(stmt.identifier, stmt.function?.parameters?, stmt.function?.body?, captured);
    ctx.add_function(stmt.identifier, function_object);
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
fn eval_block_statement(ctx: &impl runtime::JSContext, stmt: &ast::BlockStatement) {
    ctx.block_scope(|| {
        for stmt in stmt.statemets.iter() {
            if let Statement::FunctionDeclaration(stmt) = stmt {
                hoist_function_declaration(ctx, stmt);
            }
        }

        for stmt in stmt.statemets.iter() {
            eval_statement(ctx, stmt);
        }
    });
}

#[inline]
fn eval_if_statement<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &ast::IfStatement) {
    ctx.control_branch(|| eval_expression(ctx, stmt.test), 
        || eval_statement(ctx, stmt.consequence), 
        || eval_statement(ctx, stmt.alternate?),
    )
}

// https://tc39.es/ecma262/#sec-break-statement-runtime-semantics-evaluation
// 
// No labeled break implementation.
//
// Break statement invokes a termination signal that propagates over the ast
// and handled by the nearest enclosing loop.
fn eval_break_statement(ctx: &impl runtime::JSContext, stmt: &ast::BreakStatement) {
    // break_loop is a signal that the nearest enclosing loop should break.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.complete_break();
}

// https://tc39.es/ecma262/#sec-continue-statement-runtime-semantics-evaluation
//
// No labeled continue implementation.
//
// Continue statement invokes a termination signal that propagates over the ast
// and handled by the nearest enclosing loop.
fn eval_continue_statement(ctx: &impl runtime::JSContext, stmt: &ast::ContinueStatement) {
    // continue_loop is a signal that the nearest enclosing loop should continue.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.complete_continue();
}

fn eval_return_statement<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &ast::ReturnStatement) {
    let value = match stmt.argument {
        Some(expr) => eval_expression(ctx, &expr),
        None => runtime::Value::Undefined,
    };
    ctx.complete_return(value);
}


// https://262.ecma-international.org/9.0/#sec-for-statement
// 
/*
fn eval_for_statement(ctx: &impl runtime::JSContext, stmt: &ast::ForStatement) {
    ctx.scope(|| {
        match stmt.init {
            Some(init) => eval_expression(ctx, init),
            None => (),
        }
        loop {
            match stmt.test {
                Some(test) => {
                    let test_val = eval_expression(ctx, stmt.test);
                    if !test_val.truthy() {
                        if ctx.for_statement_test_falsy() {
                            break;
                        }
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

            // TODO: forwarded binding

            match stmt.update {
                Some(update) => eval_expression(ctx, &update),
                None => (),
            }
        }
    }|)
}

fn eval_for_of_statement<JSContext: runtime::JSContext>(ctx: &JSContext, stmt: &ast::ForOfStatement) {
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
*/

fn eval_while_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::WhileStatement) {
    ctx.control_loop(
        || { eval_expression(ctx, stmt.condition?) }, 
        || { eval_statement(ctx, stmt.body) },
  )
}

use ast::expression::*;

fn eval_expression<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::Expression) -> &JSContext::V {
    match expr {
        Expression::Literal(expr) => eval_literal(ctx, expr),
        Expression::Array(expr) => eval_array(ctx, expr),
        Expression::Object(expr) => eval_object(ctx, expr),
        Expression::Function(expr) => eval_function(ctx, expr),
        Expression::ArrowFunction(expr) => eval_arrow_function(ctx, expr),
        
        Expression::Binary(expr) => eval_binary(ctx, expr),
        // Expression::Unary(expr) => eval_unary(ctx, expr),
        Expression::Conditional(expr) => eval_conditional(ctx, expr),
        Expression::Logical(expr) => eval_logical(ctx, expr),
        // Expression::Update(expr) => eval_update(ctx, expr),
        
        Expression::Variable(expr) => eval_variable(ctx, expr),
        Expression::Assignment(expr) => eval_assignment(ctx, expr),
        Expression::Member(expr) => eval_member(ctx, expr),
        
        Expression::Call(expr) => eval_call(ctx, expr),
    }
}

use ast::literal::*;

#[inline]
fn eval_literal<JSContext: runtime::JSContext>(ctx: &JSContext, literal: &Literal) -> &JSContext::V {
    match literal {
        Literal::Undefined(_) => ctx.new_undefined(),
        Literal::Null(_) => ctx.new_null(),
        Literal::Boolean(literal) => ctx.new_boolean(literal.value),
        Literal::Number(literal) => ctx.new_number(literal),
        Literal::String(literal) => ctx.new_string(literal),
        Literal::Bigint(literal) => ctx.new_bigint(literal),
    }
}

#[inline]
fn eval_number<JSContext: runtime::JSContext>(ctx: &JSContext, literal: i64) -> &JSContext::V  {
    // TODO: sanity check on 2^53
    runtime::MockNumeric::new(literal)
}

#[inline]
fn eval_string<JSContext: runtime::JSContext>(ctx: &JSContext, literal: &str) -> &JSContext::V {
    runtime::MockString::new(literal)
}

#[inline]
fn eval_bigint<JSContext: runtime::JSContext>(ctx: &JSContext, literal: &str) -> &JSContext::V {
    unimplemented!(); // TODO: parse bigint
    // ctx.new_bigint(parsed_bigint)
}

#[inline]
fn eval_array<JSContext: runtime::JSContext>(ctx: &JSContext, arr: &ast::ArrayExpression) -> &JSContext::V {
    let mut elements = Vec::with_capacity(arr.elements.len());

    for elem in arr.elements.iter() {
        match elem.body.unwrap() {
            ast::parameter_element::Body::Element(e) => elements.push(eval_expression(ctx, &e)),
            ast::parameter_element::Body::Spread(e) => {
                for val in eval_expression(ctx, &e).element_iter() {
                    elements.push(val);
                }
            },
        }
    }

    ctx.new_array(elements)
}

#[inline]
fn eval_object<JSContext: runtime::JSContext>(ctx: &JSContext, obj: &ast::ObjectExpression) -> &JSContext::V {
    let mut props = Vec::with_capacity(obj.elements.len());
    for elem in obj.elements.iter() {
        match elem.element.unwrap() {
            ast::object_expression::element::Element::Property(prop) => {
                let key = &prop.name?;
                let value = eval_expression(ctx, &prop.value);
                props.push((key, value));
            },
            ast::object_expression::element::Element::Shorthand(propname) => {
                let key = ast::PropName{name: propname};
                let value = ctx.resolve_binding(propname);
                props.push((key, value));
            },
            ast::object_expression::element::Element::Method(prop) => {
                unimplemented!("asdf")
            },
            ast::object_expression::element::Element::Spread(prop) => {
                let value = eval_expression(ctx, &prop.value);
                for inner_prop in value.property_iter() {
                    props.push(inner_prop);
                }
            },
        }
    }

    ctx.new_object(props)
}

#[inline]
fn eval_function<JSContext: runtime::JSContext>(ctx: &JSContext, func: &ast::FunctionExpression) -> &JSContext::V {
    let captured_vars = ctx.capture_variables(func.body);
    let function_object = ctx.new_function(func.identifier, func.parameters?, || eval_statement(ctx, func.body), captured_vars);
    function_object
}

#[inline]
fn eval_arrow_function<JSContext: runtime::JSContext>(ctx: &JSContext, func: &ast::ArrowFunctionExpression) -> &JSContext::V {
    eval_function(ctx, func) // TODO
}

#[inline]
fn eval_assignment<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::AssignmentExpression) -> runtime::Value {
    match expr.operator {
        ast::assignment_expression::Operator::Assign => eval_assign(ctx, expr.left, expr.right),
        ast::assignment_expression::Operator::Add => eval_lval(ctx, expr.left).as_numeric().add(eval_expression(ctx, &expr.right).as_numeric()),
    }
}



#[inline]
fn eval_call<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::CallExpression) -> JSContext::V {

}

#[inline]
fn eval_conditional<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::ConditionalExpression) -> runtime::Value {
    ctx.control_branch_value(
        || eval_expression(ctx, &expr.test), 
        || eval_expression(ctx, &expr.consequent),
        || eval_expression(ctx, &expr.alternate),
    )
}

#[inline]
fn eval_logical<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::LogicalExpression) -> JSContext::V {
    match expr.operator {
        ast::logical_expression::Operator::And => {
            let mut left = ctx.new_undefined();
            ctx.control_branch_value(
                || {
                    left = eval_expression(ctx, expr.left);
                    left
                },
                || eval_expression(ctx, expr.right),
                || left,
            )
        },
        ast::logical_expression::Operator::Or => {
            let mut left = ctx.new_undefined();
            ctx.control_branch_value(
                || {
                    left = eval_expression(ctx, expr.left);
                    left
                },
                || left,
                || eval_expression(ctx, expr.right),
            )
        },
        ast::logical_expression::Operator::Coalesce => {
            ctx.control_coalesce(
                || eval_expression(ctx, expr.left),
                || eval_expression(ctx, expr.right),
            )
        }
    }
}

#[inline]
fn eval_variable<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::VariableExpression) -> JSContext::V {
    ctx.resolve_binding(expr.name)
}

#[inline]
fn eval_binary<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::BinaryExpression) -> &JSContext::V {
    let left = eval_expression(ctx, &expr.left);
    let right = eval_expression(ctx, &expr.right);
    match expr.operator {
        ast::binary_expression::Operator::Add => left.to_number().add(right.to_number()),
        ast::binary_expression::Operator::Subtract => left.to_number().sub(right.to_number()),
        ast::binary_expression::Operator::Multiply => left.to_number().mul(right.to_number()),
        ast::binary_expression::Operator::Divide => left.to_number().div(right.to_number()),
        ast::binary_expression::Operator::Modulo => left.to_number().modulo(right.to_number()),
        ast::binary_expression::Operator::Exponent => unimplemented!(),
        ast::binary_expression::Operator::BitwiseAnd => unimplemented!(),
        ast::binary_expression::Operator::BitwiseOr => unimplemented!(),
        ast::binary_expression::Operator::BitwiseXor => unimplemented!(),
        ast::binary_expression::Operator::BitwiseLeftShift => unimplemented!(),
        ast::binary_expression::Operator::BitwiseRightShift => unimplemented!(),
        ast::binary_expression::Operator::LessThan => left.as_numeric().less_than(right.as_numeric()),
        ast::binary_expression::Operator::LessThanEqual => left.as_numeric().less_than_equal(right.as_numeric()),
        ast::binary_expression::Operator::GreaterThan => left.as_numeric().greater_than(right.as_numeric()),
        ast::binary_expression::Operator::GreaterThanEqual => left.to
    }
}

#[inline]
fn eval_unary<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::UnaryExpression) -> &JSContext::V {
    ctx.op_unary(expr.operator, eval_expression(ctx, &expr.argument))
}

#[inline]
fn eval_logical<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::LogicalExpression) -> &JSContext::V {
    ctx.op_logical(expr.operator, || { expr.left }, || { expr.right })
}

#[inline]
fn eval_update<JSContext: runtime::JSContext>(ctx: &JSContext, expr: &ast::UpdateExpression) -> &JSContext::V {
    ctx.op_update(expr.operator, eval_expression(ctx, &expr.argument))
}

fn free_variable_function_declaration(bound: HashSet<String>, stmt: &ast::FunctionDeclarationStatement) -> HashSet<String> {
    for param in stmt.params.iter() {
        if !bound.has(stmt.identifier) {
            remove.insert(stmt.identifier)
        }
        bound.insert(param.identifier);
    }
    let free = free_variable_block_statement(bound, &stmt.body);
    remove.iter().for_each(|identifier| bound.remove(identifier));
    free
}

fn free_variable_block_statement(bound: HashSet<String>, stmt: &ast::BlockStatement) -> HashSet<String> {
    let free = HashSet::new();
    let remove = BTreeSet::new();
    for stmt in stmt.statements.iter() {
        match stmt {
            // Declarations
            Statement::VariableDeclaration(stmt) => {
                match stmt {
                    ast::variable_declaration::Kind::Var(declarations) => {
                        for declaration in declarations.iter() {
                            if !bound.has(stmt.identifier) {
                                remove.insert(stmt.identifier)
                            }
                            bound.insert(declaration.identifier);
                        }
                    },
                    ast::variable_declaration::Kind::Let(declarations) => {
                        for declaration in declarations.iter() {
                            if !bound.has(stmt.identifier) {
                                remove.insert(stmt.identifier)
                            }
                            bound.insert(declaration.identifier);
                        }
                    },
                    ast::variable_declaration::Kind::Const(declarations) => {
                        for declaration in declarations.iter() {
                            if !bound.has(stmt.identifier) {
                                remove.insert(stmt.identifier)
                            }
                            bound.insert(declaration.identifier);
                        }
                    },
                }
            },
            Statement::FunctionDeclaration(stmt) => {
                if !bound.has(stmt.identifier) {
                    remove.insert(stmt.identifier)
                }
                bound.insert(stmt.identifier);
                free_variable_function_declaration(bound, stmt)
            },
            // Use
            Statement::BlockStatement(stmt) => free_variable_block_statement(stmt),
            Statement::IfStatement(stmt) => {
                free.extend(used_variable_expression(stmt.test).difference(bound));
                free.extend(free_variable_block_statement(bound, &stmt.consequent));
                if let Some(alternate) = &stmt.alternate {
                    free.extend(free_variable_block_statement(bound, alternate));
                }
            },
            Statement::WhileStatement(stmt) => {
                free.extend(used_variable_expression(stmt.test).difference(bound));
                free.extend(free_variable_block_statement(bound, &stmt.body));
            },
            Statement::BreakStatement(stmt) => {},
            Statement::ContinueStatement(stmt) => {},
            Statement::ReturnStatement(stmt) => free.extend(used_variable_expression(stmt.argument).difference(bound)),
            Statement::ExpressionStatement(stmt) => free.extend(used_variable_expression(stmt.expression).difference(bound)),
        };
    }

    remove.iter().for_each(|identifier| bound.remove(identifier));
    
    free
}

fn used_variable_expression(expr: &ast::Expression) -> HashSet<String> {
    let used = HashSet::new();
    match expr {
        ast::Expression::Literal(expr) => {},
        ast::Expression::Identifier(expr) => {
            used.insert(expr.identifier);
        },
        ast::Expression::Member(expr) => {
            used.insert(expr.object.identifier);
        },
        ast::Expression::Call(expr) => {
            used.insert(expr.callee.identifier);
            for arg in expr.args.iter() {
                used.extend(e_variable_expression(arg));
            }
        },
        ast::Expression::Binary(expr) => {
            used.extend(used_variable_expression(&expr.left));
            used.extend(used_variable_expression(&expr.right));
        },
        ast::Expression::Unary(expr) => {
            used.extend(used_variable_expression(&expr.argument));
        },
        ast::Expression::Logical(expr) => {
            used.extend(used_variable_expression(&expr.left));
            used.extend(used_variable_expression(&expr.right));
        },
        ast::Expression::Update(expr) => {
            used.extend(used_variable_expression(&expr.argument));
        },
        ast::Expression::Conditional(expr) => {
            used.extend(used_variable_expression(&expr.test));
            used.extend(used_variable_expression(&expr.consequent));
            used.extend(used_variable_expression(&expr.alternate));
        },
        ast::Expression::Array(expr) => {
            for element in expr.elements.iter() {
                used.extend(used_variable_expression(element));
            }
        }
    }
}