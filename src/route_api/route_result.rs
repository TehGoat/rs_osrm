use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    slice,
};

use crate::general::{
    c_structs::{c_route::COsrmRoute, c_waypoint::CWaypoint},
    rs_structs::{route::Route, waypoint::Waypoint},
};

#[repr(C)]
pub(crate) struct CRouteResult {
    code: *const c_char,
    message: *const c_char,
    waypoints: *const CWaypoint,
    number_of_waypoints: c_int,
    routes: *const COsrmRoute,
    number_of_routes: c_int,
}

#[derive(Debug)]
pub struct RouteResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub waypoints: Vec<Waypoint>,
    pub routes: Vec<Route>,
}

impl From<&CRouteResult> for RouteResult {
    fn from(c_reasult: &CRouteResult) -> Self {
        unsafe {
            RouteResult {
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
                waypoints: if c_reasult.waypoints != std::ptr::null_mut() {
                    slice::from_raw_parts(
                        c_reasult.waypoints,
                        c_reasult.number_of_waypoints as usize,
                    )
                    .iter()
                    .map(|waypoint| waypoint.into())
                    .collect()
                } else {
                    Vec::new()
                },
                routes: if c_reasult.routes != std::ptr::null_mut() {
                    slice::from_raw_parts(
                        c_reasult.routes,
                        c_reasult.number_of_routes as usize,
                    )
                    .iter()
                    .map(|route| route.into())
                    .collect()
                } else {
                    Vec::new()
                },
            }
        }
    }
}