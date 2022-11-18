use std::os::raw::{c_char, c_double, c_int};

use crate::general::{c_string_to_option_string, c_string_to_string};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CMatchWaypoint {
    hint: *const c_char,
    distance: c_double,
    name: *const c_char,
    location: [c_double; 2],
    matchings_index: c_int,
    waypoint_index: c_int,
    alternatives_count: c_int,
}

#[derive(Debug)]
pub struct MatchWaypoint {
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
    pub matchings_index: i32,
    pub waypoint_index: i32,
    pub alternatives_count: i32,
}

impl From<&CMatchWaypoint> for MatchWaypoint {
    fn from(c_match: &CMatchWaypoint) -> Self {
        MatchWaypoint {
            hint: c_string_to_option_string(c_match.hint),
            distance: c_match.distance,
            name: c_string_to_string(c_match.name),
            location: c_match.location,
            matchings_index: c_match.matchings_index,
            waypoint_index: c_match.waypoint_index,
            alternatives_count: c_match.alternatives_count,
        }
    }
}
