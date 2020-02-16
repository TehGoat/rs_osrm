use std::os::raw::{c_short, c_double, c_int, c_char};
use crate::Boolean;
use std::ptr::null;
use std::ffi::CStr;

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
pub  struct Coordinate{
    pub latitude: c_double,
    pub longitude: c_double
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CGeneralOptions {
    pub(crate) coordinate: *const Coordinate,
    pub(crate) number_of_coordinates: c_int,
    pub(crate) bearings: *const Bearing,
    pub(crate) radiuses: *const c_double,
    pub(crate) generate_hints: Boolean,
    pub(crate) hints: *const c_char,
    pub(crate) approach: *const Approach,
    pub(crate) exclude: *const *const c_char,
    pub(crate) number_of_excludes: c_int
}

impl CGeneralOptions {
    pub(crate) fn new(option: &GeneralOptions) -> CGeneralOptions {
        let mut general_c_option = CGeneralOptions {
            coordinate: option.coordinate.as_ptr(),
            number_of_coordinates: option.coordinate.len() as c_int,
            bearings: std::ptr::null(),
            radiuses: std::ptr::null(),
            generate_hints: Boolean::from(option.generate_hints),
            hints: std::ptr::null(),
            approach: std::ptr::null(),
            exclude: std::ptr::null(),
            number_of_excludes: 0
        };

        if option.bearings.is_some() {
            general_c_option.bearings = option.bearings.as_ref().unwrap().as_ptr();
        }

        if option.radiuses.is_some() {
            general_c_option.radiuses = option.radiuses.as_ref().unwrap().as_ptr();
        }

        if option.hints.is_some() {
            general_c_option.hints = option.hints.as_ref().unwrap().as_ptr() as *const c_char;
        }

        if option.approach.is_some() {
            general_c_option.approach = option.approach.as_ref().unwrap().as_ptr();
        }

        if option.exclude.is_some() {
            let exclude = option.exclude.as_ref().unwrap();
            general_c_option.exclude = exclude.as_ptr() as *const *const c_char;
            general_c_option.number_of_excludes = exclude.len() as c_int;
        }

        general_c_option
    }
}

#[derive(Clone)]
pub struct GeneralOptions{
    pub coordinate: Vec<Coordinate>,
    pub bearings: Option<Vec<Bearing>>,
    pub radiuses: Option<Vec<f64>>,
    pub generate_hints: bool,
    pub hints: Option<Vec<String>>,
    pub approach: Option<Vec<Approach>>,
    pub exclude: Option<Vec<String>>
}

impl GeneralOptions {
    pub fn new(coordinates: &Vec<Coordinate>) -> GeneralOptions {
        GeneralOptions {
            coordinate: coordinates.clone(),
            bearings: None,
            radiuses: None,
            generate_hints: true,
            hints: None,
            approach: None,
            exclude: None,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct CWaypoint {
    hint: *const c_char,
    distance: c_double,
    name: *const c_char,
    location: [c_double; 2]
}

pub struct Waypoint {
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
            hint,
            distance: c_waypoints.distance,
            name: name_str_slice.to_owned(),
            location: c_waypoints.location
        }
    }
}