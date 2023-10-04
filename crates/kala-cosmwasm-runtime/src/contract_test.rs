#[cfg(test)]
mod tests {
    use kala_interpreter::interpreter::Evaluation;

    #[test]
    fn test_simple() {
        use kala_repr::slot::Slot;

        use crate::contract::run_expression;
        let expressions = &[
            ("3+3", Evaluation::Value(Slot::new_integer(6))),
            ("3*4", Evaluation::Value(Slot::new_integer(12))),
            //("34n-12n", Slot::new_bigint(-22)),
            
            ("(function() {
                if (true) {
                    return 3;
                } else {
                    return 4;
                }
            })()", Evaluation::Value(Slot::new_integer(3))),
            
        ];
        for (i, (expr, expected)) in expressions.into_iter().enumerate() {
            let result = run_expression(expr.to_string());
            println!("result: {}", result.to_string());
            println!("expected: {}", expected.to_string());

            assert_eq!(expected.to_string(), result.to_string(), "failed on test {}, expectecd {} got {}", i, expected.to_string(), result.to_string());
        }
    }
}