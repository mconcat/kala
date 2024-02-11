#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use jessie_ast::*;
    use jessie_ast::t::*;
    use jessie_parser::{Lexer, JessieParserState};
    use jessie_parser::lexer::lex_jessie;
    use jessie_parser::parser::ParserState;

    use crate::scope_expression;
    use crate::state::ScopeState;    

    #[test]
    fn test_scope() {
        let cases: &mut [(&str, Expr)] = &mut [
        ("function f(){const x=3;const y=4;}",
        _function("f", Some(FunctionScope::new(
            &[],
            &[],
            &[_const_var("x", 0), _const_var("y", 1)],
            &[],
        )), &[], &[
            _const(_const_var("x", 0), 3),
            _const(_const_var("y", 1), 4),
        ]),
        ),

        ("function f(){const x=3;x;function g(){const y=4;return x+y;}}",

        {
        let g = Rc::new(RefCell::new(_function_raw("g", Some(FunctionScope::new(
            &[],
            &[_const_var("x", 0)],
            &[_const_var("y", 0)],
            &[],
        )),
        &[], &[
            _const(_const_var("y", 0), 4),
            _return_value(_add(_capture("x", 0), _const_var("y", 0))),
        ]
        )));

        _function("f", Some(FunctionScope::new(
            &[],
            &[],
            &[_const_var("x", 0), _const_var("g", 1)],
            &[(_const_var("g", 1), g.clone())],
        )), &[], &[
            _const(_const_var("x", 0), 3),
            Statement::ExprStatement(Box::new(_const_var("x", 0).into())),
            Statement::LocalDeclaration(Box::new(Declaration::Function(g))) 
        ]) 
        }
        ) 
    ];

        for (i, (code, scoped)) in cases.iter_mut().enumerate() {
            let mut state = ScopeState::new();
            let mut parser_state = JessieParserState::new(lex_jessie(code.to_string()).unwrap());
            let mut ast = jessie_parser::expression(&mut parser_state).unwrap();
            println!("ast: {:?}", ast);
            if let Err(err) = scope_expression(&mut state, &mut ast) {
                panic!("case {}: {}", i, err);
            }
    
            assert_eq!(scoped.clone(), ast.clone(), "case {}", i);
        }       
    }
}