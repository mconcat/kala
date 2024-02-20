use std::rc::Rc;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use jessie_parser::JessieParserState;
use jessie_parser::lexer::lex_jessie;
use jessie_parser::parser::ParserState;
use kala_interpreter::eval_script;
use kala_repr::object::Property;
use kala_repr::slot::Slot;
use kala_repr::completion::Completion;
use utils::{FxMap, SharedString, Map};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RunJessieResponse};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:kala-cosmwasm-runtime";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    return Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    return Ok(Response::default())
}

pub(crate) fn run_module(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut state = JessieParserState::new(tokenstream);

    let console_log = Slot::new_native_function(
        SharedString::from_str("log"),
        Box::new(|args| {
            println!("{:?}", args);
            Completion::Normal
        })
    );

    let mut builtins_map = FxMap::from_iter(vec![
        (SharedString::from_str("console"), Slot::new_object(vec![
            Property{key: SharedString::from_str("log"), value: console_log}
        ]))
    ]);

    let module = jessie_parser::module::module(state, &mut builtins_map).unwrap();

    let result = kala_interpreter::module::eval_module(module);
    result
}

pub(crate) fn run_script(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut builtins_map = FxMap::new();
    builtins_map.insert(SharedString::from_str("console"), Slot::new_object(vec![
        Property{key: SharedString::from_str("log"), value: Slot::new_native_function(
            SharedString::from_str("log"),
            Box::new(|args| {
                for arg in args {
                    print!("{:?} ", arg.to_string())
                }
                println!();
                Completion::Normal
            })
        )}
    ]));

    let mut state = JessieParserState::new(tokenstream);
    let script = jessie_parser::script(state, &mut builtins_map).unwrap();

    println!("script: {:?}", script);

    let result = eval_script(script);
    result
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        QueryMsg::RunJessie(code) => {
            match run_script(code) {
                Completion::Value(slot) => {
                    RunJessieResponse {
                        result: slot.to_string(), 
                    }
                },
                Completion::Throw(_) => unimplemented!("Throwing not implemented yet"),
                _ => unimplemented!("Not implemented yet"),
            }
        }
    };

    return Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
