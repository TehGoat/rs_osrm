use std::{
    os::raw::{c_char, c_double, c_float, c_int},
    slice,
};

use crate::general::{
    c_string_to_string, c_structs::c_route_leg::COsrmRouteLeg, rs_structs::route_leg::RouteLeg,
};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CMatchRoute {
    pub(crate) duration: c_double,
    pub(crate) distance: c_double,
    pub(crate) weight_name: *const c_char,
    pub(crate) weight: c_double,
    pub(crate) geometry: *const c_char,
    pub(crate) legs: *const COsrmRouteLeg,
    pub(crate) number_of_legs: c_int,
    pub(crate) confidence: c_float,
}

#[derive(Debug)]
pub struct MatchRoute {
    pub duration: f64,
    pub distance: f64,
    pub weight_name: Option<String>,
    pub weight: f64,
    pub geometry: Option<String>,
    pub legs: Vec<RouteLeg>,
    pub number_of_legs: i32,
    pub confidence: f32,
}

impl From<&CMatchRoute> for MatchRoute {
    fn from(c_route: &CMatchRoute) -> Self {
        MatchRoute {
            duration: c_route.duration,
            distance: c_route.distance,
            weight_name: if c_route.weight_name != std::ptr::null() {
                c_string_to_string(c_route.weight_name).into()
            } else {
                None
            },
            weight: c_route.weight,
            geometry: if c_route.geometry != std::ptr::null() {
                c_string_to_string(c_route.geometry).into()
            } else {
                None
            },
            legs: if c_route.legs != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(c_route.legs, c_route.number_of_legs as usize)
                        .iter()
                        .map(|leg| leg.into())
                        .collect()
                }
            } else {
                Vec::new()
            },
            number_of_legs: if c_route.number_of_legs < 0 {
                0
            } else {
                c_route.number_of_legs as i32
            },
            confidence: c_route.confidence,
        }
    }
}
