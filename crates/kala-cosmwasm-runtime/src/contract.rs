use std::rc::Rc;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use jessie_ast::GlobalDeclarations;
use jessie_parser::JessieParserState;
use jessie_parser::lexer::lex_jessie;
use jessie_parser::parser::ParserState;
use kala_repr::slot::Slot;
use kala_repr::completion::Completion;
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

pub(crate) fn run_expression(code: String) -> Completion {
    let tokenstream = lex_jessie(code).unwrap();

    let mut state = JessieParserState::new(tokenstream, Rc::new(GlobalDeclarations::empty()));
    let expr = jessie_parser::expression(&mut state).unwrap();

    println!("{:?}", expr);
    let mut interpreter = kala_interpreter::interpreter::Interpreter::empty();

    println!("expr: {:?}", expr);
    let result = kala_interpreter::expression::eval_expr(&mut interpreter, &expr);
    result
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        QueryMsg::RunJessie(code) => {
            match run_expression(code) {
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
