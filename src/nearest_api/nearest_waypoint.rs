use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_longlong},
    ptr::null,
};

#[repr(C)]
#[derive(Clone)]
pub struct CNearestWaypoint {
    nodes: [c_longlong; 2],
    hint: *const c_char,
    distance: c_double,
    name: *const c_char,
    location: [c_double; 2],
}

#[derive(Debug)]
pub struct NearestWaypoint {
    pub nodes: [i64; 2],
    pub hint: Option<String>,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
}

impl NearestWaypoint {
    pub fn new(c_waypoints: &CNearestWaypoint) -> NearestWaypoint {
        unsafe {
            NearestWaypoint {
                nodes: c_waypoints.nodes,
                hint: if c_waypoints.hint != null() {
                    CStr::from_ptr(c_waypoints.hint as *const c_char)
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into()
                } else {
                    None
                },
                distance: c_waypoints.distance,
                name: CStr::from_ptr(c_waypoints.name as *const c_char)
                    .to_str()
                    .unwrap()
                    .to_owned(),
                location: c_waypoints.location,
            }
        }
    }
}
