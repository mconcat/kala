// The eval_ functions in this module evaluate the static semantics of the
// Jessie AST nodes, and provide additional typing/binding information. The
// runtime contexts are provided by the runtime module, which is responsible for
// evaluating the runtime semantics of the program.

use kala_ast::ast::{self, DeclarationKind};
use kala_context::JSContext;

use core::panic;
use std::cell::RefCell;


pub fn eval_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::Statement) {
    match stmt {
        ast::Statement::VariableDeclaration(stmt) => eval_variable_declaration(ctx, &stmt),
        // Function declarations are hoisted to the top of the lexical scope.
        // When the declaration statement is actually met, noop.
        ast::Statement::FunctionDeclaration(_stmt) => unimplemented!("nested function declaration not supported yet"),

        ast::Statement::Block(stmt) => eval_block_statement(ctx, &stmt),

        ast::Statement::If(stmt) => eval_if_statement(ctx, &stmt),
        // Statement::ForStatement(stmt) => eval_for_statement(ctx, stmt),
        // Statement::ForOfStatement(stmt) => eval_for_of_statement(ctx, stmt),
        ast::Statement::While(stmt) => eval_while_statement(ctx, &stmt),
        // Statement::SwitchStatement(stmt) => eval_switch_statement(ctx, stmt),
    
        // Statement::TryStatement(stmt) => eval_try_statement(ctx, stmt),


        ast::Statement::Break(stmt) => eval_break_statement(ctx, &stmt),
        ast::Statement::Continue(stmt) => eval_continue_statement(ctx, &stmt), 
        ast::Statement::Return(stmt) => eval_return_statement(ctx, &stmt),
        // Statement::ThrowStatement(stmt) => eval_throw_statement(ctx, stmt),

        ast::Statement::Expression(stmt) => { eval_expression(ctx, &stmt.expression.as_ref().unwrap()); () },

        _ => unimplemented!(),
    }
}

#[inline]
fn eval_variable_declaration<Context: JSContext>(ctx: &mut Context, stmt: &ast::VariableDeclaration) {
    /*
    if ctx.check_early_errors() {
       // early_error_variable_declaration(stmt);
    }
    */

    for decl in stmt.declarators.iter() {
        match decl.binding {
            ast::Pattern::Identifier(ident) => {
                let existing_binding = ctx.resolve_binding(&ident.name);
                let value = decl.init.map(|init| eval_expression(ctx, &init)).unwrap_or(ctx.undefined());
                match existing_binding {
                    Ok(binding) => {
                        ctx.set_binding(&ident.name, value);
                    },
                    Err(_) => {
                        match decl.kind {
                            ast::DeclarationKind::Const => ctx.initialize_immutable_binding(&ident.name, value),
                            ast::DeclarationKind::Let => ctx.initialize_mutable_binding(&ident.name, value),
                        };
                    },
                } 
            },
            _ => unimplemented!("pattern binding initializers not supported yet"),
        }
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
pub fn eval_block_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::BlockStatement) {
    ctx.block_scope(Vec::new(),|ctx| {
        /* // TODO: uncomment function hoisting=
        for stmt in stmt.body.iter() {
            if let Statement::FunctionDeclaration(stmt) = stmt {
                hoist_function_declaration(ctx, &stmt);
            }
        }
        */

        for stmt in stmt.body.iter() {
            eval_statement(ctx, stmt);
        }
    });
}

#[inline]
fn eval_if_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::IfStatement) {
    ctx.control_branch(|ctx| eval_expression(ctx, &stmt.test.as_ref().unwrap()), 
        |ctx| eval_statement(ctx, &*stmt.consequent.as_ref().unwrap()), 
        |ctx| eval_statement(ctx, &*stmt.alternate.as_ref().unwrap()),
    )
}

// https://tc39.es/ecma262/#sec-break-statement-runtime-semantics-evaluation
// 
// No labeled break implementation.
//
// Break statement invokes a termination signal that propagates over the ast
// and handled by the nearest enclosing loop.
fn eval_break_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::BreakStatement) {
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
fn eval_continue_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::ContinueStatement) {
    // continue_loop is a signal that the nearest enclosing loop should continue.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.complete_continue();
}

