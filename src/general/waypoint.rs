use std::{os::raw::{c_char, c_double}, ptr::null};

use super::c_string_to_string;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CWaypoint {
    pub(crate) hint: *const c_char,
    pub(crate) distance: c_double,
    pub(crate) name: *const c_char,
    pub(crate) location: [c_double; 2],
}

pub struct Waypoint {
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
}

impl From<&CWaypoint> for Waypoint {
    fn from(c_waypoint: &CWaypoint) -> Self {
        let mut hint: Option<String> = None;
        if c_waypoint.hint != null() {
            hint = Option::from(c_string_to_string(c_waypoint.hint));
        }

        Waypoint {
            hint,
            distance: c_waypoint.distance,
            name: c_string_to_string(c_waypoint.name),
            location: c_waypoint.location,
        }
    }
}