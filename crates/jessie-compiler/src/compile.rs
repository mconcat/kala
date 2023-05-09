use std::fmt::Binary;

use jessie_ast::*;
use jessie_interpreter::opcode::Opcode;

struct CompilerState {
    opcodes: Vec<u8>,    
}

pub fn expr(state: &mut CompilerState, expr: Expr) -> Result<(), String> {
    match expr {
        Expr::DataLiteral(x) => {
            data_literal(state, x)
        },
        Expr::Array(x) => {
            array(state, x)
        },
        Expr::Record(x) => {
            record(state, x)
        },
        Expr::ArrowFunc(x) => {
            arrow_func(state, x)
        },
        Expr::FunctionExpr(x) => {
            function_expr(state, x)
        },
        Expr::Assignment(x) => {
            assignment(state, x)
        },
        Expr::CondExpr(x) => {
            cond_expr(state, x)
        },
        Expr::BinaryExpr(x) => {
            binary_expr(state, x)
        },
        Expr::UnaryExpr(x) => {
            unary_expr(state, x)
        },
        Expr::CallExpr(x) => {
            call_expr(state, x)
        },
        Expr::ParenedExpr(x) => {
            expr(state, *x.expr)
        },
        Expr::Variable(x) => {
            variable(state, x)
        },
    }
}

pub fn data_literal(state: &mut CompilerState, data: DataLiteral) -> Result<(), String> {
    match data {
        DataLiteral::True => {
            state.opcodes.push(Opcode::True);
            return Ok(())
        },
        DataLiteral::False => {
            state.opcodes.push(Opcode::False);
            return Ok(())
        },
        DataLiteral::Null => {
            state.opcodes.push(Opcode::Null);
            return Ok(())
        },
        DataLiteral::Undefined => {
            state.opcodes.push(Opcode::Undefined);
            return Ok(())
        },
        DataLiteral::Integer(x) => {
            integer(state, x)
        },
        DataLiteral::Decimal(x) => {
            decimal(state, x)
        }
        DataLiteral::String(x) => {
            string(state, x)
        },
        DataLiteral::Bigint(x) => {
            bigint(state, x)
        },
    }
}

pub fn integer(state: &mut CompilerState, s: String) -> Result<(), String> {
    let i = s.parse::<i64>().map_err(|_| format!("Could not parse {} as an integer", s))?;
    if i >= 0 && i <= 255 {
        state.opcodes.push(Opcode::Integer1);
        state.opcodes.push(i as u8);
    } else if i >= 0 && i <= 65535 {
        state.opcodes.push(Opcode::Integer2);
        state.opcodes.push((i >> 8) as u8);
        state.opcodes.push((i & 0xff) as u8);
    } else if i >= 0 && i <= 4294967295 {
        state.opcodes.push(Opcode::Integer4);
        state.opcodes.push((i >> 24) as u8);
        state.opcodes.push(((i >> 16) & 0xff) as u8);
        state.opcodes.push(((i >> 8) & 0xff) as u8);
        state.opcodes.push((i & 0xff) as u8);
    } else {
        state.opcodes.push(Opcode::Number);
        state.opcodes.push((i >> 56) as u8);
        state.opcodes.push(((i >> 48) & 0xff) as u8);
        state.opcodes.push(((i >> 40) & 0xff) as u8);
        state.opcodes.push(((i >> 32) & 0xff) as u8);
        state.opcodes.push(((i >> 24) & 0xff) as u8);
        state.opcodes.push(((i >> 16) & 0xff) as u8);
        state.opcodes.push(((i >> 8) & 0xff) as u8);
        state.opcodes.push((i & 0xff) as u8);
    };
}

