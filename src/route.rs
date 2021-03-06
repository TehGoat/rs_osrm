use crate::general::COsrmRoute;
use crate::general::Coordinate;
use crate::general::Route;
use crate::general::Waypoint;
use crate::general::{CGeneralOptions, CWaypoint, GeneralOptions};
use crate::Osrm;
use crate::{Boolean, Status};
use core::slice;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

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

#[repr(C)]
struct CRouteRequest {
    general_options: CGeneralOptions,
    steps: Boolean,
    alternatives: Boolean,
    number_of_alternatives: u32,
    annotations: Boolean,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
    continue_straight: ContinueStraight,
    waypoints: *const u64,
    number_of_waypoints: i32,
}

impl CRouteRequest {
    fn new(request: &mut RouteRequest) -> CRouteRequest {
        let mut c_request = CRouteRequest {
            general_options: CGeneralOptions::new(&mut request.general_options),
            steps: Boolean::from(request.steps),
            alternatives: Boolean::from(request.alternatives),
            number_of_alternatives: request.number_of_alternatives,
            annotations: Boolean::from(request.annotations),
            annotations_type: request.annotations_type.clone(),
            geometries: request.geometries.clone(),
            overview: request.overview.clone(),
            continue_straight: ContinueStraight::ContinueStraightNone,
            waypoints: std::ptr::null(),
            number_of_waypoints: 0,
        };

        if request.waypoints.is_some() {
            let waypoints = request.waypoints.as_ref().unwrap();
            c_request.waypoints = waypoints.as_ptr();
            c_request.number_of_waypoints = waypoints.len() as c_int;
        }

        if request.continue_straight.is_some() {
            if request.continue_straight.unwrap() {
                c_request.continue_straight = ContinueStraight::ContinueStraightTrue;
            } else {
                c_request.continue_straight = ContinueStraight::ContinueStraightFalse;
            }
        }

        c_request
    }
}

#[repr(C)]
struct CRouteResult {
    code: *const c_char,
    message: *const c_char,
    waypoints: *const CWaypoint,
    number_of_waypoints: c_int,
    routes: *const COsrmRoute,
    number_of_routes: c_int,
}

pub struct RouteResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub waypoints: Vec<Waypoint>,
    pub routes: Vec<Route>,
}

impl RouteResult {
    fn new(c_reasult: &CRouteResult) -> RouteResult {
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

        let mut waypoints: Vec<Waypoint> = Vec::new();
        if c_reasult.waypoints != std::ptr::null_mut() {
            let waypoints_vec = unsafe {
                slice::from_raw_parts(c_reasult.waypoints, c_reasult.number_of_waypoints as usize)
                    .to_vec()
            };

            for waypoint in &waypoints_vec {
                waypoints.push(Waypoint::new(waypoint));
            }
        }

        let mut routes: Vec<Route> = Vec::new();
        if c_reasult.routes != std::ptr::null_mut() {
            let routes_vec = unsafe {
                slice::from_raw_parts(c_reasult.routes, c_reasult.number_of_routes as usize)
                    .to_vec()
            };

            for route in routes_vec {
                routes.push(route.to_route());
            }
        }

        RouteResult {
            code,
            message,
            waypoints,
            routes,
        }
    }
}

pub struct RouteRequest {
    pub general_options: GeneralOptions,
    pub steps: bool,
    pub alternatives: bool,
    pub number_of_alternatives: u32,
    pub annotations: bool,
    pub annotations_type: AnnotationsType,
    pub geometries: GeometriesType,
    pub overview: OverviewType,
    pub continue_straight: Option<bool>,
    pub waypoints: Option<Vec<u64>>,
}

impl RouteRequest {
    pub fn new(coordinates: &Vec<Coordinate>) -> RouteRequest {
        RouteRequest {
            general_options: GeneralOptions::new(coordinates),
            steps: false,
            alternatives: false,
            number_of_alternatives: 0,
            annotations: false,
            annotations_type: AnnotationsType::None,
            geometries: GeometriesType::Polyline,
            overview: OverviewType::Simplified,
            continue_straight: None,
            waypoints: None,
        }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, RouteResult) {
        unsafe {
            let mut result: *mut CRouteResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CRouteResult = &mut result;

            let status = osrm_route(
                *osrm.config,
                &mut CRouteRequest::new(self) as *mut CRouteRequest,
                result_ptr,
            );

            let converted_result = RouteResult::new(&(*result));

            route_result_destroy(result);

            (status, converted_result)
        }
    }
}
