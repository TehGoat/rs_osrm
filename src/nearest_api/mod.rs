use std::ffi::c_void;

use crate::Status;

use self::{nearest_request::CNearestRequest, nearest_result::CNearestResult};

pub mod nearest_request;
pub mod nearest_result;
pub mod nearest_waypoint;
pub mod nearest_request_builder;

#[link(name = "c_osrm")]
extern "C" {
    pub(crate) fn nearest_result_destroy(result: *mut CNearestResult);

    pub(crate) fn osrm_nearest(
        osrm: *mut c_void,
        request: *mut CNearestRequest,
        result: *mut *mut CNearestResult,
    ) -> Status;
}