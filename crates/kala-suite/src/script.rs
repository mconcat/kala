use std::{rc::Rc, cell::RefCell};

use jessie_parser::{lexer::lex_jessie, JessieParserState};
use kala_interpreter::eval_script;
use kala_repr::{slot::Slot, completion::Completion, object::Property};

pub(crate) fn run_script(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut builtins_map = FxMap::new();
    builtins_map.insert(SharedString::from_str("console"), Slot::new_object(vec![
        Property::data(SharedString::from_str("log"), Slot::new_native_function(
            SharedString::from_str("log"),
            Rc::new(RefCell::new(|args: &mut [Slot]| {
                for arg in args {
                    print!("{:?} ", arg.to_string())
                }
                println!();
                Completion::Normal
            }))
        ))
    ]));

    let state = JessieParserState::new(tokenstream);
    let script = jessie_parser::script(state, &mut builtins_map).unwrap();

    println!("script: {:?}", script);

    let result = eval_script(script);
    result
}