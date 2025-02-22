use crate::{msg::*, state::*};
use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult, Storage};

pub fn query_attendance(
    deps: Deps,
    course_name: String,
    student_address: String,
) -> StdResult<Student> {
    let student = STUDENTS.load(deps.storage, (course_name, &student_address))?;
    Ok(student)
}

pub fn query_students_by_course(deps: Deps, course_name: String) -> StdResult<Binary> {
    let students: Vec<Student> = STUDENTS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|res| res.ok())
        .filter(|((course, _), _)| course == &course_name)
        .map(|(_, student)| student)
        .collect();
    to_json_binary(&students)
}
