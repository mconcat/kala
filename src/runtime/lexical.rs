// The eval_ functions in this module evaluate the static semantics of the
// Jessie AST nodes, and provide additional typing/binding information. The
// runtime contexts are provided by the runtime module, which is responsible for
// evaluating the runtime semantics of the program.

use crate::ast;
use crate::runtime;
use crate::runtime::JSValue;

use ast::statement::Statement;

use core::panic;
use std::cell::RefCell;


pub fn eval_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::Statement) {
    match &stmt.statement {
        Some(Statement::VariableDeclaration(stmt)) => eval_variable_declaration(ctx, &stmt),
        // Function declarations are hoisted to the top of the lexical scope.
        // When the declaration statement is actually met, noop.
        Some(Statement::FunctionDeclaration(_stmt)) => unimplemented!("nested function declaration not supported yet"),

        Some(Statement::BlockStatement(stmt)) => eval_block_statement(ctx, &stmt),

        Some(Statement::IfStatement(stmt)) => eval_if_statement(ctx, &stmt),
        // Statement::ForStatement(stmt) => eval_for_statement(ctx, stmt),
        // Statement::ForOfStatement(stmt) => eval_for_of_statement(ctx, stmt),
        Some(Statement::WhileStatement(stmt)) => eval_while_statement(ctx, &stmt),
        // Statement::SwitchStatement(stmt) => eval_switch_statement(ctx, stmt),
    
        // Statement::TryStatement(stmt) => eval_try_statement(ctx, stmt),


        Some(Statement::BreakStatement(stmt)) => eval_break_statement(ctx, &stmt),
        Some(Statement::ContinueStatement(stmt)) => eval_continue_statement(ctx, &stmt), 
        Some(Statement::ReturnStatement(stmt)) => eval_return_statement(ctx, &stmt),
        // Statement::ThrowStatement(stmt) => eval_throw_statement(ctx, stmt),

        Some(Statement::ExpressionStatement(stmt)) => { eval_expression(ctx, &stmt.expression.as_ref().unwrap()); () },

        _ => unimplemented!(),
    }
}

fn early_error_variable_declaration(stmt: &ast::VariableDeclaration) {
    for decl in stmt.declarators.iter() {
        match &decl.declarator {
            Some(ast::variable_declarator::Declarator::Normal(decl)) => {
                if decl.identifier.as_ref().unwrap().name == "let" || decl.identifier.as_ref().unwrap().name == "const" {
                    panic!("early error: variable declaration cannot be `let` or `const`");
                }
            },
            Some(ast::variable_declarator::Declarator::Binding(_decl)) => {
                unimplemented!("asdf")
            },
            None => panic!("early error: variable declaration must have a declarator"),
        }
    }
}

#[inline]
fn eval_variable_declaration<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::VariableDeclaration) {
    if ctx.check_early_errors() {
        early_error_variable_declaration(stmt);
    }

    for decl in stmt.declarators.iter() {
        match &decl.declarator {
            Some(ast::variable_declarator::Declarator::Normal(decl)) => {
                // RS: Evaluation
                let name = &decl.identifier.as_ref().unwrap().name;
                let existing_binding = ctx.resolve_binding(name);
                let value = match &decl.value {
                    Some(expr) => Some(eval_expression(ctx, &expr)),
                    None => None,
                };
                let kind = ast::DeclarationKind::from_i32(stmt.kind).unwrap();
                match existing_binding {
                    Ok(binding) => {
                        ctx.set_binding(name, value.unwrap());
                    },
                    Err(_) => {
                        match kind {
                            ast::DeclarationKind::Let => {
                                ctx.initialize_mutable_binding(name, &value);
                            },
                            ast::DeclarationKind::Const => {
                                ctx.initialize_immutable_binding(name, &value.unwrap());
                            },
                            _ => panic!("unknown declaration kind"),
                        }
                    },
                }
            },
            Some(ast::variable_declarator::Declarator::Binding(decl)) => {
                unimplemented!("binding variable declarators")
            },
            None => panic!("variable declaration must have a declarator"),
        }
    }
}

#[inline]
fn early_error_function_declaration(stmt: &ast::FunctionDeclaration) {
            /*
    let decl = stmt.function.unwrap();
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

        */
}