fn eval_return_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::ReturnStatement) {
    let value = match &stmt.argument {
        Some(expr) => eval_expression(ctx, expr),
        None => JSContext::undefined(),
    };
    ctx.complete_return_value(value);
}


// https://262.ecma-international.org/9.0/#sec-for-statement
// 
/*
fn eval_for_statement(ctx: &impl JSContext, stmt: &ast::ForStatement) {
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

fn eval_for_of_statement<Context: JSContext>(ctx: &JSContext, stmt: &ast::ForOfStatement) {
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

fn eval_while_statement<Context: JSContext>(ctx: &mut Context, stmt: &ast::WhileStatement) {
    ctx.control_loop(
        |ctx| { eval_expression(ctx, &stmt.test.as_ref().unwrap()) }, 
        |ctx| { eval_statement(ctx, &*stmt.body.as_ref().unwrap()) },
  )
}

pub fn eval_expression<Context: JSContext>(ctx: &mut Context, expr: &ast::Expression) -> Context::V {
    match &expr.expression {
        Some(ast::Expression::Literal(expr)) => eval_literal(ctx, &expr.literal.as_ref().unwrap()),
        Some(ast::Expression::Array(expr)) => eval_array(ctx, &expr),
        Some(ast::Expression::Object(expr)) => eval_object(ctx, &expr),
        Some(ast::Expression::Function(expr)) => eval_function(ctx, &expr),
        Some(ast::Expression::ArrowFunction(expr)) => eval_arrow_function(ctx, &expr),
        
        Some(ast::Expression::Binary(expr)) => eval_binary(ctx, &expr),
        // Expression::Unary(expr) => eval_unary(ctx, expr),
        Some(ast::Expression::Conditional(expr)) => eval_conditional(ctx, &expr),
        Some(ast::Expression::Logical(expr)) => eval_logical(ctx, &expr),
        // Expression::Update(expr) => eval_update(ctx, expr),
        
        Some(ast::Expression::Variable(expr)) => eval_variable(ctx, &expr),
        Some(ast::Expression::Assignment(expr)) => eval_assignment(ctx, &expr),
        Some(ast::Expression::Member(expr)) => eval_member(ctx, &expr),
 
        Some(ast::Expression::Call(expr)) => eval_call(ctx, &expr),

        _ => unimplemented!(),
    }
}

#[inline]
fn eval_literal<Context: JSContext>(ctx: &mut Context, literal: &ast::Literal) -> Context::V {
    match literal {
        ast::Literal::Undefined => JSContext::undefined(),
        ast::Literal::Null => JSContext::null(),
        ast::Literal::Boolean(literal) => JSContext::boolean(*literal),
        ast::Literal::Number(literal) => JSContext::number(*literal),
        ast::Literal::String(literal) => JSContext::string(*literal),
        // Literal::Bigint(literal) => JSContext::new_bigint(literal),
        _ => unimplemented!(),
    }
}

#[inline]
fn eval_array<Context: JSContext>(ctx: &mut Context, arr: &ast::ArrayExpression) -> Context::V {
    let mut elements = Vec::with_capacity(arr.elements.len());

    for elem in arr.elements.iter() {
        match elem.body.as_ref().unwrap() {
            ast::ParameterElement::Parameter(e) => elements.push(eval_expression(ctx, &e)),
            /*
            ast::parameter_element::Body::Spread(e) => {
                for val in eval_expression(ctx, &e).element_iter() {
                    elements.push(val);
                }
            },
            */
            ast::ParameterElement::Spread(_) => unimplemented!(),
        }
    }

    ctx.new_array(elements)
}

#[inline]
fn eval_object<Context: JSContext>(ctx: &mut Context, obj: &ast::ObjectExpression) -> Context::V {
    let mut props = Vec::with_capacity(obj.elements.len());
    for elem in obj.elements.iter() {
        match elem.element.as_ref().unwrap() {
            ast::ObjectElement::KeyValue(key, value) => { 
                props.push((key, eval_expression(ctx, value.as_ref().unwrap())));
            },
            /*
            ast::object_expression::element::Element::Shorthand(propname) => {
                let key = ast::PropName{name: propname};
                let value = ctx.resolve_binding(propname);
                props.push((&key, value));
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
            */
            _ => unimplemented!(),
        }
    }

    ctx.new_object(props)
}

