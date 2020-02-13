use crate::general::{GeneralOptions, CWaypoint};
use std::os::raw::{c_int, c_double, c_char, c_void};
use crate::Status;

#[link(name = "c_osrm")]
extern {
    fn nearest_result_destroy(result: *mut CTableResult);

    fn osrm_nearest(osrm: *mut c_void, request: *mut CTableRequest, result: *mut *mut CTableResult) -> Status;
}

#[repr(C)]
pub enum Annotations
{
    NONE = 0,
    DURATION = 1,
    DISTANCE = 2,
    ALL = 3
}

#[repr(C)]
pub enum FallbackCoordinate
{
    INPUT = 0,
    SNAPPED = 1
}

#[repr(C)]
struct CTableRequest {
    general_options: GeneralOptions,
    sources: *const c_int,
    number_of_sources: i32,
    destinations: *const c_int,
    number_of_destinations: i32,
    annotations: Annotations,
    fallback_speed: c_double,
    fallback_coordinate: FallbackCoordinate,
    scale_factor: c_double
}

#[repr(C)]
struct CTableResult
{
    code: *const c_char,
    message: *const c_char,
    durations: *const *const c_double,
    sources: *const CWaypoint,
    destinations: *const CWaypoint,
    number_of_sources: c_int,
    number_of_destinations: c_int,
}