// hoist_function_declaration is called when a function declaration is
// encountered in the lexical scope, at the time of evaluating parent statements.
// It creates and adds the function objects to the current context.
#[inline]
fn hoist_function_declaration<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::FunctionDeclaration) -> JSContext::V {
    // TODO: implement closure definition. global declaration is sufficient for now.

    /*
    if ctx.check_early_errors() {
        early_error_function_declaration(stmt);
    }

    // capture variables
    // within the function body, the free variables(excluding those declared as parameters or variables) are captured.
    let captured = ctx.extract_free_variables(free_variable_function_declaration(HashSet::new(), stmt));
    */

    let function = stmt.function.as_ref().unwrap();
    let function_object = ctx.function(
        function.identifier.as_ref().map(|name| name.name.to_string()), 
        function.parameters.iter().map(|param| {
            // TODO: handle other patterns
            if let ast::parameter_pattern::Body::Pattern(p) = param.body.as_ref().unwrap() {
                if let ast::pattern::Pattern::Identifier(id) = p.pattern.as_ref().unwrap() {
                    return id.name.to_string()
                }
            };
            unimplemented!("unknown pattern")
            
        }).collect(), 
        function, 
        Vec::new());
    function_object
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
pub fn eval_block_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::BlockStatement) {
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
fn eval_if_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::IfStatement) {
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
fn eval_break_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::BreakStatement) {
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
fn eval_continue_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::ContinueStatement) {
    // continue_loop is a signal that the nearest enclosing loop should continue.
    // it sets the internal flag to true, which is checked by the surrounding
    // iteration statements(e.g. block, loop, switch)
    ctx.complete_continue();
}

fn eval_return_statement<JSContext: runtime::JSContext>(ctx: &mut JSContext, stmt: &ast::ReturnStatement) {
    let value = match &stmt.argument {
        Some(expr) => eval_expression(ctx, expr),
        None => JSContext::undefined(),
    };
    ctx.complete_return_value(value);
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
        |ctx| { eval_expression(ctx, &stmt.test.as_ref().unwrap()) }, 
        |ctx| { eval_statement(ctx, &*stmt.body.as_ref().unwrap()) },
  )
}

use ast::expression::Expression;

pub fn eval_expression<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::Expression) -> JSContext::V {
    match &expr.expression {
        Some(Expression::Literal(expr)) => eval_literal(ctx, &expr.literal.as_ref().unwrap()),
        Some(Expression::Array(expr)) => eval_array(ctx, &expr),
        Some(Expression::Object(expr)) => eval_object(ctx, &expr),
        Some(Expression::Function(expr)) => eval_function(ctx, &expr),
        Some(Expression::ArrowFunction(expr)) => eval_arrow_function(ctx, &expr),
        
        Some(Expression::Binary(expr)) => eval_binary(ctx, &expr),
        // Expression::Unary(expr) => eval_unary(ctx, expr),
        Some(Expression::Conditional(expr)) => eval_conditional(ctx, &expr),
        Some(Expression::Logical(expr)) => eval_logical(ctx, &expr),
        // Expression::Update(expr) => eval_update(ctx, expr),
        
        Some(Expression::Variable(expr)) => eval_variable(ctx, &expr),
        Some(Expression::Assignment(expr)) => eval_assignment(ctx, &expr),
        Some(Expression::Member(expr)) => eval_member(ctx, &expr),
 
        Some(Expression::Call(expr)) => eval_call(ctx, &expr),

        _ => unimplemented!(),
    }
}

use ast::literal::Literal;

#[inline]
fn eval_literal<JSContext: runtime::JSContext>(ctx: &mut JSContext, literal: &ast::Literal) -> JSContext::V {
    match &literal.literal {
        Some(Literal::UndefinedLiteral(_)) => JSContext::undefined(),
        Some(Literal::NullLiteral(_)) => JSContext::null(),
        Some(Literal::BooleanLiteral(literal)) => JSContext::boolean(*literal),
        Some(Literal::NumberLiteral(literal)) => JSContext::number(*literal),
        Some(Literal::StringLiteral(literal)) => JSContext::string(*literal),
        // Literal::Bigint(literal) => JSContext::new_bigint(literal),
        _ => unimplemented!(),
    }
}

#[inline]
fn eval_array<JSContext: runtime::JSContext>(ctx: &mut JSContext, arr: &ast::ArrayExpression) -> JSContext::V {
    let mut elements = Vec::with_capacity(arr.elements.len());

    for elem in arr.elements.iter() {
        match elem.body.as_ref().unwrap() {
            ast::parameter_element::Body::Element(e) => elements.push(eval_expression(ctx, &e)),
            /*
            ast::parameter_element::Body::Spread(e) => {
                for val in eval_expression(ctx, &e).element_iter() {
                    elements.push(val);
                }
            },
            */
            _ => unimplemented!(),
        }
    }

    ctx.new_array(elements)
}

