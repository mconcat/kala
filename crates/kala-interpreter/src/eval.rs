use std::borrow::BorrowMut;

use kala_ast::ast::{self, FunctionExpression, NodeF, ParameterElement};
use kala_context::environment_record::EnvironmentRecord;
use crate::context::{InterpreterContext, CompletionSignal};
use crate::declare::{*, self};
use crate::value::JSValue;
use crate::node::{InterpreterF as F, Identifier};
use crate::node;
use crate::literal::{Literal, NumberLiteral};

pub struct Eval<'a> {
    pub ctx: &'a mut InterpreterContext,
}

impl<'a> Eval<'a> {
    pub fn new(ctx: &'a mut InterpreterContext) -> Self {
        Eval {
            ctx,
        }
    }

    pub fn statement(&mut self, stmt: &mut node::Statement) {
        let mut stmt = &mut stmt.statement;
        match &mut stmt {
            ast::Statement::VariableDeclaration(stmt) => self.eval_variable_declaration(stmt),
            ast::Statement::FunctionDeclaration(stmt) => self.eval_function_declaration(stmt),
    
            ast::Statement::Block(stmt) => self.block(&mut stmt.block),
    
            ast::Statement::If(stmt) => self.eval_if(stmt),
            ast::Statement::For(stmt) => self.eval_for(stmt),
            ast::Statement::ForOf(stmt) => self.eval_for_of(stmt),
            ast::Statement::While(stmt) => self.eval_while(stmt),
            ast::Statement::Switch(stmt) => unimplemented!(), // eval_switch(stmt),
    
            ast::Statement::Try(stmt) =>  unimplemented!(),// eval_try(stmt),
    
            ast::Statement::Break(stmt) => self.eval_break(stmt),
            ast::Statement::Continue(stmt) => self.eval_continue(stmt),
            ast::Statement::Return(stmt) => self.eval_return(stmt),
            ast::Statement::Throw(stmt) => unimplemented!(), // eval_throw(stmt),
    
            ast::Statement::Expression(stmt) => { self.expression(&mut stmt.expression); },
        }
    }
    
    fn eval_variable_declaration(&mut self, stmt: &mut ast::VariableDeclaration<F>) {
        for decl in &mut stmt.declarators {
            if let Some(expr) = &mut decl.init {
                let value = self.expression(expr);
                if value.is_none() {
                    unimplemented!("TODO: handle error")
                }
                declare_binding(&mut self.ctx, stmt.kind.is_mutable(), &decl.binding, &value);
            } else {
                declare_binding(&mut self.ctx, stmt.kind.is_mutable(), &decl.binding, &None);
            }
        }
    }
    
    fn eval_function_declaration(&mut self, stmt: &mut ast::FunctionDeclaration<F>) {
        let function_env = EnvironmentRecord::new(); 
        // TODO: add captured variables to function_env
        let value = JSValue::function(function_env, stmt.function.clone());
        self.ctx.declare_immutable_binding(stmt.function.function.name.as_ref().unwrap(), &value);
    }
    
    pub fn block(&mut self, block: &mut ast::BlockStatement<F>) {
        self.ctx.enter_scope();
        for stmt in block.body.iter_mut() {
            self.statement(stmt);
        }
        self.ctx.exit_scope();
    }
    
    fn eval_if(&mut self, stmt: &mut ast::IfStatement<F>) {
        let cond = self.expression(&mut stmt.test);
        if cond.is_none() {
            unimplemented!("TODO: handle error")
        }
        let cond = cond.unwrap();
        self.ctx.enter_scope();
        if cond.is_truthy() {
            self.statement(&mut stmt.consequent);
        } else if let Some(alt) = &mut stmt.alternate {
            self.statement(alt);
        }
        self.ctx.exit_scope();
    }
    
