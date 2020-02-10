#![allow(dead_code)]

use crate::Osrm;
use std::ffi::{c_void, CString, CStr};
use std::os::raw::c_char;
use std::ptr::null;
use core::slice;

#[link(name = "c_osrm")]
extern {
    fn nearest_request_create(latitude: f64, longitude: f64) -> *mut CNearestRequest;
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
pub struct CWaypoint {
    nodes: [u32; 2],
    hint: *const c_char,
    distance: f64,
    name: *const c_char,
    location: [f64; 2]
}

pub struct Waypoint {
    pub nodes: [u32; 2],
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2]
}

impl Waypoint {
    pub fn new(c_waypoints: &CWaypoint) -> Waypoint {

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

        Waypoint {
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
    pub bearing: i8,
    pub range: i8
}

#[repr(C)]
#[derive(Clone)]
pub enum Approach {
    UNRESTRICTED = 0,
    CURB = 1,
}

#[repr(C)]
pub struct CNearestResult
{
    code: *const c_char,
    message: *const c_char,
    waypoints: *const CWaypoint,
    number_of_waypoints: i32
}

pub struct NearestResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub way_points: Option<Vec<Waypoint>>
}

impl NearestResult {
    //noinspection RsBorrowChecker
    pub fn new(c_reasult: &CNearestResult, status: &Status) -> NearestResult {

        let mut code: Option<String> = None;
        if c_reasult.code != null() {
            let c_code_buf: *const c_char = c_reasult.code;
            let c_code_str: &CStr = unsafe { CStr::from_ptr(c_code_buf) };
            let code_str_slice: &str = c_code_str.to_str().unwrap();
            code = Option::from(code_str_slice.to_owned());
        }

        let mut message: Option<String> = None;
        if c_reasult.message != null() {
            let c_code_buf: *const c_char = c_reasult.code;
            let c_code_str: &CStr = unsafe { CStr::from_ptr(c_code_buf) };
            let code_str_slice: &str = c_code_str.to_str().unwrap();
            message = Option::from(code_str_slice.to_owned());
        }

        let mut way_points: Option<Vec<Waypoint>> = None;
        if status == &Status::Ok && c_reasult.waypoints != null() {
            println!("Not null");
            println!("{}", (*c_reasult).number_of_waypoints);
            let array = unsafe { slice::from_raw_parts((*c_reasult).waypoints, (*c_reasult).number_of_waypoints as usize) };

            let mut waypoint_vec = Vec::with_capacity(array.len());

            for waypoint in array {
                waypoint_vec.push(Waypoint::new(waypoint));
            }

            way_points = Option::from(waypoint_vec);
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
    latitude: f64,
    longitude: f64,
    number_of_results: u32,
    radius: f64,
    bearing: *mut Bearing,
    generate_hints: c_char,
    hint: *const c_char,
    approach: Approach,
    exclude: *const c_char
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
                radius: -1.0,
                bearing: None,
                generate_hints: true,
                hint: None,
                approach: Approach::UNRESTRICTED,
                exclude: None
            }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, NearestResult) {
        unsafe {
            let mut result: *mut CNearestResult = std::ptr::null_mut();
            let result_ptr : *mut *mut CNearestResult = &mut result;
            let request = self.convert_to_c_mearest();
            let status = osrm_nearest(*osrm.config, request, result_ptr);
            nearest_request_destroy(request);

            let converted_result = NearestResult::new(&(*result), &status);

            nearest_result_destroy(result);

            (status, converted_result)
        }
    }

    //noinspection RsBorrowChecker
    fn convert_to_c_mearest(&mut self) -> *mut CNearestRequest {
        unsafe {
            let mut crequest = nearest_request_create(self.latitude, self.longitude);
            (*crequest).number_of_results = self.number_of_results;
            (*crequest).radius = self.radius;

            if self.bearing.is_some() {
                let mut bearing_box = Box::new(self.bearing.unwrap());
                (*crequest).bearing = &mut *bearing_box;
            }

            if !self.generate_hints {
                (*crequest).generate_hints = 0;
            }

            if self.hint.is_some() {
                (*crequest).hint = CString::new(self.hint.as_ref().unwrap().as_str()).unwrap().as_ptr();
            }

            (*crequest).approach = self.approach.clone();

            if self.exclude.is_some() {
                (*crequest).exclude = CString::new(self.hint.as_ref().unwrap().as_str()).unwrap().as_ptr();
            }

            crequest
        }
    }
}