pub fn decimal(state: &mut CompilerState, s: String) -> Result<(), String> {
    let (i, f) = s.split_once('.').ok_or(format!("Could not parse {} as a decimal", s))?;

    if f.len() == 0 {
        return integer(state, i.to_string());
    }

    let i = i.parse::<i64>().map_err(|_| format!("Could not parse {} as an integer", i))?;

    let f = f.parse::<u64>().map_err(|_| format!("Could not parse {} as a decimal", f))?;
    
    state.opcodes.push(Opcode::Number);

    state.opcodes.push((f >> 56) as u8);
    state.opcodes.push(((f >> 48) & 0xff) as u8);
    state.opcodes.push(((f >> 40) & 0xff) as u8);
    state.opcodes.push(((f >> 32) & 0xff) as u8);
    state.opcodes.push(((f >> 24) & 0xff) as u8);
    state.opcodes.push(((f >> 16) & 0xff) as u8);
    state.opcodes.push(((f >> 8) & 0xff) as u8);
    state.opcodes.push((f & 0xff) as u8);

    state.opcodes.push((i >> 56) as u8);
    state.opcodes.push(((i >> 48) & 0xff) as u8);
    state.opcodes.push(((i >> 40) & 0xff) as u8);
    state.opcodes.push(((i >> 32) & 0xff) as u8);
    state.opcodes.push(((i >> 24) & 0xff) as u8);
    state.opcodes.push(((i >> 16) & 0xff) as u8);
    state.opcodes.push(((i >> 8) & 0xff) as u8);
    state.opcodes.push((i & 0xff) as u8);

    Ok(())
}

pub fn string(state: &mut CompilerState, s: String) -> Result<(), String> {
    let mut bytes = s.as_bytes().to_vec();
    bytes.push(0);
    if bytes.len() <= 255 {
        state.opcodes.push(Opcode::String1);
        state.opcodes.push(bytes.len() as u8);
    } else if bytes.len() <= 65535 {
        state.opcodes.push(Opcode::String2);
        state.opcodes.push((bytes.len() >> 8) as u8);
        state.opcodes.push((bytes.len() & 0xff) as u8);
    } else {
        state.opcodes.push(Opcode::String4);
        state.opcodes.push((bytes.len() >> 24) as u8);
        state.opcodes.push(((bytes.len() >> 16) & 0xff) as u8);
        state.opcodes.push(((bytes.len() >> 8) & 0xff) as u8);
        state.opcodes.push((bytes.len() & 0xff) as u8);
    }

    state.opcodes.append(&mut bytes);

    Ok(())
}

pub fn bigint(state: &mut CompilerState, s: String) -> Result<(), String> {
    unimplemented!()
}

pub fn array(state: &mut CompilerState, arr: Array) -> Result<(), String> {
    unimplemented!()    
}

pub fn record(state: &mut CompilerState, data: Record) -> Result<(), String> {
    unimplemented!()
}

pub fn arrow_func(state: &mut CompilerState, data: Function) -> Result<(), String> {
    unimplemented!()
}

// a function is a pair of (dynamically constructed lexical scope, static function code).
pub fn function_expr(state: &mut CompilerState, data: Function) -> Result<(), String> {
    // closure is defined in state-level declarations, just like a top level function
    state.declare_closure(data.name, |state| {
        // add_captures adds opcodes to take the captured values from the stack
        state.add_captures(data.captures);

        state.allocate_frame_return(data.typeann);
        state.allocate_frame_parameters(data.parameters);
        state.allocate_frame_locals(data.scope);
        
        if let Some(expr) = data.expression {
            expr(state)?;
            state.opcodes.push(Opcode::Return);
            state.cleanup_closure();
            return Ok(());
        } 

        for stmt in data.statements {
            stmt(state)?;
        }

        state.cleanup_closure();
        Ok(())
    });
}

