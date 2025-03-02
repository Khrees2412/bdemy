use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryInputMsg {
    Greet {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExampleInstantiateMsg {
    pub admins: Vec<String>,
}

// Courses Messages

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub course_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Student {
    pub name: String,
    pub attended_classes: u32,
    pub status: CourseStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CourseStatus {
    NotStarted,
    Started,
    Completed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Course {
    pub name: String,
    pub stake_amount: Uint128,
    pub status: CourseStatus,
}
