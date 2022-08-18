#[path="./gen/nessie.ast.rs"]
pub mod ast;

#[path="./runtime/lexical.rs"]
pub mod lexical;

#[path="./runtime/runtime.rs"]
pub mod runtime;

#[path="./interpreter/lib.rs"]
pub mod interpreter;

/*
#[wasm_bindgen]
pub fn evaluate(code: String) -> String {
    let mut ctx = interpreter::runtime::JSContext::new();
    let parse_result = ast::FunctionExpression::decode(code.as_bytes());
    if parse_result.is_err() {
        return "Error parsing code".to_string();
    }
    let func = parse_result.unwrap();
    let result = lexical::eval_expression(&mut ctx, &ast::Expression{expression: Some(ast::expression::Expression::Function(func))});
    result.to_string()
}
*/

#[cfg(test)]
mod pipeline_tests {
    // js snippets are defined under babel/syntax.test.js
    // TODO: integrate

    use crate::ast;
    use prost::Message;
    use hex;
    use crate::runtime::JSContext;

    const TESTS: &[(&str, &str, &str)] = &[
        (
            "(function f() {
                let a = 1;
                let b = 2;
                return a+b;
            })()", "72590a5772550a5322510a030a01661a4a0a150a130801120f0a0d0a030a016112060a040a0210010a150a130801120f0a0d0a030a016212060a040a0210020a1a62180a163214080112075a050a030a01611a075a050a030a0162", 
            "3",
        ),
        (
            "(function f(a) {
                return a;
            })(1)",
            "722f0a2d722b0a1f221d0a030a016612070a050a030a01611a0d0a0b62090a075a050a030a016112080a060a040a021001",
            "1",
        ),
        (
            "(function f() {
                const o = {
                    a: 1,
                    b: 2,
                };
                o.x = 3;
                return o;
            })()",
            "72770a7572730a71226f0a030a01661a680a370a35080212310a2f0a030a016f12281a260a110a0f0a0512030a016112060a040a0210010a110a0f0a0512030a016212060a040a0210020a20721e0a1c621a1210120e0a075a050a030a016f1a030a01781a060a040a0210030a0b62090a075a050a030a016f",
            "{a:1,b:2,x:3}"
        )
    ];

    #[test]
    fn test_evaluation_cases() {
        for (name, code, expected_result) in TESTS.iter() {
            test_evaluation_case(name, code, expected_result)
        }
    }

    fn test_evaluation_case(name: &str, code: &str, expected_result: &str) {
        let mut ctx = crate::interpreter::runtime::JSContext::new();
        let parse_result = ast::Statement::decode(hex::decode(code).expect("Failed to decode hex").as_slice());
        if parse_result.is_err() {
            panic!("Error parsing code");
        }
        let expr = parse_result.unwrap();
        crate::lexical::eval_statement(&mut ctx, &expr);
        let result = ctx.completion().unwrap().get_return();
        assert_eq!(result.unwrap().to_string(), expected_result);
    }
}