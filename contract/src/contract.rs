use crate::{msg::*, state::*};
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage, Uint128,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let course = Course {
        name: msg.course_name.clone(),
        stake_amount: Uint128::zero(),
        status: CourseStatus::NotStarted,
    };
    COURSES.save(deps.storage, msg.course_name, &course)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}
