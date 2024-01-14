use crate::{state::ScopeState, scope_variable, scope_function};
use jessie_ast::{Expr, LValue, CallPostOp, LValueCallPostOp, PropDef};

pub fn scope_expression(state: &mut ScopeState, expr: &mut Expr) -> Result<(), &'static str> {
    match expr {
        Expr::DataLiteral(_) => Ok(()),
        Expr::Array(arr) => {
            for elem in arr.as_mut().0.iter_mut() {
                scope_expression(state, elem)?
            }
            Ok(())
        },
        Expr::Record(rec) => {
            for prop in rec.as_mut().0.iter_mut() {
                match prop {
                    PropDef::KeyValue(_, expr) => scope_expression(state, expr)?,
                    PropDef::Shorthand(_, var) => scope_variable(state, var.as_mut())?,
                    PropDef::Spread(var) => scope_expression(state, var)?,
                    PropDef::Getter(getter) => scope_function(state, getter.as_mut())?,
                    PropDef::Setter(setter) => scope_function(state, setter.as_mut())?,
                }
            }
            Ok(())
        },
        Expr::Function(func) => scope_function(state, func),
        Expr::Assignment(assign) => {
            match &mut assign.as_mut().1 {
                LValue::CallLValue(lvalue) => {
                    scope_expression(state, &mut lvalue.as_mut().expr)?;
                    for op in lvalue.post_ops.iter_mut() {
                        match op {
                            LValueCallPostOp::Index(index) => scope_expression(state, index)?,
                            LValueCallPostOp::Member(_) => (),
                        }
                    }
                },
                LValue::Variable(var) => scope_variable(state, var.as_mut())?,
            }
            scope_expression(state, &mut assign.as_mut().2)
        },
        Expr::CondExpr(expr) => {
            scope_expression(state, &mut expr.0)?;
            scope_expression(state, &mut expr.1)?;
            scope_expression(state, &mut expr.2)
        },
        Expr::BinaryExpr(expr) => {
            scope_expression(state, &mut expr.1)?;
            scope_expression(state, &mut expr.2)
        }
        Expr::UnaryExpr(expr) => {
            scope_expression(state, &mut expr.expr)
        },
        Expr::CallExpr(expr) => {
            scope_expression(state, &mut expr.expr)?;
            for op in expr.post_ops.iter_mut() {
                match op {
                    CallPostOp::Index(expr) => scope_expression(state, expr)?,
                    CallPostOp::Member(_) => (),
                    CallPostOp::Call(args) => for arg in args.iter_mut() {
                        scope_expression(state, arg)?
                    }
                }
            }
            Ok(())
        },
        Expr::ParenedExpr(expr) => scope_expression(state, expr.as_mut()),
        Expr::Variable(var) => scope_variable(state, var),
        Expr::Spread(spread) => unimplemented!("TODO"),
    }
}