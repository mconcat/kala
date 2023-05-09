#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::jessie_parser::{expression, param};
    use crate::parser::ParserState;
    use crate::lexer::*;
    use jessie_ast::*;

    fn expr_test_cases() -> Vec<(&'static str, Expr)> {
        vec![
        ("undefined", Expr::DataLiteral(DataLiteral::Undefined)),
        ("3", Expr::new_number(3)),
        ("5+6", Expr::new_add(Expr::new_number(5), Expr::new_number(6))),
        ("function f(x) { return x; }", {
            let param_decl = Rc::new(Declaration::Parameters(vec![Pattern::Variable("x".into(), None)]));
            Expr::FunctionExpr(Box::new(Function{
            name: Some("f".into()),
            parameters: param_decl.clone(),
            typeann: None,
            statements: vec![Statement::Return(Some(Expr::Variable(UseVariable::new("x", param_decl.clone()))))],
            expression: None,
            scope: Scope{
                declarations: Some(Box::new(vec![Rc::new(Declaration::Parameters(vec![Pattern::Variable("x".into(), None)]))])),
            },
        }))
        }),
        ("function f(x, y) {
            return x+y;   
        }", {
            let param_decl = Rc::new(Declaration::Parameters(vec![Pattern::Variable("x".into(), None), Pattern::Variable("y".into(), None)]));
            Expr::FunctionExpr(Box::new(Function{
            name: Some("f".into()), 
            parameters: Rc::new(Declaration::Parameters(vec![Pattern::Variable("x".into(), None), Pattern::Variable("y".into(), None)])), 
            typeann: None,
            statements: vec![Statement::Return(Some(Expr::new_add(Expr::Variable(UseVariable::new("x", param_decl.clone())), Expr::Variable(UseVariable::new("y", param_decl.clone())))))],
            expression: None,
            scope: Scope{
                declarations: Some(Box::new(vec![param_decl.clone()])),
            },
        }))}),
            /* 
            // Excluded due to destructing parameter
        ("function f(x, [y, z]) {
            let t = x+y;
            return z;
        }", Expr::FunctionExpr(Box::new(Function(
            Some("f".to_string()), 
            vec![
                Pattern::Variable("x".to_string(), None),
                Pattern::ArrayPattern(vec![Pattern::Variable("y".to_string(), None), Pattern::Variable("z".to_string(), None)], None),    
            ], 
            None,
            BlockOrExpr::Block(Block::new(vec![
                Statement::Declaration(Declaration { kind: DeclarationKind::Let, bindings: vec![Binding::VariableBinding("t".to_string(), Some(Expr::new_add(Expr::Variable("x".to_string()), Expr::Variable("y".to_string()))))] }),
                Statement::Return(Some(Expr::Variable("z".to_string()))),
            ])))))),
            */
        ("[3, v, true, {}, ...g, 123n, 4.67]", Expr::Array(Array( 
            vec![
                Element::Expr(Expr::new_number(3)),
                Element::Expr(Expr::Variable(UseVariable::new_unbound("v"))),
                Element::Expr(Expr::DataLiteral(DataLiteral::True)),
                Element::Expr(Expr::Record(Record(vec![]))),
                Element::Spread(Expr::Variable(UseVariable::new_unbound("g"))),
                Element::Expr(Expr::DataLiteral(DataLiteral::Bigint("123".to_string()))),
                Element::Expr(Expr::DataLiteral(DataLiteral::Decimal("4.67".to_string()))),
            ]
        ))),
        ("{x: y, t    : [p], ...o, short}", Expr::Record(Record( 
            vec![
                PropDef::KeyValue(PropName::Ident("x".into()), Expr::Variable(UseVariable::new_unbound("y"))),
                PropDef::KeyValue(PropName::Ident("t".into()), Expr::Array(Array (vec![Element::Expr(Expr::Variable(UseVariable::new_unbound("p")))]))),
                PropDef::Spread(Expr::Variable(UseVariable::new_unbound("o"))),
                PropDef::Shorthand(UseVariable::new_unbound("short")),
            ]
        ))),
        ("(3+2)*1&&undefined/5&x-7||true==={x:y}%6", Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Or, 
            Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::And, 
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mul, 
                    Expr::ParenedExpr(Box::new(Expr::new_add(Expr::new_number(3), Expr::new_number(2)))), 
                    Expr::new_number(1)
                ))), 
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseAnd, 
                    Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Div, 
                        Expr::DataLiteral(DataLiteral::Undefined), 
                        Expr::new_number(5)
                    ))), 
                    Expr::new_sub(Expr::Variable(UseVariable::new_unbound("x")), Expr::new_number(7))
                )))))),
            Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::StrictEqual, 
                Expr::DataLiteral(DataLiteral::True), 
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mod, 
                    Expr::Record(Record(vec![PropDef::KeyValue(PropName::Ident("x".into()), Expr::Variable(UseVariable::new_unbound("y")))])), 
                    Expr::new_number(6)
                )))))))))),
        ("x.y.z", Expr::CallExpr(Box::new(CallExpr { 
            expr: Expr::CallExpr(Box::new(CallExpr {
                expr: Expr::Variable(UseVariable::new_unbound("x")),
                post_op: CallPostOp::MemberPostOp(MemberPostOp::Member("y".to_string()))
            })),
            post_op: CallPostOp::MemberPostOp(MemberPostOp::Member("z".to_string()))
        }))),
        ("function f(x) {
            const y = 3;
            {
                let z = 5;
                return x+y+z;
            }
        }", {
            let param_decl = Rc::new(Declaration::Parameters(vec![Pattern::Variable("x".into(), None)]));
            let decl_y = Rc::new(Declaration::Const(vec![Binding::VariableBinding("y".into(), Some(Expr::new_number(3)))]));
            let decl_z = Rc::new(Declaration::Let(vec![Binding::VariableBinding("z".into(), Some(Expr::new_number(5)))]));
            Expr::FunctionExpr(Box::new(Function { 
            name: Some("f".into()), 
            parameters: param_decl.clone(),
            typeann: None,
            expression: None, 
            statements: vec![
                Statement::Declaration(decl_y.clone()), 
                Statement::Block(Box::new(Block::new(
                    vec![
                    Statement::Declaration(decl_z.clone()),
                    Statement::Return(Some(Expr::new_add(Expr::new_add(Expr::Variable(UseVariable::new("x", param_decl.clone())), Expr::Variable(UseVariable::new("y", decl_y.clone()))), Expr::Variable(UseVariable::new("z", decl_z.clone())))))
                    ],
                    Scope{
                        declarations: Some(Box::new(vec![decl_z.clone()])),
                    },
                )))
            ],
            scope: Scope{
                declarations: Some(Box::new(vec![param_decl, decl_y])),
            },
        }))
            })
        ]
    }

    #[test]
    fn it_works() {
        expr_test_cases().iter().for_each(|case| {
            println!("===========");
            println!("test for {}", case.0);
            let mut lexer_state = ParserState::new(Str(case.0));
            println!("lexer_state: {:?}", lexer_state);
            let tokenstream = lex(&mut lexer_state).unwrap();
            println!("tokenstream: {:?}", tokenstream);
            let mut state = ParserState::new(VecToken(tokenstream));

            let result = expression(&mut state);
            assert_eq!(result, Ok(case.1.clone()));
            println!("success, result: {:?}", result);
        });
    }
}
