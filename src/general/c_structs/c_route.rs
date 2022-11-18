use std::{
    os::raw::{c_char, c_double, c_int},
    slice,
};

use crate::general::{c_string_to_option_string, rs_structs::route::Route};

use super::c_route_leg::COsrmRouteLeg;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmRoute {
    pub(crate) duration: c_double,
    pub(crate) distance: c_double,
    pub(crate) weight_name: *const c_char,
    pub(crate) weight: c_double,
    pub(crate) geometry: *const c_char,
    pub(crate) legs: *const COsrmRouteLeg,
    pub(crate) number_of_legs: c_int,
}

impl From<&COsrmRoute> for Route {
    fn from(c_route: &COsrmRoute) -> Self {
        Route {
            duration: c_route.duration,
            distance: c_route.distance,
            weight_name: c_string_to_option_string(c_route.weight_name),
            weight: c_route.weight,
            geometry: c_string_to_option_string(c_route.geometry),
            legs: if c_route.legs != std::ptr::null_mut() {
                unsafe { slice::from_raw_parts(c_route.legs, c_route.number_of_legs as usize).to_vec() }
                    .iter()
                    .map(|leg| leg.into())
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}