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

pub fn add_course(
    deps: DepsMut,
    _info: MessageInfo,
    course_name: String,
    stake_amount: Uint128,
) -> StdResult<Response> {
    let course = Course {
        name: course_name.clone(),
        stake_amount,
        status: CourseStatus::NotStarted,
    };
    COURSES.save(deps.storage, course_name, &course)?;
    Ok(Response::new().add_attribute("method", "add_course"))
}

pub fn update_stake_amount(
    deps: DepsMut,
    _info: MessageInfo,
    course_name: String,
    new_stake_amount: Uint128,
) -> StdResult<Response> {
    COURSES.update(deps.storage, course_name, |course| {
        let mut course = course.ok_or_else(|| StdError::generic_err("Course not found"))?;
        course.stake_amount = new_stake_amount;
        Ok(course)
    })?;
    Ok(Response::new().add_attribute("method", "update_stake_amount"))
}

pub fn update_student_status(
    deps: DepsMut,
    _info: MessageInfo,
    course_name: String,
    student_address: String,
    new_status: CourseStatus,
) -> StdResult<Response> {
    let student_key = (&course_name, &student_address);
    STUDENTS.update(deps.storage, student_key, |student| {
        let mut student = student.ok_or_else(|| StdError::generic_err("Student not found"))?;
        student.status = new_status;
        Ok(student)
    })?;
    Ok(Response::new().add_attribute("method", "update_student_status"))
}

pub fn start_course(deps: DepsMut, _info: MessageInfo, course_name: String) -> StdResult<Response> {
    COURSES.update(deps.storage, &course_name, |course| {
        let mut course = course.ok_or_else(|| StdError::generic_err("Course not found"))?;
        course.status = CourseStatus::Started;
        Ok(course)
    })?;
    Ok(Response::new().add_attribute("method", "start_course"))
}

pub fn complete_course(
    deps: DepsMut,
    _info: MessageInfo,
    course_name: String,
) -> StdResult<Response> {
    COURSES.update(deps.storage, &course_name, |course| {
        let mut course = course.ok_or_else(|| StdError::generic_err("Course not found"))?;
        course.status = CourseStatus::Completed;
        Ok(course)
    })?;
    Ok(Response::new().add_attribute("method", "complete_course"))
}

pub fn claim_stake(deps: DepsMut, info: MessageInfo, course_name: String) -> StdResult<Response> {
    let course = COURSES.load(deps.storage, &course_name)?;
    if course.status != CourseStatus::Completed {
        return Err(StdError::generic_err("Course is not yet completed"));
    }

    let student_key = (&course_name, &info.sender.to_string());
    let student = STUDENTS.may_load(deps.storage, student_key.clone())?;
    if student.is_none() {
        return Err(StdError::generic_err("Student not found"));
    }

    let stake_amount = course.stake_amount;
    STUDENTS.remove(deps.storage, student_key);

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: "uatom".to_string(),
            amount: stake_amount,
        }],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "claim_stake"))
}
