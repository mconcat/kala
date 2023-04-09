#[cfg(test)]
mod tests {
    use crate::jessie_types::PropDef;
    use crate::json_parser::*;
    use crate::jessie_types::*;
    use crate::jessie_parser::expr;
    use crate::parser::ParserState;
    
    fn expr_test_cases() -> Vec<(&'static str, Expr)> {
        vec![
        ("undefined", Expr::DataLiteral(DataLiteral::Undefined)),
        ("3", Expr::new_number(3)),
        ("5+6", Expr::new_add(Expr::new_number(5), Expr::new_number(6))),
        ("function f(x) { return x; }", Expr::FunctionExpr(Box::new(Function(
            Some("f".to_string()),
            vec![Pattern::Variable("x".to_string(), None)],
            None,
            Block::new(vec![Statement::Return(Some(Expr::Variable("x".to_string())))]),
        )))),
        ("function f(x, y) {
            return x+y;   
        }", Expr::FunctionExpr(Box::new(Function(
            Some("f".to_string()), 
            vec![Pattern::Variable("x".to_string(), None), Pattern::Variable("y".to_string(), None)], 
            None,
            Block::new(vec![Statement::Return(Some(Expr::new_add(Expr::Variable("x".to_string()), Expr::Variable("y".to_string()))))]))))),
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
            Block::new(vec![
                Statement::Declaration(Declaration { kind: DeclarationKind::Let, bindings: vec![Binding::VariableBinding("t".to_string(), Some(Expr::new_add(Expr::Variable("x".to_string()), Expr::Variable("y".to_string()))))] }),
                Statement::Return(Some(Expr::Variable("z".to_string()))),
            ]))))),
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
         )))
        ]
    }

    #[test]
    fn it_works() {
        expr_test_cases().iter().for_each(|case| {
            println!("===========");
            println!("test for {}", case.0);
            let mut state = ParserState::new(case.0.to_string());
            let result = expr(&mut state);
            assert_eq!(result, Ok(case.1.clone()));
        });
    }
}