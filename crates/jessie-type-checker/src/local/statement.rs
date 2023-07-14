
pub fn infer_top_level_function(state: &mut InferenceState, function: &Function) {
    for statement in function.statements {
        infer_statement(state, &statement);
    }
}

pub fn infer_statement(state: &mut InferenceState, statement: &Statement) {
    match statement {
        Statement::LocalDeclaration(declarations) => {
            for declaration in declarations {
                infer_local_declaration(state, declaration);
            }
        },
        Statement::FunctionDeclaration(declaration) => {
            infer_function_declaration(state, declaration);
        },
        Statement::Block(block) => {
            for statement in &block.0 {
                infer_statement(state, statement);
            }
        },
        Statement::IfStatement(if_statement) => {
            infer_expression(state, &if_statement.condition);
            infer_statement(state, &if_statement.consequent);
            match &if_statement.alternate {
                ElseArm::NoElse => {},
                ElseArm::Else(block) => {
                    infer_statement(state, block);
                },
                ElseArm::ElseIf(else_if) => {
                    infer_statement(state, else_if);
                },
            }
        },
        Statement::WhileStatement(while_statement) => {
            infer_expression(state, &while_statement.condition);
            infer_statement(state, &while_statement.body);
        },
        Statement::Continue => {},
        Statement::Break => {},
        Statement::Return(expr) => {
            let inferred = infer_expression(state, expr);
            state.assert_return_type(inferred);
        },
        Statement::ReturnEmpty => {},
        Statement::Throw(expr) => {
            let inferred = infer_expression(state, expr);
            state.assert_throw_type(inferred);
        },
        Statement::ExprStatement(expr) => {
            infer_expression(state, expr);
        },
    }
}

pub fn infer_local_declaration(state: &mut InferenceState, declaration: &DeclarationIndex) {
    
}

pub fn infer_function_declaration(state: &mut InferenceState, declaration: &DeclarationIndex) {
    
}
