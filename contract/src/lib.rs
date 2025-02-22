mod contract;
mod execute;
mod msg;
mod queries;
mod state;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use msg::CourseStatus;

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
pub fn get_attendance(
    deps: Deps,
    course_name: String,
    student_address: String,
) -> StdResult<msg::Student> {
    queries::query_attendance(deps, course_name, student_address)
}

#[entry_point]
pub fn get_students_by_course(deps: Deps, course_name: String) -> StdResult<Binary> {
    queries::query_students_by_course(deps, course_name)
}

#[entry_point]
pub fn add_course(
    deps: DepsMut,
    info: MessageInfo,
    course_name: String,
    stake_amount: Uint128,
) -> StdResult<Response> {
    execute::add_course(deps, info, course_name, stake_amount)
}

#[entry_point]
pub fn update_stake_amount(
    deps: DepsMut,
    info: MessageInfo,
    course_name: String,
    new_stake_amount: Uint128,
) -> StdResult<Response> {
    execute::update_stake_amount(deps, info, course_name, new_stake_amount)
}

#[entry_point]
pub fn update_student_status(
    deps: DepsMut,
    info: MessageInfo,
    course_name: String,
    student_address: String,
    new_status: CourseStatus,
) -> StdResult<Response> {
    execute::update_student_status(deps, info, course_name, student_address, new_status)
}

#[entry_point]
pub fn start_course(deps: DepsMut, info: MessageInfo, course_name: String) -> StdResult<Response> {
    execute::start_course(deps, info, course_name)
}
#[entry_point]
pub fn complete_course(
    deps: DepsMut,
    info: MessageInfo,
    course_name: String,
) -> StdResult<Response> {
    execute::complete_course(deps, info, course_name)
}

#[entry_point]
pub fn claim_stake(deps: DepsMut, info: MessageInfo, course_name: String) -> StdResult<Response> {
    execute::claim_stake(deps, info, course_name)
}
