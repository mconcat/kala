use std::fmt::Binary;

use jessie_ast::*;
use crate::types::*;



pub fn infer_declaration(state: &mut InferenceState, x: DeclarationPointer) -> Result<(), String> {

}

pub fn infer_block(state: &mut InferenceState, x: Box<Block>) -> Result<(), String> {

}

pub fn infer_if_statement(state: &mut InferenceState, x: Box<IfStatement>) -> Result<(), String> {
    
}

pub fn infer_while_statement(state: &mut InferenceState, x: Box<WhileStatement>) -> Result<(), String> {

}

pub fn infer_return(state: &mut InferenceState, x: Option<Expr>) -> Result<(), String> {

}

pub fn infer_expr(state: &mut InferenceState, x: Expr, ty: &mut Option<BoundType>) -> Result<(), String> {
    match x {
        Expr::DataLiteral(x) => infer_data_literal(state, &x, ty),
        Expr::Array(x) => infer_array(state, &x, ty),
        Expr::Record(x) => infer_record(state, &x, ty),
        Expr::ArrowFunc(x) => infer_arrow_func(state, &x, ty),
        Expr::FunctionExpr(x) => infer_function_expr(state, &x, ty),
        Expr::Assignment(x) => infer_assignment(state, &x, ty),
        Expr::CondExpr(x) => infer_cond_expr(state, &x, ty),
        Expr::BinaryExpr(x) => infer_binary_expr(state, &x, ty),
        Expr::UnaryExpr(x) => infer_unary_expr(state, &x, ty),
        Expr::CallExpr(x) => infer_call_expr(state, &x, ty),
        Expr::ParenedExpr(x) => infer_expr(state, &x, ty),
        Expr::Variable(x) => infer_variable(state, &x, ty),
    }
}

pub fn infer_data_literal(state: &mut InferenceState, x: &DataLiteral, ty: &mut Option<BoundType>) -> Result<(), String> {
    let ity = match x {
        DataLiteral::Bigint(_) => Type::Bigint,
        DataLiteral::String(s) => Type::StringLiteral(s.clone()),
        DataLiteral::False | DataLiteral::True => Type::Boolean,
        DataLiteral::Undefined => Type::Undefined,
        DataLiteral::Null => unimplemented!("null"),
        DataLiteral::Integer(i) => Type::Number, // TODO
        DataLiteral::Decimal(_) => Type::Number,
    };

    if let Some(ty) = ty && ty != ity {
        Err(format!("Type mismatch: {:?} != {:?}", ty, ity));
    } else {
        *ty = Some(ity);
        Ok(())
    }
}

//                           => |- [] <- []T
// |- x <- T                 => |- [x] <- []T
// |= x <- T, y <- T         => |- [x, y] <- []T
// |= x <- T, y <- T, z <- T => |- [x, y, z] <- []T
// ...
// |- x <- T, y <- U => |- [x, y] <- [T, U]
// |- x <- T, y <- U, z <- V => |- [x, y, z] <- [T, U, V]
// ...
pub fn infer_array(state: &mut InferenceState, x: &Array, ty: &mut Option<BoundType>) -> Result<(), String> {
    Err("TODO".to_string())
}

// |- x -> T 
pub fn infer_record(state: &mut InferenceState, x: &Record, ty: &mut Option<BoundType>) -> Result<(), String> {
    Err("TODO".to_string())
}

pub fn infer_arrow_func(state: &mut InferenceState, x: &Box<Function>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_function_expr(state: &mut InferenceState, x: &Box<Function>, ty: &mut Option<BoundType>) -> Result<(), String> {
    // 1. Assign type variable for each of the parameters 
    // 2. Construct ground facts: known function signatures and parameter types
    // 3. Iterate through the body statements
    // 4. Accumulate type information and apply to the variables
    //
    // After we have a row polymorphic type inferred by the ground facts and
    // the abstract interpretation of the body statement,
    // we can later constraint the parameter types by walking through all the 
    // usage of this function. For now, lets follow the polymorphic/megaorphic
    // convention of V8 optimization approach; specialize for less or equal than
    // 4 different types, and use adapter struct for more.
    //

    // enter scope
    let parent = state.enter_function_scope();

    // Assign type variable to the function itself    
    state.declare_function(x.name, None)?;

    let Declaration::Parameters(parameters) = x.parameters.as_ref();
    // Assign type variables to function parameters
    for param in parameters {
        state.declare_parameter(param, None)
    }

    // Iterate through the body statements
    for statement in x.statements {
        match statement {
            Statement::Declaration(s) => {
                match s.as_ref() {
                    Declaration::Const(b) => state.declare_const(b, None),
                    Declaration::Let(b) => state.declare_let(b, None),
                    Declaration::Function(b) => state.declare_function(b, None),
                    Declaration::Parameters(_) => unreachable!("asdf"),
                }
            }
            Statement::Block(s) => {
                let block_parent = state.enter_block_scope();
                infer_block(state, s.statements);
                state.exit_block_scope(block_parent);
            }
            Statement::IfStatement(s) => {
                // Conditional expression is inferred to be type of either:
                // - Boolean
                // - Optional type(T?)
                // TODO: flow type checking
                state.assert_conditional(s.condition);
                infer_block(state, s.consequent);
            }
            Statement::WhileStatement(s) => {
                state.assert_conditional(s.condition);
                infer_block(state, s.body)
            }
            Statement::Return(s) => {
                let ty = s.map(|s| infer_expr(state, s, state.return_type()));
            }
            Statement::Throw(s) => {
                infer_expression(state, s);
            },
            Statement::ExprStatement(s) => infer_expr(state, s, None)?,
            Statement::Continue | Statement::Break => unreachable!("should not appear outside of a loop, todo: parser could make this happen, return error instead of unreachable"),
            _ => unimplemented!("asdf"),
        }
    }

    Ok(())
}

pub fn infer_function_body_statement(state: &mut InferenceState, x: &Statement);

pub fn infer_assginment(state: &mut InferenceState, x: &Box<Assignment>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_cond_expr(state: &mut InferenceState, x: &Box<CondExpr>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_binary_expr(state: &mut InferenceState, x: &Box<BinaryExpr>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_unary_expr(state: &mut InferenceState, x: &Box<UnaryExpr>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_call_expr(state: &mut InferenceState, x: &Box<CallExpr>, ty: &mut Option<BoundType>) -> Result<(), String> {

}

pub fn infer_variable(state: &mut InferenceState, x: &UseVariable, ty: &mut Option<BoundType>) -> Result<(), String> {
    
}

