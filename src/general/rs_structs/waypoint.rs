use std::{ptr::null};

use crate::general::{c_string_to_string, c_structs::c_waypoint::CWaypoint};

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