#[inline]
fn eval_object<JSContext: runtime::JSContext>(ctx: &mut JSContext, obj: &ast::ObjectExpression) -> JSContext::V {
    let mut props = Vec::with_capacity(obj.elements.len());
    for elem in obj.elements.iter() {
        match elem.element.as_ref().unwrap() {
            ast::object_expression::element::Element::Property(prop) => {
                let key  = match prop.name.as_ref().unwrap().name.as_ref().unwrap() {
                    ast::prop_name::Name::Identifier(id) => id,
                    ast::prop_name::Name::StringLiteral(lit) => lit,
                    ast::prop_name::Name::NumberLiteral(lit) => lit.to_string(),
                    _ => unimplemented!(),
                };
                let value = eval_expression(ctx, prop.value.as_ref().unwrap());
                props.push((key, value));
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
pub fn eval_function<JSContext: runtime::JSContext>(ctx: &mut JSContext, func: &ast::FunctionExpression) -> JSContext::V {
    // TODO: implement variable capture
    // only the locally declared variables and function parameters are available now
    
    let params = func.parameters.iter().map(|pat| match pat.body.as_ref().unwrap() {
        ast::parameter_pattern::Body::Pattern(pat) => match pat.pattern.as_ref().unwrap() {
                ast::pattern::Pattern::Identifier(id) => id.name.to_string(),
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
fn eval_arrow_function<JSContext: runtime::JSContext>(ctx: &JSContext, func: &ast::ArrowFunctionExpression) -> JSContext::V {
    unimplemented!("arrow function lieteral")
    // eval_function(ctx,) // TODO
}

#[inline]
fn eval_assignment<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::AssignmentExpression) -> JSContext::V {
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
fn eval_call<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::CallExpression) -> JSContext::V {
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
fn eval_conditional<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::ConditionalExpression) -> JSContext::V {
    ctx.control_branch_value(
        |ctx| eval_expression(ctx, &*expr.test.as_ref().unwrap()), 
        |ctx| eval_expression(ctx, &*expr.consequent.as_ref().unwrap()),
        |ctx| eval_expression(ctx, &*expr.alternate.as_ref().unwrap()),
    )
}

fn eval_logical<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::LogicalExpression) -> JSContext::V {
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
fn eval_variable<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::VariableExpression) -> JSContext::V {
    match ctx.resolve_binding(&expr.name.as_ref().unwrap().name.to_string()) {
        Ok(val) => val,
        Err(err) => unimplemented!("{}", err)
    }
}
use crate::ast::binary_expression::Operator;

// binary operations does NOT coerces the values to primitive.
#[inline]
fn eval_binary<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::BinaryExpression) -> JSContext::V {
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

        Operator::Bitand => ctx.op_bit_and(&mut left, &right),
        Operator::Bitor => ctx.op_bit_or(&mut left, &right),
        Operator::Bitxor => ctx.op_bit_xor(&mut left, &right),
        Operator::Lshift => ctx.op_bit_lshift(&mut left, &right),
        Operator::Rshift => ctx.op_bit_rshift(&mut left, &right),
        Operator::Urshift => ctx.op_bit_urshift(&mut left, &right),

        _ => unimplemented!(""),
    }
}

#[inline]
fn eval_unary<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::UnaryExpression) -> JSContext::V {
    use ast::unary_expression::Operator;
    match Operator::from_i32(expr.operator) {
        Some(Operator::Pos) => {
            eval_expression(ctx, &*expr.argument.as_ref().unwrap())
        }
        Some(Operator::Neg) => {
            let arg = eval_expression(ctx, &*expr.argument.as_ref().unwrap());
            match arg.as_number() {
                Some(n) => ctx.wrap_number(n.op_neg()),
                None => unimplemented!(),
            }
        },
        Some(Operator::Not) => {
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
fn eval_update<JSContext: runtime::JSContext>(_ctx: &mut JSContext, _expr: &ast::UpdateExpression) -> JSContext::V {
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
fn eval_member<JSContext: runtime::JSContext>(ctx: &mut JSContext, expr: &ast::MemberExpression) -> JSContext::V {
    let obj = eval_expression(ctx, expr.object.as_ref().unwrap());
    let propname = match expr.member.as_ref().unwrap() {
        ast::member_expression::Member::Index(iexpr) => {
            let index = eval_expression(ctx, iexpr);
            ctx.object_property_computed(&obj, &index)
        },
        ast::member_expression::Member::Property(id) => {
            ctx.object_property(&obj, &id.name)
        },
    };

}