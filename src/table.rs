use crate::general::Coordinate;
use crate::general::Waypoint;
use crate::general::{CGeneralOptions, CWaypoint, GeneralOptions};
use crate::{Osrm, Status};
use core::slice;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_void};

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

#[repr(C)]
struct CTableRequest {
    general_options: CGeneralOptions,
    sources: *const c_int,
    number_of_sources: c_int,
    destinations: *const c_int,
    number_of_destinations: c_int,
    annotations: Annotations,
    fallback_speed: c_double,
    fallback_coordinate: FallbackCoordinate,
    scale_factor: c_double,
}

impl CTableRequest {
    fn new(request: &mut TableRequest) -> CTableRequest {
        let mut c_request = CTableRequest {
            general_options: CGeneralOptions::new(&mut request.general_options),
            sources: std::ptr::null(),
            number_of_sources: 0,
            destinations: std::ptr::null(),
            number_of_destinations: 0,
            annotations: request.annotations.clone(),
            fallback_speed: request.fallback_speed,
            fallback_coordinate: request.fallback_coordinate.clone(),
            scale_factor: request.scale_factor,
        };

        if request.sources.is_some() {
            let sources = request.sources.as_ref().unwrap();
            c_request.sources = sources.as_ptr();
            c_request.number_of_sources = sources.len() as c_int;
        }

        if request.destinations.is_some() {
            let destinations = request.destinations.as_ref().unwrap();
            c_request.destinations = destinations.as_ptr();
            c_request.number_of_destinations = destinations.len() as c_int;
        }

        c_request
    }
}

#[repr(C)]
struct CTableResult {
    code: *const c_char,
    message: *const c_char,
    durations: *const f64,
    distances: *const f64,
    sources: *const CWaypoint,
    destinations: *const CWaypoint,
    number_of_sources: c_int,
    number_of_destinations: c_int,
}

#[derive(Debug)]
pub struct TableResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub durations: Option<Vec<Vec<f64>>>,
    pub distances: Option<Vec<Vec<f64>>>,
    pub sources: Option<Vec<Waypoint>>,
    pub destinations: Option<Vec<Waypoint>>,
}

impl TableResult {
    fn new(c_reasult: &CTableResult) -> TableResult {
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

        let mut durations: Option<Vec<Vec<f64>>> = None;
        if c_reasult.durations != std::ptr::null_mut() {
            let durations_vec = unsafe {
                slice::from_raw_parts(
                    c_reasult.durations,
                    (c_reasult.number_of_sources * c_reasult.number_of_destinations) as usize,
                )
            };

            let mut rs_vec = Vec::new();
            for i in 0..c_reasult.number_of_sources {
                let mut rs_tmp_vec = Vec::new();
                for j in 0..c_reasult.number_of_destinations {
                    rs_tmp_vec.push(durations_vec[(i * c_reasult.number_of_sources + j) as usize]);
                }
                rs_vec.push(rs_tmp_vec);
            }

            durations = Option::from(rs_vec);
        }

        let mut distances: Option<Vec<Vec<f64>>> = None;
        if c_reasult.distances != std::ptr::null_mut() {
            let distances_vec = unsafe {
                slice::from_raw_parts(
                    c_reasult.distances,
                    (c_reasult.number_of_sources * c_reasult.number_of_destinations) as usize,
                )
            };

            let mut rs_vec = Vec::new();
            for i in 0..c_reasult.number_of_sources {
                let mut rs_tmp_vec = Vec::new();
                for j in 0..c_reasult.number_of_destinations {
                    rs_tmp_vec.push(distances_vec[(i * c_reasult.number_of_sources + j) as usize]);
                }
                rs_vec.push(rs_tmp_vec);
            }

            distances = Option::from(rs_vec);
        }

        let mut sources: Option<Vec<Waypoint>> = None;
        if c_reasult.sources != std::ptr::null_mut() {
            let sources_vec = unsafe {
                slice::from_raw_parts(c_reasult.sources, c_reasult.number_of_sources as usize)
                    .to_vec()
            };

            let mut rs_vec = Vec::new();
            for source in &sources_vec {
                rs_vec.push(Waypoint::new(source));
            }

            sources = Option::from(rs_vec);
        }

        let mut destinations: Option<Vec<Waypoint>> = None;
        if c_reasult.destinations != std::ptr::null_mut() {
            let destinations_vec = unsafe {
                slice::from_raw_parts(
                    c_reasult.destinations,
                    c_reasult.number_of_destinations as usize,
                )
                .to_vec()
            };

            let mut rs_vec = Vec::new();
            for destination in &destinations_vec {
                rs_vec.push(Waypoint::new(destination));
            }

            destinations = Option::from(rs_vec);
        }

        TableResult {
            code,
            message,
            durations,
            distances,
            sources,
            destinations,
        }
    }
}

pub struct TableRequest {
    general_options: GeneralOptions,
    sources: Option<Vec<i32>>,
    destinations: Option<Vec<i32>>,
    annotations: Annotations,
    fallback_speed: f64,
    fallback_coordinate: FallbackCoordinate,
    scale_factor: f64,
}

impl TableRequest {
    pub fn new(coordinates: &Vec<Coordinate>) -> TableRequest {
        TableRequest {
            general_options: GeneralOptions::new(coordinates),
            sources: None,
            destinations: None,
            annotations: Annotations::DURATION,
            fallback_speed: std::f64::MAX,
            fallback_coordinate: FallbackCoordinate::INPUT,
            scale_factor: 1.0,
        }
    }

    pub fn annotations(&mut self, annotations: Annotations) ->&mut TableRequest{
        self.annotations = annotations;
        self
    }

    pub fn sources(&mut self, sources: Vec<i32>) ->&mut TableRequest{
        self.sources = Some(sources);
        self
    }

    pub fn destinations(&mut self, destinations: Vec<i32>) ->&mut TableRequest{
        self.destinations = Some(destinations);
        self
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, TableResult) {
        unsafe {
            let mut result: *mut CTableResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CTableResult = &mut result;

            let status = osrm_table(
                *osrm.config,
                &mut CTableRequest::new(self) as *mut CTableRequest,
                result_ptr,
            );

            let converted_result = TableResult::new(&(*result));

            table_result_destroy(result);

            (status, converted_result)
        }
    }
}
