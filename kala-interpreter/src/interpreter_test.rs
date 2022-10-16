/// Test suites for interpreter implementation
/// 

use crate::eval::*;
use crate::lexical::*;
use crate::value::*;
use crate::context::*;
use kala_ast::ast;



#[cfg(test)]
mod interpreter_test {
    use crate::lexical::{Expression, Statement};
    use crate::literal::{Literal, NumberLiteral, StringLiteral};
    use crate::eval::*;
    use crate::context::InterpreterContext;
    use crate::value::{JSValue, JSNumber, JSString};

    fn test_literal1() -> Expression {
        Expression::literal(Literal::Number(NumberLiteral::SMI(1)))
    }

    fn test_literal2() -> Expression {
        Expression::literal(Literal::String(StringLiteral("hello".to_string())))
    }

    fn eval_script(mut expr: Expression) -> Option<JSValue> {
        let mut context = InterpreterContext::new();
        let mut eval = Eval::new(context);
        eval.expression(&mut expr)
    }

    #[test]
    fn test_expression() {
        let test_cases = vec![
            (test_literal1(), Some(JSValue::Number(JSNumber::SMI(1)))),
            (test_literal2(), Some(JSValue::String(JSString("hello".to_string())))),
        ];
        
        for (expr, expected) in test_cases {
            let result = eval_script(expr);
            assert_eq!(result, expected);
        }
    }
}