use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    slice,
};

use super::{
    match_route::{CMatchRoute, MatchRoute},
    match_waypoint::{CMatchWaypoint, MatchWaypoint},
};

#[repr(C)]
pub(crate) struct CMatchResult {
    code: *const c_char,
    message: *const c_char,
    waypoints: *const CMatchWaypoint,
    number_of_waypoints: c_int,
    routes: *const CMatchRoute,
    number_of_routes: c_int,
}

#[derive(Debug)]
pub struct MatchResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub tracepoints: Vec<MatchWaypoint>,
    pub matchings: Vec<MatchRoute>,
}

impl From<&CMatchResult> for MatchResult {
    fn from(c_reasult: &CMatchResult) -> Self {
        unsafe {
            MatchResult {
                code: if c_reasult.code != std::ptr::null_mut() {
                    CStr::from_ptr(c_reasult.code as *const c_char)
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into()
                } else {
                    None
                },
                message: if c_reasult.message != std::ptr::null_mut() {
                    CStr::from_ptr(c_reasult.message as *const c_char)
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into()
                } else {
                    None
                },
                tracepoints: if c_reasult.waypoints != std::ptr::null_mut() {
                    slice::from_raw_parts(
                        c_reasult.waypoints,
                        c_reasult.number_of_waypoints as usize,
                    )
                    .iter()
                    .map(|waypoint| MatchWaypoint::from(waypoint))
                    .collect()
                } else {
                    Vec::new()
                },
                matchings: if c_reasult.routes != std::ptr::null_mut() {
                    slice::from_raw_parts(
                        c_reasult.routes,
                        c_reasult.number_of_routes as usize,
                    )
                    .iter()
                    .map(|route| MatchRoute::from(route))
                    .collect()
                } else {
                    Vec::new()
                },
            }
        }
    }
}
