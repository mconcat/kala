#[cfg(test)]
mod tests {
    use crate::jessie_operation::BinaryOp;
    use crate::jessie_types::PropDef;
    use crate::jessie_types::*;
    use crate::jessie_parser::expression;
    use crate::parser::ParserState;
    use crate::lexer::*;
    
    fn expr_test_cases() -> Vec<(&'static str, Expr)> {
        vec![
        ("undefined", Expr::DataLiteral(DataLiteral::Undefined)),
        ("3", Expr::new_number(3)),
        ("5+6", Expr::new_add(Expr::new_number(5), Expr::new_number(6))),
        ("function f(x) { return x; }", Expr::FunctionExpr(Box::new(Function(
            Some("f".to_string()),
            vec![Pattern::Variable("x".to_string(), None)],
            None,
            BlockOrExpr::Block(Block::new(vec![Statement::Return(Some(Expr::Variable("x".to_string())))])),
        )))),
        ("function f(x, y) {
            return x+y;   
        }", Expr::FunctionExpr(Box::new(Function(
            Some("f".to_string()), 
            vec![Pattern::Variable("x".to_string(), None), Pattern::Variable("y".to_string(), None)], 
            None,
            BlockOrExpr::Block(Block::new(vec![Statement::Return(Some(Expr::new_add(Expr::Variable("x".to_string()), Expr::Variable("y".to_string()))))])))))),
            /* 
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
                Element::Expr(Expr::Variable("v".to_string())),
                Element::Expr(Expr::DataLiteral(DataLiteral::True)),
                Element::Expr(Expr::Record(Record(vec![]))),
                Element::Spread(Expr::Variable("g".to_string())),
                Element::Expr(Expr::DataLiteral(DataLiteral::Bigint("123".to_string()))),
                Element::Expr(Expr::DataLiteral(DataLiteral::Number("4.67".to_string()))),
            ]
        ))),
        ("{x: y, t    : [p], ...o, short}", Expr::Record(Record( 
            vec![
                PropDef::KeyValue(PropName::Ident("x".to_string()), Expr::Variable("y".to_string())),
                PropDef::KeyValue(PropName::Ident("t".to_string()), Expr::Array(Array (vec![Element::Expr(Expr::Variable("p".to_string()))]))),
                PropDef::Spread(Expr::Variable("o".to_string())),
                PropDef::Shorthand(PropName::Ident("short".to_string())),
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
                    Expr::new_sub(Expr::Variable("x".to_string()), Expr::new_number(7))
                )))))),
            Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::StrictEqual, 
                Expr::DataLiteral(DataLiteral::True), 
                Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mod, 
                    Expr::Record(Record(vec![PropDef::KeyValue(PropName::Ident("x".to_string()), Expr::Variable("y".to_string()))])), 
                    Expr::new_number(6)
                )))))))))),
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
