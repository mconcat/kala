#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use jessie_parser::VecToken;
use jessie_parser::lexer::lex_jessie;
use jessie_parser::parser::ParserState;
use kala_interpreter::interpreter::Evaluation;
use kala_repr::slot::Slot;
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

pub(crate) fn run_expression(code: String) -> Evaluation {
    let tokenstream = lex_jessie(code).unwrap();

    let mut state = ParserState::new(VecToken(tokenstream), vec![]);
    let expr = jessie_parser::expression(&mut state).unwrap();

    let mut interpreter = kala_interpreter::interpreter::Interpreter::new();

    println!("expr: {:?}", expr);
    let result = kala_interpreter::expression::eval_expr(&mut interpreter, &expr);
    result

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        QueryMsg::RunJessie(code) => {
            match run_expression(code) {
                Evaluation::Value(slot) => {
                    RunJessieResponse {
                        result: slot.to_string(), 
                    }
                },
                Evaluation::Throw(_) => unimplemented!("Throwing not implemented yet"),
            }
        }
    };

    return Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {}
