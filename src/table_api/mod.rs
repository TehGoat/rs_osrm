use std::ffi::c_void;

use crate::Status;

use self::{table_request::CTableRequest, table_result::CTableResult};

pub mod table_request;
pub mod table_request_builder;
pub mod table_result;

#[link(name = "c_osrm")]
extern "C" {
    fn table_result_destroy(result: *mut CTableResult);

    fn osrm_table(
        osrm: *mut c_void,
        request: *mut CTableRequest,
        result: *mut *mut CTableResult,
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