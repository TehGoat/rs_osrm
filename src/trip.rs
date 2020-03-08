use crate::general::c_string_to_string;
use libc::c_double;
use crate::route::OverviewType;
use crate::route::GeometriesType;
use crate::route::AnnotationsType;
use crate::general::Route;
use crate::general::Coordinate;
use crate::general::COsrmRoute;
use crate::Osrm;
use std::ffi::CStr;
use crate::general::{CGeneralOptions, GeneralOptions};
use std::os::raw::{c_int, c_char, c_void};
use crate::{Status, Boolean};
use core::slice;

#[link(name = "c_osrm")]
extern {
    fn trip_result_destroy(result: *mut CTripResult);

    fn osrm_trip(osrm: *mut c_void, request: *mut CTripRequest, result: *mut *mut CTripResult) -> Status;
}

#[repr(C)]
#[derive(Clone)]
pub enum trip_start
{
    StartAny,
    First,
}

#[repr(C)]
#[derive(Clone)]
pub enum trip_end
{
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
    waypoint_index: c_int
}

pub struct TripWaypoint {
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
    pub trips_index: i32,
    pub waypoint_index: i32
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

#[repr(C)]
struct CTripRequest {
    general_options: CGeneralOptions,
    roundtrip: Boolean,
    source: trip_start,
    destination: trip_end,
    steps: Boolean,
    annotations: Boolean,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
}

impl CTripRequest {
    fn new(request: &mut TripRequest) -> CTripRequest {
        CTripRequest{
            general_options: CGeneralOptions::new(&mut request.general_options),
            roundtrip: Boolean::from(request.roundtrip),
            source: request.source.clone(),
            destination: request.destination.clone(),
            steps: Boolean::from(request.steps),
            annotations: Boolean::from(request.annotations),
            annotations_type: request.annotations_type.clone(),
            geometries: request.geometries.clone(),
            overview: request.overview.clone()
        }
    }
}

pub struct TripRequest {
    general_options: GeneralOptions,
    roundtrip: bool,
    source: trip_start,
    destination: trip_end,
    steps: bool, 
    annotations: bool,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType
}

impl TripRequest {
    pub fn new(coordinates: &Vec<Coordinate>) -> TripRequest{
        TripRequest{
            general_options: GeneralOptions::new(coordinates),
            roundtrip: true,
            source: trip_start::StartAny,
            destination: trip_end::EndAny,
            steps: false,
            annotations: false,
            annotations_type: AnnotationsType::None,
            geometries: GeometriesType::Polyline,
            overview: OverviewType::Simplified
        }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, TripResult) {
        unsafe {
            let mut result: *mut CTripResult = std::ptr::null_mut();
            let result_ptr : *mut *mut CTripResult = &mut result;

            let status = osrm_trip(*osrm.config,
                                    &mut CTripRequest::new(self) as *mut CTripRequest,
                                    result_ptr);

            let converted_result = TripResult::new(&(*result));

            trip_result_destroy(result);

            (status, converted_result)

        }
    }
}

#[repr(C)]
struct CTripResult
{
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
    pub trips: Vec<Route>
}

impl TripResult {
    fn new(c_reasult: &CTripResult) -> TripResult {

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
                slice::from_raw_parts(c_reasult.waypoints, c_reasult.number_of_waypoints as usize).to_vec()
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

            for route in routes_vec {
                trips.push(route.to_route());
            }
        }

        TripResult{
            code,
            message,
            waypoints,
            trips
        }

    }
}

