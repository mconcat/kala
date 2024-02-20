use std::{rc::Rc, cell::RefCell};

use jessie_parser::{lexer::lex_jessie, JessieParserState};
use kala_repr::{slot::Slot, object::Property, completion::Completion};
use utils::Map;

pub(crate) fn inmemory_state() -> Slot {
    let state: Rc<RefCell<Map<Slot>>> = Rc::new(RefCell::new(Map::default()));

    let get_closure = Rc::new(RefCell::new({
        let state = state.clone();
        move |args: &mut [Slot]| {
            let key: Rc<str> = args[0].to_string().as_str().into();
            let value = state.borrow_mut().get(&key).cloned().unwrap_or(Slot::new_undefined());
            Completion::Return(value)
        }
    }));

    let set_closure = Rc::new(RefCell::new({
        let state = state.clone();
        move |args: &mut [Slot]| {
            let key = args[0].to_string().into();
            let value = args[1].clone();
            let old = state.borrow_mut().insert(key, value).unwrap_or(Slot::new_undefined());
            Completion::Return(old)
        }
    }));

    Slot::new_object(vec![
        Property::data("get", Slot::new_native_function("get", get_closure)),
        Property::data("set", Slot::new_native_function("set", set_closure)),
    ])
}
/* 
pub(crate) fn run_module(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut state = JessieParserState::new(tokenstream);

    let console_log = Slot::new_native_function(
        "log",
        Rc::new(RefCell::new(|args: &mut [Slot]| {
            println!("{:?}", args);
            Completion::Normal
        }))
    );

    let mut builtins_map = Map::from_iter(vec![
        ("console".into(), Slot::new_object(vec![
            Property::data("log", console_log)
        ]))
    ]);

    let module = jessie_parser::module::module(state).unwrap();



    let result = kala_interpreter::module::eval_module(module);
    result
}
*/