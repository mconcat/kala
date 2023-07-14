use std::intrinsics::unreachable;

use jessie_ast::{Function, Statement, DeclarationIndex, Expr, DataLiteral, BinaryExpr, BinaryOp};

use crate::Type;

pub fn infer_expression(state: &mut InferenceState, expr: &Expr) -> Type {
    match expr {
        Expr::DataLiteral(lit) => infer_literal(state, expr),
        Expr::Array(array) => infer_array(state, array),
        Expr::Record(record) => infer_record(state, record),
        Expr::ArrowFunc(arrow_func) => infer_func(state, arrow_func),
        // TODO: recursive function
        Expr::FunctionExpr(func) => infer_func(state, func),
        Expr::Assignment(assignment) => infer_assignment(state, assignment),
        Expr::CondExpr(cond_expr) => infer_cond_expr(state, cond_expr),
        Expr::BinaryExpr(binary_expr) => infer_binary_expr(state, binary_expr),
        Expr::UnaryExpr(unary_expr) => infer_unary_expr(state, unary_expr),
        Expr::CallExpr(call_expr) => infer_call_expr(state, call_expr),
        Expr::ParenedExpr(parened_expr) => infer_expression(state, &parened_expr),
        Expr::Variable(variable) => infer_variable(state, variable),
        Expr::Spread(spread) => unimplemented!("spread operator is not implemented yet"),
    }
}

pub fn infer_literal(state: &mut InferenceState, lit: &DataLiteral) -> Type {
    match lit {
        DataLiteral::Number(_) => Type::Number,
        DataLiteral::String(_) => Type::String,
        DataLiteral::False | DataLiteral::True => Type::Boolean,
        DataLiteral::Null => Type::Null,
        DataLiteral::Undefined => Type::Undefined,
        DataLiteral::BigInt(_) => Type::BigInt,
        DataLiteral::Decimal(_) => Type::Number,
    }
}

pub fn infer_array(state: &mut InferenceState, array: &Array) -> Type {
    let mut types = Vec::with_capacity(array.0.len());
    for expr in &array.0 {
        types.push(infer_expression(state, expr));
    }
    Type::Array(types)
}

pub fn infer_record(state: &mut InferenceState, record: &Record) -> Type {
    let mut fields = Vec::with_capacity(record.0.len());
    for key_value in &record.0 {
        fields.push((key_value.key.clone(), infer_expression(state, &key_value.value)));
    }
    Type::Record(fields)
}

pub fn infer_func(state: &mut InferenceState, func: &Function) -> Type {
    let mut params = Vec::with_capacity(func.params.len());
    for param in &func.params {
        params.push(infer_expression(state, param));
    }
    let return_type = infer_expression(state, &func.body);
    Type::Function(params, Box::new(return_type))
}

pub fn infer_assignment(state: &mut InferenceState, assignment: &Assignment) -> Type {
}

pub fn infer_cond_expr(state: &mut InferenceState, cond_expr: &CondExpr) -> Type {
    infer_expression(state, cond_expr.0);
    let t1 = infer_expression(state, cond_expr.1);
    let t2 = infer_expression(state, cond_expr.2);
    state.unify(t1, t2)
}

pub fn infer_binary_expr(state: &mut InferenceState, binary_expr: &BinaryExpr) -> Type {
    match binary_expr.0 {
        BinaryOp::Or | BinaryOp::And => {
            let t1 = infer_expression(state, binary_expr.1);
            let t2 = infer_expression(state, binary_expr.2);
            state.unify(t1, t2)
        },
        BinaryOp::Coalesce => {
            let t1 = infer_expression(state, binary_expr.1);
            let t2 = infer_expression(state, binary_expr.2);
            unimplemented!("coalesce operator is not implemented yet")
        },
        BinaryOp::BitOr | BinaryOp::BitXor | BinaryOp::BitAnd => {
            let t1 = infer_expression(state, binary_expr.1);
            let t2 = infer_expression(state, binary_expr.2);
            state.unify(t1, t2)
        },
    }
}