use std::{rc::Rc, cell::RefCell};

use jessie_parser::{lexer::lex_jessie, JessieParserState};
use kala_interpreter::eval_script;
use kala_repr::{slot::Slot, completion::Completion, object::Property};
use utils::Map;

pub(crate) fn run_script(code: String) -> Completion {
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

    println!("script: {:?}", script);

    let mut scope_state = jessie_scope::ScopeState::new(builtins_map);
    jessie_scope::scope_script(&mut scope_state, &mut script);

    let result = eval_script(scope_state.used_builtins(), script);
    result
}