pub fn assignment(state: &mut CompilerState, data: Assignment) -> Result<(), String> {
    let reference = match data.0 {
        LValue::Index(obj, idx) => {
            expr(state, obj)?;
            expr(state, idx)?;
            state.reference_field();
        },
        LValue::Member(obj, mem) => {
            expr(state, obj)?;
            state.push_string(mem);
            state.reference_field();
        },
        LValue::Variable(var) => {
            state.reference_local(var);
        },
    }

    if data.1 == AssignOp::Assign {
        expr(state, *data.2)?;
        state.opcodes.push(Opcode::Assign);
        return Ok(());
    }

    eval_lvalue(reference)?;
    expr(state, *data.2)?;
    match data.1 {
        AssignOp::Assign => unreachable!(),
        AssignOp::AssignAdd => state.opcodes.push(Opcode::Add),
        AssignOp::AssignSub => state.opcodes.push(Opcode::Sub),
        AssignOp::AssignMul => state.opcodes.push(Opcode::Mul),
        AssignOp::AssignDiv => state.opcodes.push(Opcode::Div),
        AssignOp::AssignMod => state.opcodes.push(Opcode::Mod),
        AssignOp::AssignBitAnd => state.opcodes.push(Opcode::BitAnd),
        AssignOp::AssignBitOr => state.opcodes.push(Opcode::BitOr),
        AssignOp::AssignBitXor => state.opcodes.push(Opcode::BitXor),
        // AssignOp::AssignLShift => state.opcodes.push(Opcode::BitLeftShift),
        _ => unimplemented!()
    }

    state.opcodes.push(Opcode::Assign);
    return Ok(())
}

pub fn cond_expr(state: &mut CompilerState, data: CondExpr) -> Result<(), String> {
    expr(state, *data.0)?;
    let condition = enter_if()?;
    expr(state, *data.1)?;
    let consequent = enter_else()?;
    expr(state, *data.2)?;
    let alternative = end_if()?;
    settle_if(condition, consequent, alternative)?;
    Ok(())
}

pub fn binary_expr(state: &mut CompilerState, data: BinaryExpr) -> Result<(), String> {
    match data.0 {
        BinaryOp::Add => state.opcodes.push(Opcode::Add),
        BinaryOp::Sub => state.opcodes.push(Opcode::Sub),
        BinaryOp::Mul => state.opcodes.push(Opcode::Mul),
        BinaryOp::Div => state.opcodes.push(Opcode::Div),
        BinaryOp::Mod => state.opcodes.push(Opcode::Mod),
        BinaryOp::Pow => state.opcodes.push(Opcode::Pow),
        BinaryOp::BitAnd => state.opcodes.push(Opcode::BitAnd),
        BinaryOp::BitOr => state.opcodes.push(Opcode::BitOr),
        BinaryOp::BitXor => state.opcodes.push(Opcode::BitXor),
        BinaryOp::BitLeftShift => state.opcodes.push(Opcode::BitLeftShift),
        BinaryOp::BitRightShift => state.opcodes.push(Opcode::BitRightShift),
        BinaryOp::StrictEqual => state.opcodes.push(Opcode::StrictEqual),
        BinaryOp::StrictNotEqual => state.opcodes.push(Opcode::StrictNotEqual),
        BinaryOp::LT => state.opcodes.push(Opcode::LT),
        BinaryOp::LTE => state.opcodes.push(Opcode::LTE),
        BinaryOp::GT => state.opcodes.push(Opcode::GT),
        BinaryOp::GTE => state.opcodes.push(Opcode::GTE),
        BinaryOp::And => logical_and(state, data.1, data.2),
        BinaryOp::Or => logical_or(state, data.1, data.2),
        BinaryOp::Coalesce => coalesce(state, data.1, data.2),
    }
}

pub fn unary_expr(state: &mut CompilerState, data: UnaryExpr) -> Result<(), String> {
    expr(state, data.expr)?;
    for op in data.ops {
        match op {
            UnaryOp::Not => state.opcodes.push(Opcode::Not),
            UnaryOp::Neg => state.opcodes.push(Opcode::Neg),
            UnaryOp::BitNot => state.opcodes.push(Opcode::BitNot),
            UnaryOp::Pos => state.opcodes.push(Opcode::Pos),
            UnaryOp::TypeOf => state.opcodes.push(Opcode::TypeOf),
        }
    }

    Ok(())
}

pub fn call_expr(state: &mut CompilerState, data: CallExpr) -> Result<(), String> {
    unimplemented!()
}

pub fn variable(state: &mut CompilerState, data: UseVariable) -> Result<(), String> {
    
}
