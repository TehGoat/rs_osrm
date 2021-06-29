use std::{ffi::c_void, os::raw::{c_char, c_double, c_int}};

use crate::{Status, general::c_string_to_string};

use self::{trip_request::CTripRequest, trip_result::CTripResult};

pub mod trip_request;
pub mod trip_result;
pub mod trip_request_builder;

#[link(name = "c_osrm")]
extern "C" {
    fn trip_result_destroy(result: *mut CTripResult);

    fn osrm_trip(
        osrm: *mut c_void,
        request: *mut CTripRequest,
        result: *mut *mut CTripResult,
    ) -> Status;
}

#[repr(C)]
#[derive(Clone)]
pub enum trip_start {
    StartAny,
    First,
}

#[repr(C)]
#[derive(Clone)]
pub enum trip_end {
    EndAny,
    Last,
}

#[repr(C)]
#[derive(Clone)]
struct CTripWaypoint {
    hint: *const c_char,
    distance: c_double,
    name: *const c_char,
    location: [c_double; 2],
    trips_index: c_int,
    waypoint_index: c_int,
}

pub struct TripWaypoint {
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
    pub trips_index: i32,
    pub waypoint_index: i32,
}

impl TripWaypoint {
    fn new(c_waypoints: &CTripWaypoint) -> TripWaypoint {
        let mut hint: Option<String> = None;
        if c_waypoints.hint != std::ptr::null() {
            hint = Option::from(c_string_to_string(c_waypoints.hint));
        }

        TripWaypoint {
            hint,
            distance: c_waypoints.distance,
            name: c_string_to_string(c_waypoints.name),
            location: c_waypoints.location,
            trips_index: c_waypoints.trips_index,
            waypoint_index: c_waypoints.waypoint_index,
        }
    }
}