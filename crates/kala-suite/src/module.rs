use std::{rc::Rc, cell::RefCell};

use jessie_parser::{lexer::lex_jessie, JessieParserState};
use kala_repr::{slot::Slot, object::Property, completion::Completion};
use utils::{SharedString, FxMap, Map};

pub(crate) fn inmemory_state() -> Slot {
    let state: Rc<RefCell<FxMap<Slot>>> = Rc::new(RefCell::new(FxMap::new()));

    let get_closure = Rc::new(RefCell::new({
        let state = state.clone();
        move |args: &mut [Slot]| {
            let key = args[0].to_string().into();
            let value = state.borrow_mut().get(key).cloned().unwrap_or(Slot::new_undefined());
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
        Property::data("ref", Slot::new_object(vec![
            Property::accessor("value",
                Slot::new_native_function("value", get_closure),
                Slot::new_native_function("value", set_closure),
            )
        ]))
    ])
}

pub(crate) fn run_module(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut state = JessieParserState::new(tokenstream);

    let console_log = Slot::new_native_function(
        SharedString::from_str("log"),
        Rc::new(RefCell::new(|args: &mut [Slot]| {
            println!("{:?}", args);
            Completion::Normal
        }))
    );

    let mut builtins_map = FxMap::from_iter(vec![
        (SharedString::from_str("console"), Slot::new_object(vec![
            Property::data(SharedString::from_str("log"), console_log)
        ]))
    ]);

    let module = jessie_parser::module::module(state, &mut builtins_map).unwrap();

    let result = kala_interpreter::module::eval_module(module);
    result
}