    fn eval_for(&mut self, stmt: &mut ast::ForStatement<F>) {
        if let Some(init) = &mut stmt.init {
            unimplemented!("for init");
        }
        self.ctx.enter_for_scope();
        loop {
            self.ctx.loop_scope();
            if let Some(cond) = &mut stmt.test {
                let cond = self.expression(cond);
                if cond.is_none() {
                    self.ctx.exit_for_scope();
                    return; // TODO: return error
                }
                if !cond.unwrap().is_truthy() {
                    break;
                }
            }
    
            self.statement(&mut stmt.body);
    
            if let Some(completion) = self.ctx.completion_signal() {
                match completion {
                    CompletionSignal::Break => {
                        self.ctx.clear_completion_signal();
                        break
                    },
                    CompletionSignal::Continue => {
                        self.ctx.clear_completion_signal();
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
                self.expression(update);
            }
        }
        self.ctx.exit_for_scope();
    }
    
    fn eval_for_of(&mut self, stmt: &mut ast::ForOfStatement<F>) {
        unimplemented!()
        /*
        let iterable = self.expression(&mut stmt.decl.init.expect("for-of must have init"));
       
        ctx.enter_scope();
        for item in iterable.iter() {
            declare_binding(stmt.kind, &stmt.decl.binding, item);
            eval_statement(&mut stmt.body);
    
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
    
    fn eval_while(&mut self, stmt: &mut ast::WhileStatement<F>) {
        self.ctx.enter_scope();
        loop {
            let cond = self.expression(&mut stmt.test).unwrap(); // TODO: handle error
            if !cond.is_truthy() {
                break;
            }
    
            self.statement(&mut stmt.body);
    
            if let Some(completion) = self.ctx.completion_signal() {
                match completion {
                    CompletionSignal::Break => {
                        self.ctx.clear_completion_signal();
                        break
                    },
                    CompletionSignal::Continue => {
                        self.ctx.clear_completion_signal();
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
        self.ctx.exit_scope();
    }
    
    fn eval_switch(&mut self, stmt: &mut ast::SwitchStatement<F>) {
        unimplemented!()
        /* 
        let discriminant = self.expression(&mut stmt.discriminant);
        ctx.enter_switch_scope();
        for case in stmt.cases.iter_mut() {
            if case.test.is_none() || discriminant == self.expression(case.test.as_mut().unwrap()) {
                for stmt in case.consequent.iter_mut() {
                    eval_statement(stmt);
                }
                break;
            }
        }
        ctx.exit_switch_scope();
        */
    }
    
    fn eval_break(&mut self, stmt: &mut ast::BreakStatement) {
        self.ctx.termination_break()
    }
    
    fn eval_continue(&mut self, stmt: &mut ast::ContinueStatement) {
        self.ctx.termination_continue()
    }
    
    fn eval_return(&mut self, stmt: &mut ast::ReturnStatement<F>) {
        let val = stmt.argument.as_mut().map(|x| self.expression(x).unwrap()); // TODO: handle error
        self.ctx.termination_return(&val)
    }
    
    fn eval_throw(&mut self, stmt: &mut ast::ThrowStatement<F>) {
        let val = self.expression(&mut stmt.argument).unwrap(); // TODO: handle error
        self.ctx.termination_throw(&val)
    }
    
    pub fn expression(&mut self, expr: &mut <F as NodeF>::Expression) -> Option<JSValue> {
        match &mut expr.expression {
            ast::Expression::Literal(lit) => Some(self.eval_literal(lit)),
            ast::Expression::Array(arr) => self.eval_array(arr),
            ast::Expression::Object(obj) => self.eval_object(obj),
            ast::Expression::Variable(ident) => self.eval_variable(&mut ident.name),
            ast::Expression::Binary(bin) => self.eval_binary(bin),
            ast::Expression::Unary(unary) => self.eval_unary(unary),
            ast::Expression::Conditional(cond) => self.eval_conditional(cond),
            ast::Expression::Logical(logical) => self.eval_logical(logical),
            ast::Expression::Call(call) => self.eval_call(call),
            ast::Expression::Update(update) => self.eval_update(update),
            ast::Expression::Member(index) => self.eval_member(index),
            ast::Expression::Assignment(assign) => self.eval_assignment(assign),
            ast::Expression::Function(func) => self.function(func),
            ast::Expression::ArrowFunction(func) => self.eval_arrow_function(func),
            ast::Expression::Parenthesized(paren) => self.expression(&mut paren.expression),
        }
    }
    
    #[inline]
    fn eval_literal(&mut self, lit: &mut Literal) -> JSValue {
        match lit {
            Literal::Undefined => JSValue::Undefined,
            Literal::Boolean(b) => JSValue::boolean(b.0),
            Literal::Number(NumberLiteral::SMI(n)) => JSValue::number(*n as i32), // TODO
            Literal::String(s) => JSValue::string(s.0.clone()),
        }
    }
    
    #[inline]
    fn eval_array(&mut self, arr: &mut ast::ArrayExpression<F>) -> Option<JSValue> {
        let mut elements = Vec::new();
        for element in arr.elements.iter_mut() {
            match element {
                ast::ParameterElement::Parameter(param) => {
                    elements.push(self.expression(param)?);
                }
                _ => unimplemented!(),
            }
        }
        Some(JSValue::array(elements))
    }
    
    #[inline]
    fn eval_object(&mut self, obj: &mut ast::ObjectExpression<F>) -> Option<JSValue> {
        let mut properties = Vec::new();
        for property in obj.properties.iter_mut() {
            match property {
                ast::ObjectElement::KeyValue(key, value) => properties.push((key.clone(), self.expression(value)?)),
                ast::ObjectElement::Shorthand(key) => properties.push((key.clone(), self.eval_variable(key)?)),
                _ => unimplemented!()
            }
        };
    
        Some(JSValue::object(properties))
    }
    
    #[inline]
    fn eval_variable(&mut self, ident: &mut node::Identifier) -> Option<JSValue> {
        self.ctx.get_binding_value(&ident)
    }
    
    #[inline]
    fn eval_binary(&mut self, bin: &mut ast::BinaryExpression<F>) -> Option<JSValue> {
        // eval for all types for strict (non) equal operation
        match bin.operator {
            ast::BinaryOperator::StrictEqual => {
                let left = self.expression(&mut bin.left)?;
                let right = self.expression(&mut bin.right)?;
                return Some(JSValue::boolean(left.equal(&right)))
            },
            ast::BinaryOperator::StrictNotEqual => {
                let left = self.expression(&mut bin.left)?;
                let right = self.expression(&mut bin.right)?;
                return Some(JSValue::boolean(!left.equal(&right)))
            },
            _ => {},
        }
    
        // eval for only number type for anything else
    
        let mut operand = self.expression(&mut bin.left)?;
        
        let mut left = operand.as_mut_number();
        if left.is_none() {
            // TODO: throw error
            return None
        }
        let mut left = left.unwrap();
    
        let mut argument = self.expression(&mut bin.right)?;
    
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
    
    fn eval_unary(&mut self, unary: &mut ast::UnaryExpression<F>) -> Option<JSValue> {
        let mut argument = self.expression(&mut unary.argument)?;
        match unary.operator {
            ast::UnaryOperator::Minus => { argument.as_mut_number()?.negate(); },
            // ast::UnaryOperator::Plus => argument.as_mut_number()?.positive(),
            // ast::UnaryOperator::Tilde => argument.bit_not(),
            // ast::UnaryOperator::TypeOf => operand.type_of(),
            // ast::UnaryOperator::Void => JSValue::Undefined,
            ast::UnaryOperator::Bang => { argument.as_mut_boolean()?.not(); },
            _ => unimplemented!(),
        }

        Some(argument)
    }
    
    
    
    fn eval_assignment(&mut self, expr: &mut ast::AssignmentExpression<F>) -> Option<JSValue> {
        let value = self.expression(&mut expr.right)?;
        match &mut expr.left {
            ast::LValue::Variable(ident) => {
                self.ctx.set_binding_value(&ident, &value);
            }
            ast::LValue::Member(index) => {
                let mut object = self.expression(&mut index.object)?;
                let property = match &mut index.property {
                    ast::Member::Property(ident) => ident.clone(), 
                    ast::Member::Computed(expr) => {
                        let value = self.expression(expr)?.to_string();
                        Identifier::new(value)
                    }
                };
                object.set_property(&property, value.clone());
            }
        }
        Some(value)
    }
    
    fn eval_conditional(&mut self, expr: &mut ast::ConditionalExpression<F>) -> Option<JSValue> {
        let discriminant = self.expression(&mut expr.test)?;
        if discriminant.is_truthy() {
            self.expression(&mut expr.consequent)
        } else {
            self.expression(&mut expr.alternate)
        }
    }
    
    fn eval_logical(&mut self, expr: &mut ast::LogicalExpression<F>) -> Option<JSValue> {
        match expr.operator {
            ast::LogicalOperator::And => {
                let left = self.expression(&mut expr.left)?;
                if !left.is_truthy() {
                    Some(left)
                } else {
                    self.expression(&mut expr.right)
                }
            },
            ast::LogicalOperator::Or => {
                let left = self.expression(&mut expr.left)?;
                if left.is_truthy() {
                    Some(left)
                } else {
                    self.expression(&mut expr.right)
                }
            },
            ast::LogicalOperator::Coalesce => {
                let left = self.expression(&mut expr.left)?;
                if !left.is_undefined() {
                    Some(left)
                } else {
                    self.expression(&mut expr.right)
                }
            },
        }
    }
    
    fn eval_call(&mut self, call: &mut ast::CallExpression<F>) -> Option<JSValue> {
        let mut callee = self.expression(&mut call.callee)?;
        let mut args = Vec::new();
        for arg in call.arguments.iter_mut() {
            match arg {
                ParameterElement::Parameter(expr) => {
                    args.push(self.expression(expr)?);
                },
                _ => unimplemented!(),
            }
        }
        callee.call(&mut self.ctx, args)
    }
    
    fn eval_update(&mut self, expr: &mut ast::UpdateExpression<F>) -> Option<JSValue> {
        unimplemented!()
    }
    
    fn eval_member(&mut self, expr: &mut ast::MemberExpression<F>) -> Option<JSValue> {
        let object = self.expression(&mut expr.object)?;
        match &mut expr.property {
            ast::Member::Property(ident) => {
                object.get_property(ident)
            },
            ast::Member::Computed(expr) => {
                let property = self.expression(expr)?;
                object.get_computed_property(&property)
            },
        }
    }
    
    pub fn function(&mut self, expr: &mut node::Function) -> Option<JSValue> {
        Some(JSValue::function(self.ctx.function_environment(expr.capture_variables())?, expr.clone()))
    }
    
    fn eval_arrow_function(&mut self, expr: &mut ast::ArrowFunctionExpression<F>) -> Option<JSValue> {
        unimplemented!()
    }
}

