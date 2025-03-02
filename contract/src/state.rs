use crate::msg::*;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const COURSES: Map<String, Course> = Map::new("courses");
pub const STUDENTS: Map<(String, String), Student> = Map::new("students");