#[inline]
pub fn eval_function<Context: JSContext>(ctx: &mut Context, func: &ast::FunctionExpression) -> Context::V {
    // TODO: implement variable capture
    // only the locally declared variables and function parameters are available now
    
    let params = func.params.iter().map(|pat| match pat {
        ast::Pattern::Body::Pattern(pat) => match pat.pattern.as_ref().unwrap() {
                ast::Pattern::Identifier(id) => id.name.to_string(),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }).collect();
        /*
        ast::parameter_element::Body::Spread(e) => {
            for val in eval_expression(ctx, &e).element_iter() {
                params.push(val);
            }
        },
        */

    let function_object = ctx.new_function(func.identifier.as_ref().map(|x| x.name.to_string()), params, func, vec![]);

    function_object

    /*
    let captured_vars = ctx.capture_variables(func.body);
    let function_object = ctx.new_function(func.identifier, func.parameters?, || eval_statement(ctx, func.body), captured_vars);
    function_object
    */
}

#[inline]
fn eval_arrow_function<Context: JSContext>(ctx: &mut Context, func: &ast::ArrowFunctionExpression) -> Context::V {
    unimplemented!("arrow function lieteral")
    // eval_function(ctx,) // TODO
}

#[inline]
fn eval_assignment<Context: JSContext>(ctx: &mut Context, expr: &ast::AssignmentExpression) -> Context::V {
    let left = match expr.left.as_ref().unwrap().lvalue.as_ref().unwrap() {
        ast::assignment_expression::l_value::Lvalue::Identifier(id) => {
            &id.name.to_string()
        },        
        ast::assignment_expression::l_value::Lvalue::Member(mexpr) => {
            unimplemented!("member expression")
        }
    };

    return match ast::assignment_expression::Operator::from_i32(expr.operator).unwrap() {
        ast::assignment_expression::Operator::Assign => {
            let right = eval_expression(ctx, expr.right.as_ref().unwrap());
            ctx.set_binding(left, right);
            right
        },
        _ => unimplemented!(),
    };
}


#[inline]
fn eval_call<Context: JSContext>(ctx: &mut Context, expr: &ast::CallExpression) -> Context::V {
    let callee = eval_expression(ctx, expr.callee.as_ref().unwrap());
            
    let args = expr.arguments.iter().map(|elem| {
        match elem.body.as_ref().unwrap() {
            ast::call_expression::call_element::Body::Element(e) => eval_expression(ctx, &e),
            _ => unimplemented!(),
        }
    }).collect();

    ctx.enter_function(&callee, args).unwrap_or(ctx.new_undefined())
    
    /* 
    callee.as_reference().call(ctx, expr.arguments.iter().map(|arg| eval_expression(ctx, arg)).collect())
    let args = expr.arguments.iter().map(|arg| {
        match arg.body.unwrap() {
            ast::call_expression::call_element::Body::Element(e) => e,
            _ => unimplemented!(),
        }
    }).collect::<Vec<_>>();
    ctx.call(callee, args)
    */
}

#[inline]
fn eval_conditional<Context: JSContext>(ctx: &mut Context, expr: &ast::ConditionalExpression) -> Context::V {
    ctx.control_branch_value(
        |ctx| eval_expression(ctx, &*expr.test.as_ref().unwrap()), 
        |ctx| eval_expression(ctx, &*expr.consequent.as_ref().unwrap()),
        |ctx| eval_expression(ctx, &*expr.alternate.as_ref().unwrap()),
    )
}

fn eval_logical<Context: JSContext>(ctx: &mut Context, expr: &ast::LogicalExpression) -> Context::V {
    use ast::logical_expression::Operator;
    match Operator::from_i32(expr.operator) {
        Some(Operator::And) => {
            let left = RefCell::new(ctx.new_undefined());
            ctx.control_branch_value(
                |ctx| { *left.borrow_mut() = eval_expression(ctx, &*expr.left.as_ref().unwrap()); left.borrow().clone() },
                |ctx| eval_expression(ctx, &*expr.right.as_ref().unwrap()),
                |_| left.borrow().clone(),
            )
        },
        Some(Operator::Or) => {
            let left = RefCell::new(ctx.new_undefined());
            ctx.control_branch_value(
                |ctx| {
                    *left.borrow_mut() = eval_expression(ctx, &*expr.left.as_ref().unwrap());
                    left.borrow().clone()
                },
                |_| left.borrow().clone(), 
                |ctx| eval_expression(ctx, &*expr.right.as_ref().unwrap()),
            )
        },
        Some(Operator::Coalesce) => {
            ctx.control_coalesce(
                |ctx| eval_expression(ctx, &*expr.left.as_ref().unwrap()),
                |ctx| eval_expression(ctx, &*expr.right.as_ref().unwrap()),
            )
        },
        _ => unimplemented!(),
    }
}

#[inline]
fn eval_variable<Context: JSContext>(ctx: &mut Context, expr: &ast::VariableExpression) -> Context::V {
    match ctx.resolve_binding(&expr.name.as_ref().unwrap().name.to_string()) {
        Ok(val) => val,
        Err(err) => unimplemented!("{}", err)
    }
}
use crate::ast::binary_expression::Operator;

// binary operations does NOT coerces the values to primitive.
#[inline]
fn eval_binary<Context: JSContext>(ctx: &mut Context, expr: &ast::BinaryExpression) -> Context::V {
    let left = eval_expression(ctx, &*expr.left.as_ref().unwrap());
    let right = eval_expression(ctx, &*expr.right.as_ref().unwrap());
    let op = Operator::from_i32(expr.operator).unwrap();
    match op {
        Operator::Add => ctx.op_add(&mut left, &right),
        Operator::Sub => ctx.op_sub(&mut left, &right),
        Operator::Mul => ctx.op_mul(&mut left, &right),
        Operator::Div => ctx.op_div(&mut left, &right),
        Operator::Mod => ctx.op_mod(&mut left, &right),
        Operator::Pow => ctx.op_pow(&mut left, &right),

        Operator::Eq => ctx.op_eq(&mut left, &right),
        Operator::Neq => ctx.op_neq(&mut left, &right),
        Operator::Gt => ctx.op_gt(&mut left, &right),
        Operator::Gte => ctx.op_gte(&mut left, &right),
        Operator::Lt => ctx.op_lt(&mut left, &right),
        Operator::Lte => ctx.op_lte(&mut left, &right),
/*
        Operator::Bitand => ctx.op_bit_and(&mut left, &right),
        Operator::Bitor => ctx.op_bit_or(&mut left, &right),
        Operator::Bitxor => ctx.op_bit_xor(&mut left, &right),
        Operator::Lshift => ctx.op_bit_lshift(&mut left, &right),
        Operator::Rshift => ctx.op_bit_rshift(&mut left, &right),
        Operator::Urshift => ctx.op_bit_urshift(&mut left, &right),
*/
        _ => unimplemented!(""),
    }
}

#[inline]
fn eval_unary<Context: JSContext>(ctx: &mut Context, expr: &ast::UnaryExpression) -> Context::V {
    match expr.operator {
        Operator::Pos => {
            eval_expression(ctx, &*expr.argument.as_ref().unwrap())
        }
        Operator::Neg => {
            let arg = eval_expression(ctx, &*expr.argument.as_ref().unwrap());
            match arg.as_number() {
                Some(n) => ctx.wrap_number(n.op_neg()),
                None => unimplemented!(),
            }
        },
        Operator::Not => {
            let arg = eval_expression(ctx, &*expr.argument.as_ref().unwrap());
            match arg.as_boolean() {
                Some(b) => ctx.new_boolean(!b),
                None => unimplemented!("some type error message"),        
            }
        },
        _ => unimplemented!(),
    }
}

#[inline]
fn eval_update<Context: JSContext>(_ctx: &mut Context, _expr: &ast::UpdateExpression) -> Context::V {
/*    use crate::runtime::JSNumeric;
    use ast::update_expression::Operator;


    let mut value = eval_expression(ctx, &*expr.argument.as_ref().unwrap()).to_number();
    let mut result = if expr.prefix {
        value
    } else {
        ctx.dup(value).to_number()
    };

    match Operator::from_i32(expr.operator) {
        Some(Operator::Inc) => {
            value.op_inc();
        },
        Some(Operator::Dec) => {
            value.op_dec();
        },
        _ => unimplemented!(),
    };

    ctx.wrap_number(result)
    */
    unimplemented!()
}
/*
fn free_variable_function_declaration(bound: HashSet<String>, stmt: &ast::FunctionDeclaration) -> HashSet<String> {
    let remove = HashSet::new();

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
                for declaration in stmt.declarators.iter() {
                    if !bound.has(stmt.identifier) {
                        remove.insert(stmt.identifier)
                    }
                    bound.insert(declaration.identifier);
                }
            },
            Statement::FunctionDeclaration(stmt) => {
                if !bound.has(stmt.identifier) {
                    remove.insert(stmt.identifier)
                }
                bound.insert(stmt.identifier);
                free_variable_function_declaration(bound, &stmt)
            },
            // Use
            Statement::BlockStatement(stmt) => free_variable_block_statement(stmt),
            Statement::IfStatement(stmt) => {
                free.extend(used_variable_expression(stmt.test).difference(&bound));
                free.extend(free_variable_block_statement(bound, &stmt.consequent));
                if let Some(alternate) = &stmt.alternate {
                    free.extend(free_variable_block_statement(bound, alternate));
                }
            },
            Statement::WhileStatement(stmt) => {
                free.extend(used_variable_expression(stmt.test).difference(&bound));
                free.extend(free_variable_block_statement(bound, &stmt.body));
            },
            Statement::BreakStatement(stmt) => {},
            Statement::ContinueStatement(stmt) => {},
            Statement::ReturnStatement(stmt) => free.extend(used_variable_expression(stmt.argument).difference(&bound)),
            Statement::ExpressionStatement(stmt) => free.extend(used_variable_expression(stmt.expression).difference(&bound)),
        };
    }

    remove.iter().for_each(|identifier| bound.remove(identifier));
    
    free
}

fn used_variable_expression(expr: &ast::Expression) -> HashSet<String> {
    let used = HashSet::new();
    match expr.expression {
        Some(Expression::Literal(expr)) => {},
        Some(Expression::Variable(expr)) => {
            used.insert(expr.identifier);
        },
        Some(Expression::Member(expr)) => {
            used.insert(expr.object.identifier);
        },
        Some(Expression::Call(expr)) => {
            used.insert(expr.callee.identifier);
            for arg in expr.args.iter() {
                used.extend(used_variable_expression(arg));
            }
        },
        Some(Expression::Binary(expr)) => {
            used.extend(used_variable_expression(&*expr.left.unwrap()));
            used.extend(used_variable_expression(&*expr.right.unwrap()));
        },
        Some(Expression::Unary(expr)) => {
            used.extend(used_variable_expression(&*expr.argument.unwrap()));
        },
        Some(Expression::Logical(expr)) => {
            used.extend(used_variable_expression(&*expr.left.unwrap()));
            used.extend(used_variable_expression(&*expr.right.unwrap()));
        },
        Some(Expression::Update(expr)) => {
            used.extend(used_variable_expression(&*expr.argument.unwrap()));
        },
        Some(Expression::Conditional(expr)) => {
            used.extend(used_variable_expression(&*expr.test.unwrap()));
            used.extend(used_variable_expression(&*expr.consequent.unwrap()));
            used.extend(used_variable_expression(&*expr.alternate.unwrap()));
        },
        Some(Expression::Array(expr))=> {
            for element in expr.elements.iter() {
                used.extend(used_variable_expression(element));
            }
        }
        None => unimplemented!(),
    };
    used
}
*/

#[inline]
fn eval_member<Context: JSContext>(ctx: &mut Context, expr: &ast::MemberExpression) -> Context::V {
    let obj = eval_expression(ctx, expr.object.as_ref().unwrap());
    let propname = match expr.member.as_ref().unwrap() {
        ast::Member::Computed(iexpr) => {
            let index = eval_expression(ctx, &iexpr);
            ctx.object_property_computed(&obj, &index)
        },
        ast::Member::Property(id) => {
            ctx.object_property(&obj, &id.name)
        },
    };

}