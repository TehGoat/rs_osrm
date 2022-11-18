use std::slice;

use crate::{Boolean, general::{c_string_to_string, c_structs::c_lanes::COsrmLanes}};

#[derive(Debug)]
pub struct Lanes {
    pub indications: Vec<String>,
    pub valid: bool,
}

impl From<&COsrmLanes> for Lanes {
    fn from(c_lanes: &COsrmLanes) -> Self {
        Lanes {
            indications: unsafe {
                slice::from_raw_parts(c_lanes.indications, c_lanes.number_of_indications as usize)
            }
            .iter()
            .map(|indication| c_string_to_string(*indication))
            .collect(),
            valid: c_lanes.valid == Boolean::TRUE,
        }
    }
}