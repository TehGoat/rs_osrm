#![allow(dead_code)]

use crate::{Osrm, Boolean};
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_short, c_longlong};
use std::ptr::null;
use std::borrow::ToOwned;
use core::slice;

#[link(name = "c_osrm")]
extern {
    fn nearest_request_create(latitude: c_double, longitude: c_double) -> *mut CNearestRequest;
    fn nearest_request_destroy(request: *mut CNearestRequest);

    fn nearest_result_destroy(result: *mut CNearestResult);

    fn osrm_nearest(osrm: *mut c_void, request: *mut CNearestRequest, result: *mut *mut CNearestResult) -> Status;
}


#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Status
{
    Ok = 0,
    Error = 1
}

#[repr(C)]
#[derive(Clone)]
pub struct CNearestWaypoint {
    nodes: [c_longlong; 2],
    hint: *const c_char,
    distance: c_double,
    name: *const c_char,
    location: [c_double; 2]
}

pub struct NearestWaypoint {
    pub nodes: [i64; 2],
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2]
}

impl NearestWaypoint {
    pub fn new(c_waypoints: &CNearestWaypoint) -> NearestWaypoint {

        let mut hint: Option<String> = None;
        if c_waypoints.hint != null() {
            let c_hint_buf: *const c_char = c_waypoints.hint;
            let c_hint_str: &CStr = unsafe { CStr::from_ptr(c_hint_buf) };
            let hint_str_slice: &str = c_hint_str.to_str().unwrap();
            hint = Option::from(hint_str_slice.to_owned());
        }

        let c_name_buf: *const c_char = c_waypoints.name;
        let c_name_str: &CStr = unsafe { CStr::from_ptr(c_name_buf) };
        let name_str_slice: &str = c_name_str.to_str().unwrap();

        NearestWaypoint {
            nodes: c_waypoints.nodes,
            hint,
            distance: c_waypoints.distance,
            name: name_str_slice.to_owned(),
            location: c_waypoints.location
        }
    }
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Bearing {
    pub bearing: c_short,
    pub range: c_short
}

#[repr(C)]
#[derive(Clone)]
pub enum Approach {
    UNRESTRICTED = 0,
    CURB = 1,
}

#[repr(C)]
#[derive(Clone)]
pub struct Coordinate{
    latitude: c_double,
    longitude: c_double
}

#[repr(C)]
#[derive(Clone)]
pub struct GeneralOptions{
    coordinate: Coordinate,
    number_of_coordinates: c_int,
    bearings: *mut Bearing,
    radiuses: *mut c_double,
    generate_hints: Boolean,
    hints: *mut c_char,
    approach: *mut Approach,
    exclude: *mut c_char,
    number_of_excludes: c_int
}

#[repr(C)]
pub struct CNearestResult
{
    code: *mut c_char,
    message: *mut c_char,
    waypoints: *mut CNearestWaypoint,
    number_of_waypoints: c_int
}

pub struct NearestResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub way_points: Option<Vec<NearestWaypoint>>
}

impl NearestResult {
    //noinspection RsBorrowChecker
    pub fn new(c_reasult: &CNearestResult, status: &Status) -> NearestResult {

        let mut code: Option<String> = None;
        if c_reasult.code != std::ptr::null_mut() {
            let c_code_buf: *const c_char = c_reasult.code;
            let c_code_str: &CStr = unsafe { CStr::from_ptr(c_code_buf) };
            let code_str_slice: &str = c_code_str.to_str().unwrap();
            code = Option::from(code_str_slice.to_owned());
        }

        let mut message: Option<String> = None;
        if c_reasult.message != std::ptr::null_mut() {
            let c_code_buf: *const c_char = c_reasult.code;
            let c_code_str: &CStr = unsafe { CStr::from_ptr(c_code_buf) };
            let code_str_slice: &str = c_code_str.to_str().unwrap();
            message = Option::from(code_str_slice.to_owned());
        }

        let mut way_points: Option<Vec<NearestWaypoint>> = None;

        if c_reasult.waypoints != std::ptr::null_mut() {
            let test_vec = unsafe {slice::from_raw_parts(c_reasult.waypoints, c_reasult.number_of_waypoints as usize).to_vec()};

            let mut rs_vec = Vec::new();
            for waypoint in &test_vec {
                rs_vec.push(NearestWaypoint::new(waypoint));
            }

            way_points = Option::from(rs_vec);
        }

        NearestResult{
            code,
            message,
            way_points
        }

    }
}

#[repr(C)]
struct CNearestRequest {
    general_options: GeneralOptions,
    number_of_results: u32,
}

impl CNearestRequest {
    fn new(request: &mut NearestRequest) -> CNearestRequest {
        let mut c_request = CNearestRequest {
            number_of_results: request.number_of_results,
            general_options: GeneralOptions {
                generate_hints: Boolean::from(request.generate_hints),
                radiuses: std::ptr::null_mut(),//&mut request.radius as *mut c_double,
                approach: std::ptr::null_mut(),//&mut request.approach,
                exclude: std::ptr::null_mut(),
                hints: std::ptr::null_mut(),
                bearings: std::ptr::null_mut(),
                number_of_coordinates: 1,
                number_of_excludes: 0,
                coordinate: Coordinate {
                    latitude: request.latitude,
                    longitude: request.longitude
                }
            }
        };

        match &mut request.hint {
            Some(hint) => {
                c_request.general_options.hints = hint.as_mut_ptr() as *mut i8
            },
            None => {}
        }
        match &mut request.exclude {
            Some(exclude) => {
                c_request.general_options.exclude = exclude.as_mut_ptr() as *mut i8
            },
            None => {}
        }
        match &mut request.bearing {
            Some(bearing) => {
                c_request.general_options.bearings = bearing
            },
            None => {}
        }

        c_request
    }
}

pub struct NearestRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub number_of_results: u32,
    pub radius: f64,
    pub bearing: Option<Bearing>,
    pub generate_hints: bool,
    pub hint: Option<String>,
    pub approach: Approach,
    pub exclude: Option<String>
}

impl NearestRequest {
    pub fn new(latitude: f64, longitude: f64) -> NearestRequest {
            NearestRequest {
                latitude,
                longitude,
                number_of_results: 1,
                radius: std::f64::MAX,
                bearing: None,
                generate_hints: true,
                hint: None,
                approach: Approach::UNRESTRICTED,
                exclude: None,
            }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, NearestResult) {
        unsafe {
            let mut result: *mut CNearestResult = std::ptr::null_mut();
            let result_ptr : *mut *mut CNearestResult = &mut result;

            let status = osrm_nearest(*osrm.config, &mut CNearestRequest::new(self) as *mut CNearestRequest, result_ptr);

            let converted_result = NearestResult::new(&(*result), &status);

            nearest_result_destroy(result);

            (status, converted_result)
        }
    }

}
