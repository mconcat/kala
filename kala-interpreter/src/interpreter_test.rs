/// Test suites for interpreter implementation
/// 

use crate::eval::*;
use crate::lexical::*;
use crate::value::*;
use crate::context::*;
use kala_ast::ast;



#[cfg(test)]
mod interpreter_test {
    use kala_ast::ast::{self, ObjectElement};
    use kala_ast::parse;

    use crate::lexical::{Expression, Statement};
    use crate::literal::{Literal, NumberLiteral, StringLiteral};
    use crate::eval::*;
    use crate::context::InterpreterContext;
    use crate::value::{JSValue, JSNumber, JSString, JSBoolean};
    use crate::interpreter_test::InterpreterF;

    fn parse_expr(source: &str) -> Expression {
        parse::parse_expr::<InterpreterF>(source)
    } 

    fn test_literal_01() -> (Expression, JSValue) {
        let expr = parse_expr("1");
        let val = JSValue::Number(JSNumber::SMI(1));
        (expr, val)
    }

    fn test_literal_02() -> (Expression, JSValue) {
        let expr = parse_expr("'hello'");
        let val = JSValue::String(JSString("hello".to_string()));
        (expr, val)
    }

    fn test_array_01() -> (Expression, JSValue) {
        let expr = parse_expr("[1, 2, 3][0]");

        let val = JSValue::Number(JSNumber::SMI(1));

        (expr, val)
    }
    
    fn test_object_01() -> (Expression, JSValue) {
        let expr = parse_expr("({a: 1, b: 2}).a");

        let val = JSValue::Number(JSNumber::SMI(1));

        (expr, val)
    }

    fn test_binary_01() -> (Expression, JSValue) {
        let expr = parse_expr("1+1");
        let val = JSValue::Number(JSNumber::SMI(2));

        (expr, val)
    }

    fn test_unary_01() -> (Expression, JSValue) {
        let expr = parse_expr("!true");
        let val = JSValue::Boolean(JSBoolean(false));

        (expr, val)
    }

    fn test_logical_01() -> (Expression, JSValue) {
        let expr = parse_expr("true && false");
        let val = JSValue::Boolean(JSBoolean(false));

        (expr, val)
    }

    fn test_function_01() -> (Expression, JSValue) {
        let expr = parse_expr("(function() { return 1; })()");

        let val = JSValue::Number(JSNumber::SMI(1));

        (expr, val)
    }
    
    fn eval_script(mut expr: Expression) -> JSValue {
        let mut context = InterpreterContext::new();
        let mut eval = Eval::new(context);
        eval.expression(&mut expr).unwrap()
    }

    #[test]
    fn test_expression() {
        let test_cases = vec![
            test_literal_01(),
            test_literal_02(),
            test_array_01(),
            test_object_01(),
            test_binary_01(),
            test_unary_01(),
            test_logical_01(),
            test_function_01(),
        ];
        
        for (expr, value) in test_cases {
            let result = eval_script(expr);
            assert_eq!(result, value);
        }
    }
}