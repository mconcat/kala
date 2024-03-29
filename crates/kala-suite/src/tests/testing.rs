use std::{fs, rc::Rc, cell::RefCell};

use jessie_parser::{lexer::lex_jessie, scope, JessieParserState};
use kala_interpreter::{eval_script, statement::eval_statement, interpreter::Interpreter};
use kala_repr::{slot::Slot, completion::Completion, object::Property, function::Frame};
use utils::Map;

use crate::module::inmemory_state;
#[test]
fn test_simple() {
    let code = fs::read_to_string("src/tests/simple.js").unwrap();
    test_cases(code);
}

#[test] 
fn test_capture() {
    let code = fs::read_to_string("src/tests/capture.js").unwrap();
    test_cases(code);
}

#[test]
fn test_arithmetic() {
    let code = fs::read_to_string("src/tests/arithmetic.js").unwrap();
    test_cases(code);
}
#[test]
fn test_state() {
    let code = fs::read_to_string("src/tests/state.js").unwrap();
    state_test_cases(code);
}
 
#[test]
fn test_assign() {
    let code = fs::read_to_string("src/tests/assign.js").unwrap();
    test_cases(code);
}

#[test]
fn test_accessor() {
    let code = fs::read_to_string("src/tests/accessor.js").unwrap();
    test_cases(code);
}

pub fn state_test_cases(code: String) {
    let tokenstream = lex_jessie(code).unwrap();

    let mut builtins_map = Map::default();

    builtins_map.insert("state".into(), inmemory_state());

    let mut state = JessieParserState::new(tokenstream);
    let mut script = jessie_parser::script(&mut state).unwrap();

    let mut scope_state = jessie_scope::ScopeState::new(builtins_map);
    jessie_scope::scope_script(&mut scope_state, &mut script).unwrap();


    println!("script: {:?}", script);
    let mut interpreter = Interpreter::new(scope_state.used_builtins(), Frame::empty());

    for (i, case) in script.statements.statements.chunks(2).enumerate() {
        print!("\n\n\n\n\n");
        println!("case {}: {:?}", i, case[0]);

        let actual = eval_statement(&mut interpreter, &case[0]);

        let expected = eval_statement(&mut interpreter, &case[1]);
        assert_eq!(actual, expected);
    }
}

pub fn test_cases(code: String) {
    let tokenstream = lex_jessie(code).unwrap();

    let mut builtins_map = Map::default();
    builtins_map.insert("console".into(), Slot::new_object(vec![
        Property::data("log", Slot::new_native_function(
            "log",
            Rc::new(RefCell::new(|args: &mut [Slot]| {
                for arg in args {
                    print!("{:?} ", arg.to_string())
                }
                println!();
                Completion::Normal
            }))
        ))
    ]));

    let mut state = JessieParserState::new(tokenstream);
    let mut script = jessie_parser::script(&mut state).unwrap();

    let mut scope_state = jessie_scope::ScopeState::new(builtins_map);
    jessie_scope::scope_script(&mut scope_state, &mut script).unwrap();

    println!("script: {:?}", script);
    let mut interpreter = Interpreter::new(scope_state.used_builtins(), Frame::empty());

    for (i, case) in script.statements.statements.chunks(2).enumerate() {
        print!("\n\n\n\n\n");
        println!("case {}: {:?}", i, case[0]);

        let actual = eval_statement(&mut interpreter, &case[0]);

        let expected = eval_statement(&mut interpreter, &case[1]);
        assert_eq!(actual, expected);
    }
}