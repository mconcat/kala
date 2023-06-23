#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::expression;
    use crate::parser::ParserState;
    use crate::lexer::*;
    use jessie_ast::*;
    use jessie_ast::helper::*;
    use utils::{OwnedSlice, SharedString};

    fn expr_test_cases() -> Vec<(&'static str, Expr)> {
        vec![
        ("undefined", undefined()),
        ("3", number(3)),
        ("5+6", add(number(5), number(6))),
        ("function f(x) { return x; }", {
            function_expr(
                Some("f"), 
                vec![],
                vec![variable("x")],
                vec![],
                vec![
                    return_statement(variable("x")),
                ],
            ) 
        }),
        ("function f(x, y) {
            return x+y;   
        }", {
            function_expr(
                Some("f"),
                vec![],
                vec![variable("x"), variable("y")],
                vec![],
                vec![
                    return_statement(add(variable("x"), variable("y"))),
                ],
            )
        }),
            // Excluded due to destructing parameter
        ("function f(x, [y, z]) {
            let t = x+y;
            return z;
        }", 
            function_expr(
                Some("f"),
                vec![],
                vec![variable("x"), array(vec![variable("y"), variable("z")])],
                vec![variable("t")],
                vec![
                    let_statement(variable("t"), add(variable("x"), variable("y"))),
                    return_statement(variable("z")),
                ],
            )   
        ),
        ("[3, v, true, {}, ...g, 123n, 4.67]", array(vec![
            number(3),
            variable("v"),
            boolean(true),
            record(vec![]),
            spread(variable("g")),
            bigint(123),
            decimal("4.67"),
        ])),
        ("{x: y, t    : [p], ...o, short}", record(vec![
            keyvalue("x", variable("y")),
            keyvalue("t", array(vec![variable("p")])),
            spread(variable("o")),
            shorthand("short"),
        ])
        ),
        ("(3+2)*1&&undefined/5&x-7||true==={x:y}%6", 
        logical_or(
            logical_and(
                mul(add(number(3), number(2)), number(1)),
                bitand(div(undefined(), number(5)), sub(variable("x"), number(7)))
            ),
            equal(boolean(true), modulo(record(vec![keyvalue("x", variable("y"))]), number(6)))
            )
        ),
        ("x.y.z", 
            property(
                property(
                    variable("x"),
                    "y"
                ),
                "z"
            )
        ),
        ("function f(x) {
            const y = 3;
            {
                let z = 5;
                return x+y+z;
            }
        }", {
            function_expr(
                Some("f"),
                vec![],
                vec![variable("x")],
                vec![variable("y"), variable("z")],
                vec![
                    const_statement(variable("y"), number(3)),
                    block(vec![
                        let_statement(variable("z"), number(5)),
                        return_statement(add(add(variable("x"), variable("y")), variable("z"))),
                    ]),
                ],
            )
        }),
        ("function f(x) {
            return function(y) {
                return x+y;
            };   
        }", {
            function_expr(
                Some("f"),
                vec![],
                vec![variable("x")],
                vec![],
                vec![
                    return_statement(
                        function_expr(
                            None,
                            vec![variable("x")],
                            vec![variable("y")],
                            vec![],
                            vec![
                                return_statement(add(variable("x"), variable("y"))),
                            ],
                        )
                    ),
                ],
            )
        })
        ]
    }

    #[test]
    fn it_works() {
        expr_test_cases().iter().for_each(|case| {
            println!("===========");
            println!("test for {}", case.0);
            let mut lexer_state = ParserState::new(Str(case.0.to_string()), vec![]);
            println!("lexer_state: {:?}", lexer_state);
            let tokenstream = lex(&mut lexer_state).unwrap();
            println!("tokenstream: {:?}", tokenstream);
            let mut state = ParserState::new(VecToken(tokenstream), vec![]);

            let result = expression(&mut state);
            assert_eq!(result, Ok(case.1.clone()));
            println!("success, result: {:?}", result);
        });
    }
}
