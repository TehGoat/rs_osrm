pub mod match_request;
pub mod match_result;
pub mod match_route;
pub mod match_waypoint;


use crate::Status;
use core::ffi::c_void;

use self::{match_request::CMatchRequest, match_result::CMatchResult};


#[link(name = "c_osrm")]
extern "C" {
    pub(crate) fn match_result_destroy(result: *mut CMatchResult);

    pub(crate) fn osrm_match(
        osrm: *mut c_void,
        request: *mut CMatchRequest,
        result: *mut *mut CMatchResult,
    ) -> Status;
}

#[repr(C)]
#[derive(Clone)]
pub enum Annotations {
    NONE = 0,
    DURATION = 1,
    DISTANCE = 2,
    ALL = 3,
}

#[repr(C)]
#[derive(Clone)]
pub enum FallbackCoordinate {
    INPUT = 0,
    SNAPPED = 1,
}

#[repr(C)]
#[derive(Clone)]
pub enum Gap {
    Split = 0,
    Ignore = 1,
}