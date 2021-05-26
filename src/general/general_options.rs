use std::{ffi::CString, os::raw::{c_char, c_double, c_int}};

use crate::Boolean;

use super::{Approach, Bearing, COsrmCoordinate, Coordinate, to_vec_ccoordinate};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CGeneralOptions {
    pub(crate) coordinate: *const COsrmCoordinate,
    pub(crate) number_of_coordinates: c_int,
    pub(crate) bearings: *const *const Bearing,
    pub(crate) radiuses: *const *const c_double,
    pub(crate) generate_hints: Boolean,
    pub(crate) skip_waypoints: Boolean,
    pub(crate) hints: *const *const c_char,
    pub(crate) approach: *const *const Approach,
    pub(crate) exclude: *const *const c_char,
    pub(crate) number_of_excludes: c_int,
}

impl From<&mut GeneralOptions> for CGeneralOptions {
    fn from(option: &mut GeneralOptions) -> Self {
        option.c_coordinate = to_vec_ccoordinate(&option.coordinate);
        let mut general_c_option = CGeneralOptions {
            coordinate: option.c_coordinate.as_ptr(),
            number_of_coordinates: option.coordinate.len() as c_int,
            bearings: std::ptr::null(),
            radiuses: std::ptr::null(),
            generate_hints: Boolean::from(option.generate_hints),
            skip_waypoints: Boolean::from(option.skip_waypoints),
            hints: std::ptr::null(),
            approach: std::ptr::null(),
            exclude: std::ptr::null(),
            number_of_excludes: 0,
        };

        if option.bearings.is_some() {
            option.bearings_t.clear();

            for bearing in option.bearings.as_ref().unwrap() {
                match bearing {
                    Some(it) => {
                        option.bearings_t.push(it);
                    }
                    None => {
                        option.bearings_t.push(std::ptr::null());
                    }
                }
            }
            general_c_option.bearings = option.bearings_t.as_ptr();
        }

        if option.radiuses.is_some() {
            option.radiuses_t.clear();

            for radiuse in option.radiuses.as_ref().unwrap() {
                match radiuse {
                    Some(it) => {
                        option.radiuses_t.push(it);
                    }
                    None => {
                        option.radiuses_t.push(std::ptr::null());
                    }
                }
            }
            general_c_option.radiuses = option.radiuses_t.as_ptr();
        }

        if option.hints.is_some() {
            let mut c_hint_vec = Vec::new();
            for hint in option.hints.as_ref().unwrap() {
                c_hint_vec.push(CString::new(hint.clone()).unwrap());
            }
            option.c_hints = Option::from(c_hint_vec);
            general_c_option.hints =
                option.c_hints.as_ref().unwrap().as_ptr() as *const *const c_char;
        }

        if option.approach.is_some() {
            option.approach_t.clear();

            for approach in option.approach.as_ref().unwrap() {
                match approach {
                    Some(it) => {
                        option.approach_t.push(it);
                    }
                    None => {
                        option.approach_t.push(std::ptr::null());
                    }
                }
            }

            general_c_option.approach = option.approach_t.as_ptr();
        }

        if option.exclude.is_some() {
            let mut c_exclude_vec = Vec::new();
            for exclude in option.exclude.as_ref().unwrap() {
                c_exclude_vec.push(CString::new(exclude.clone()).unwrap());
            }
            option.c_exclude = Option::from(c_exclude_vec);
            general_c_option.exclude =
                option.c_exclude.as_ref().unwrap().as_ptr() as *const *const c_char;
            general_c_option.number_of_excludes = option.exclude.as_ref().unwrap().len() as c_int;
        }

        general_c_option
    }
}

#[derive(Clone)]
pub struct GeneralOptions {
    coordinate: Vec<Coordinate>,
    c_coordinate: Vec<COsrmCoordinate>,
    bearings: Option<Vec<Option<Bearing>>>,
    bearings_t: Vec<*const Bearing>,
    radiuses: Option<Vec<Option<f64>>>,
    radiuses_t: Vec<*const f64>,
    generate_hints: bool,
    skip_waypoints: bool,
    hints: Option<Vec<String>>,
    c_hints: Option<Vec<CString>>,
    approach: Option<Vec<Option<Approach>>>,
    approach_t: Vec<*const Approach>,
    exclude: Option<Vec<String>>,
    c_exclude: Option<Vec<CString>>,
}

impl GeneralOptions {
    pub fn new(coordinates: &Vec<Coordinate>) -> GeneralOptions {
        GeneralOptions {
            coordinate: coordinates.clone(),
            c_coordinate: to_vec_ccoordinate(&coordinates),
            bearings: None,
            bearings_t: vec![],
            radiuses: None,
            radiuses_t: vec![],
            generate_hints: true,
            skip_waypoints: false,
            hints: None,
            c_hints: None,
            approach: None,
            approach_t: vec![],
            exclude: None,
            c_exclude: None,
        }
    }
}




