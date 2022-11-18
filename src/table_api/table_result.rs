use std::{ffi::CStr, os::raw::{c_char, c_int}, slice};

use crate::general::{c_structs::c_waypoint::CWaypoint, rs_structs::waypoint::Waypoint};

#[repr(C)]
pub(crate) struct CTableResult {
    code: *const c_char,
    message: *const c_char,
    durations: *const f64,
    distances: *const f64,
    sources: *const CWaypoint,
    destinations: *const CWaypoint,
    number_of_sources: c_int,
    number_of_destinations: c_int,
}

pub struct TableResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub durations: Option<Vec<Vec<f64>>>,
    pub distances: Option<Vec<Vec<f64>>>,
    pub sources: Option<Vec<Waypoint>>,
    pub destinations: Option<Vec<Waypoint>>,
}

impl TableResult {
    pub(crate) fn new(c_reasult: &CTableResult) -> TableResult {
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
                    rs_tmp_vec.push(durations_vec[(i * c_reasult.number_of_destinations + j) as usize]);
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
                    rs_tmp_vec.push(distances_vec[(i * c_reasult.number_of_destinations + j) as usize]);
                }
                rs_vec.push(rs_tmp_vec);
            }

            distances = Option::from(rs_vec);
        }

        let sources: Option<Vec<Waypoint>> = if c_reasult.sources != std::ptr::null_mut() {
            Some(
                unsafe {
                    slice::from_raw_parts(c_reasult.sources, c_reasult.number_of_sources as usize)
                        .to_vec()
                }
                .iter()
                .map(|source| source.into())
                .collect(),
            )
        } else {
            None
        };

        let destinations: Option<Vec<Waypoint>> =
            if c_reasult.destinations != std::ptr::null_mut() {
                Some(
                    unsafe {
                        slice::from_raw_parts(
                            c_reasult.destinations,
                            c_reasult.number_of_destinations as usize,
                        )
                        .to_vec()
                    }
                    .iter()
                    .map(|destination| destination.into())
                    .collect(),
                )
            } else {
                None
            };

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