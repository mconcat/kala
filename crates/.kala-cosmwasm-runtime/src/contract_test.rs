#[cfg(test)]
mod tests {
    use kala_repr::completion::Completion;
    
    #[test]
    fn test_simple() {
        use kala_repr::slot::Slot;

        use crate::contract::run_script;
        let expressions = &[
            ("3+4;", Completion::Value(Slot::new_integer(7))),
            //("3*4", Completion::Value(Slot::new_integer(12))),
            //("34n-12n", Slot::new_bigint(-22)),
            
            ("(function() {
                if (true) {
                    return 3;
                } else {
                    return 4;
                }
            })();", Completion::Value(Slot::new_integer(3))),
            ("(function() {
                let x = 3;
                return x;
            })();", Completion::Value(Slot::new_integer(3))),
            ("(function(arg) {
                let x = 3;
                return x+arg;
            })(4);", Completion::Value(Slot::new_integer(7))),
            ("(function(arg1, arg2) {
                let local1 = 3;
                const local2 = 4;
                return local1+local2+arg1+arg2;
            })(5, 6);", Completion::Value(Slot::new_integer(18))),
            ("((function(arg1){
                return function(arg2) {
                    return arg1+arg2;
                };
            })(3))(4);", Completion::Value(Slot::new_integer(7))),
            ("(function(obj) {
                return obj.x+obj.y;
            })({x:3, y:4});", Completion::Value(Slot::new_integer(7))),
            ("(function(){
                function f() {
                    return 3;
                }
                return f();
            })();", Completion::Value(Slot::new_integer(3))),
            ("(function(){
                function f() {
                    return 3;
                }
                return f;
            })()();", Completion::Value(Slot::new_integer(3))),
            ("(function(){
                console.log('console log is working');
            })();", Completion::Normal),
            ("failing test just to check if console.log works", Completion::Normal)
        ];
        for (i, (expr, expected)) in expressions.into_iter().enumerate() {
            print!("\n\n\n\n\n\ntest {}", i);
            let result = run_script(expr.to_string());
            println!("result: {}", result.to_string());
            println!("expected: {}", expected.to_string());

            assert_eq!(expected.to_string(), result.to_string(), "failed on test {}, expectecd {} got {}", i, expected.to_string(), result.to_string());
        }
    }
}