use std::{ffi::CStr, os::raw::{c_char, c_int}, slice};

use crate::general::{c_structs::c_route::COsrmRoute, rs_structs::route::Route};

use super::{CTripWaypoint, TripWaypoint};

#[repr(C)]
pub(crate) struct CTripResult {
    code: *const c_char,
    message: *const c_char,
    waypoints: *const CTripWaypoint,
    number_of_waypoints: c_int,
    trips: *const COsrmRoute,
    number_of_trips: c_int,
}

pub struct TripResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub waypoints: Vec<TripWaypoint>,
    pub trips: Vec<Route>,
}

impl TripResult {
    pub(crate) fn new(c_reasult: &CTripResult) -> TripResult {
        let mut code: Option<String> = None;
        if c_reasult.code != std::ptr::null_mut() {
            let c_code_buf: *const c_char = c_reasult.code;
            let c_code_str: &CStr = unsafe { CStr::from_ptr(c_code_buf) };
            let code_str_slice: &str = c_code_str.to_str().unwrap();
            code = Option::from(code_str_slice.to_owned());
        }

        let mut message: Option<String> = None;
        if c_reasult.message != std::ptr::null_mut() {
            let c_message_buf: *const c_char = c_reasult.message;
            let c_message_str: &CStr = unsafe { CStr::from_ptr(c_message_buf) };
            let message_str_slice: &str = c_message_str.to_str().unwrap();
            message = Option::from(message_str_slice.to_owned());
        }

        let mut waypoints: Vec<TripWaypoint> = Vec::new();
        if c_reasult.waypoints != std::ptr::null_mut() {
            let waypoints_vec = unsafe {
                slice::from_raw_parts(c_reasult.waypoints, c_reasult.number_of_waypoints as usize)
                    .to_vec()
            };

            for waypoint in &waypoints_vec {
                waypoints.push(TripWaypoint::new(waypoint));
            }
        }

        let mut trips: Vec<Route> = Vec::new();
        if c_reasult.trips != std::ptr::null_mut() {
            let routes_vec = unsafe {
                slice::from_raw_parts(c_reasult.trips, c_reasult.number_of_trips as usize).to_vec()
            };

            for route in routes_vec.iter() {
                trips.push(Route::from(route));
            }
        }

        TripResult {
            code,
            message,
            waypoints,
            trips,
        }
    }
}