#![allow(dead_code)]

use crate::Osrm;
use std::ffi::{c_void, CString};
use std::os::raw::c_char;

#[link(name = "c_osrm")]
extern {
    fn nearest_request_create(latitude: f64, longitude: f64) -> *mut CNearestRequest;
    fn nearest_request_destroy(request: *mut CNearestRequest);

    fn nearest_result_destroy(request: *mut CNearestResult);

    fn osrm_nearest(osrm: *mut c_void, request: *mut CNearestRequest, result: *mut *mut CNearestResult) -> Status;
}

#[repr(C)]
#[derive(Debug)]
pub enum Status
{
    Ok = 0,
    Error = 1
}

#[repr(C)]
pub struct Waypoint {
    pub nodes: [u32; 2],
    pub hint: *const c_char,
    pub distance: f64,
    pub name: *const c_char,
    pub location: [f64; 2]
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
    pub code: *const c_char,
    pub message: *const c_char,
    pub waypoints: *mut Waypoint,
    pub number_of_waypoints: i32
}

impl Drop for CNearestResult {
    fn drop(&mut self) {
        unsafe {
            nearest_result_destroy(self);
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

    pub fn run(&mut self, osrm: &Osrm) -> (Status, *mut CNearestResult) {
        unsafe {
            let mut result: *mut CNearestResult = std::ptr::null_mut();
            let result_ptr : *mut *mut CNearestResult = &mut result;
            let mut request = self.convert_to_c_mearest();
            let status = osrm_nearest(*osrm.config, request, result_ptr);
            nearest_request_destroy(request);

            (status, result)
        }
    }

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
