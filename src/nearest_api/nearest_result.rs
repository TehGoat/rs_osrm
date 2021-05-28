use core::slice;
use std::borrow::ToOwned;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use super::nearest_waypoint::{CNearestWaypoint, NearestWaypoint};



#[repr(C)]
pub struct CNearestResult {
    code: *mut c_char,
    message: *mut c_char,
    waypoints: *mut CNearestWaypoint,
    number_of_waypoints: c_int,
}

#[derive(Debug)]
pub struct NearestResult {
    pub code: Option<String>,
    pub message: Option<String>,
    pub waypoints: Option<Vec<NearestWaypoint>>,
}

impl NearestResult {
    pub fn new(c_reasult: &CNearestResult) -> NearestResult {
        unsafe {
            NearestResult {
                code: if c_reasult.code != std::ptr::null_mut() {
                    CStr::from_ptr(c_reasult.code as *const c_char)
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into()
                } else {
                    None
                },
                message: if c_reasult.message != std::ptr::null_mut() {
                    CStr::from_ptr(c_reasult.message as *const c_char)
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into()
                } else {
                    None
                },
                waypoints: if c_reasult.waypoints != std::ptr::null_mut() {
                        slice::from_raw_parts(
                            c_reasult.waypoints,
                            c_reasult.number_of_waypoints as usize,
                        )
                        .iter()
                        .map(|waypoint| NearestWaypoint::new(waypoint))
                        .collect::<Vec<NearestWaypoint>>()
                        .into()
                    
                } else {
                    None
                },
            }
        }
    }
}
