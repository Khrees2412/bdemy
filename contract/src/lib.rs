mod contract;
mod msg;
mod queries;
mod state;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query_attendance(deps: Deps, course_name: String, student_address: String) ->  StdResult<msg::Student> {
    queries::query_attendance(deps, course_name, student_address)
}

#[entry_point]
pub fn query_students_by_course(deps: Deps, course_name: String) -> StdResult<Binary>  {
    queries::query_students_by_course(deps, course_name)
}

