pub mod route_request;
pub mod route_request_builder;
pub mod route_result;


use crate::Status;
use std::os::raw::c_void;

use self::{route_request::CRouteRequest, route_result::CRouteResult};

#[link(name = "c_osrm")]
extern "C" {
    fn route_result_destroy(result: *mut CRouteResult);

    fn osrm_route(
        osrm: *mut c_void,
        request: *mut CRouteRequest,
        result: *mut *mut CRouteResult,
    ) -> Status;
}

#[repr(C)]
#[derive(Clone)]
pub enum GeometriesType {
    Polyline,
    Polyline6,
    GeoJSON,
}

#[repr(C)]
#[derive(Clone)]
pub enum OverviewType {
    Simplified,
    Full,
    False,
}

#[repr(C)]
#[derive(Clone)]
pub enum AnnotationsType {
    None,
    Duration,
    Nodes,
    Distance,
    Weight,
    Datasources,
    Speed,
    All,
}

#[repr(C)]
#[derive(Clone)]
enum ContinueStraight {
    ContinueStraightNone,
    ContinueStraightTrue,
    ContinueStraightFalse,
}

